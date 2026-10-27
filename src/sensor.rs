//! Safe, byte-addressed fake sensor memory. No raw pointers or physical MMIO.
pub const STATUS: usize = 0x00;
pub const SAMPLE_ID: usize = 0x02;
pub const ALT_MM: usize = 0x04;
pub const VEL_MM_S: usize = 0x08;
pub const READY: u8 = 0x01;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SensorReading {
    pub sample_id: u16,
    pub altitude_m: f64,
    pub velocity_m_s: f64,
}

#[derive(Default)]
pub struct SensorRegisters {
    bytes: [u8; 12],
}
impl SensorRegisters {
    /// Borrow bytes from a coherent snapshot. Invalid ranges return None.
    pub fn read<const N: usize>(&self, address: usize) -> Option<[u8; N]> {
        self.bytes
            .get(address..address.checked_add(N)?)?
            .try_into()
            .ok()
    }

    /// The simulator publishes all fields together before each firmware update.
    pub(crate) fn publish(&mut self, sample_id: u16, altitude: f64, velocity: f64) {
        self.bytes[STATUS] = READY;
        self.bytes[SAMPLE_ID..SAMPLE_ID + 2].copy_from_slice(&sample_id.to_le_bytes());
        self.bytes[ALT_MM..ALT_MM + 4]
            .copy_from_slice(&((altitude * 1000.0).round() as i32).to_le_bytes());
        self.bytes[VEL_MM_S..VEL_MM_S + 4]
            .copy_from_slice(&((velocity * 1000.0).round() as i32).to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshot_has_little_endian_signed_values() {
        let mut registers = SensorRegisters::default();
        assert_eq!(registers.read::<1>(STATUS), Some([0]));
        registers.publish(513, 2.0, -0.25);
        assert_eq!(registers.read::<1>(STATUS), Some([READY]));
        assert_eq!(registers.read::<2>(SAMPLE_ID), Some([1, 2]));
        assert_eq!(registers.read::<4>(ALT_MM), Some([0xd0, 0x07, 0, 0]));
        assert_eq!(i32::from_le_bytes(registers.read(VEL_MM_S).unwrap()), -250);
    }
    #[test]
    fn invalid_reads_are_safe() {
        let registers = SensorRegisters::default();
        assert!(registers.read::<4>(10).is_none());
        assert!(registers.read::<1>(usize::MAX).is_none());
    }
    #[test]
    fn reference_decoder_obeys_ready_and_units() {
        let mut registers = SensorRegisters::default();
        assert!(crate::reference_controller::read_sensor(&registers).is_none());
        registers.publish(7, 2.123, -0.25);
        assert_eq!(
            crate::reference_controller::read_sensor(&registers),
            Some(SensorReading {
                sample_id: 7,
                altitude_m: 2.123,
                velocity_m_s: -0.25,
            })
        );
    }

    #[test]
    fn reference_controller_stores_fresh_data_and_clears_failed_reads() {
        let mut controller = crate::reference_controller::FlightController::default();
        let mut registers = SensorRegisters::default();
        registers.publish(5, 2.0, -0.25);
        let (reading, correction) = controller.update(2.0, &registers, 0.02).unwrap();
        assert_eq!(controller.latest, Some(reading));
        assert_eq!(reading.velocity_m_s, -0.25);
        assert!((correction - 0.625).abs() < 1e-9); // D uses decoded velocity
        registers.publish(6, 1.5, 0.0);
        let (reading, _) = controller.update(2.0, &registers, 0.02).unwrap();
        assert_eq!(reading.sample_id, 6);
        assert_eq!(controller.latest.unwrap().altitude_m, 1.5);
        assert!(
            controller
                .update(2.0, &SensorRegisters::default(), 0.02)
                .is_none()
        );
        assert!(controller.latest.is_none());
    }
}
