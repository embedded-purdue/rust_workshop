//! Reference register driver and controller integration.
use crate::reference_pid::Pid;
use crate::sensor::{ALT_MM, READY, SAMPLE_ID, STATUS, SensorReading, SensorRegisters, VEL_MM_S};

pub fn read_sensor(registers: &SensorRegisters) -> Option<SensorReading> {
    let status = registers.read::<1>(STATUS)?[0];
    if status & READY == 0 {
        return None;
    }
    let sample_id = u16::from_le_bytes(registers.read::<2>(SAMPLE_ID)?);
    let altitude_mm = i32::from_le_bytes(registers.read::<4>(ALT_MM)?);
    let velocity_mm_s = i32::from_le_bytes(registers.read::<4>(VEL_MM_S)?);
    Some(SensorReading {
        sample_id,
        altitude_m: f64::from(altitude_mm) / 1000.0,
        velocity_m_s: f64::from(velocity_mm_s) / 1000.0,
    })
}

#[derive(Default)]
pub struct FlightController {
    pub pid: Pid,
    pub latest: Option<SensorReading>,
}
impl FlightController {
    pub fn update(
        &mut self,
        target: f64,
        registers: &SensorRegisters,
        dt: f64,
    ) -> Option<(SensorReading, f64)> {
        self.latest = read_sensor(registers);
        let reading = self.latest?;
        let correction = self
            .pid
            .update(target, reading.altitude_m, reading.velocity_m_s, dt);
        Some((reading, correction))
    }
}
