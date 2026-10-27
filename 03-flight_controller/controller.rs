//! Student sensor driver. Complete this before adding I and D to pid.rs.
use crate::pid::Pid;
use crate::sensor::{SensorReading, SensorRegisters};
// You will need: STATUS, READY, SAMPLE_ID, ALT_MM, VEL_MM_S from crate::sensor.

pub fn read_sensor(registers: &SensorRegisters) -> Option<SensorReading> {
    // read the one-byte STATUS register. If READY bit is clear, return None.
    // read SAMPLE_ID as two bytes, then use u16::from_le_bytes.
    // read ALT_MM and VEL_MM_S as four bytes each, use i32::from_le_bytes.
    // convert mm and mm/s to m and m/s by dividing by 1000.0.
    // return Some(SensorReading { sample_id, altitude_m, velocity_m_s }).
    // Example of a safe read: let bytes = registers.read::<4>(crate::sensor::ALT_MM)?;
    let _ = registers;
    None // no reading yet: firmware leaves motors off
}

#[derive(Default)]
pub struct FlightController {
    pub pid: Pid,
    /// Owned data retained between calls. A failed read clears stale data.
    pub latest: Option<SensorReading>,
}
impl FlightController {
    pub fn update(
        &mut self,
        target: f64,
        registers: &SensorRegisters,
        dt: f64,
    ) -> Option<(SensorReading, f64)> {
        //replace None with read_sensor(registers) to save a fresh reading.
        self.latest = None;
        let _ = registers;
        let reading = self.latest?;
        //the provided code passes the stored, decoded values into your PID.
        let correction = self
            .pid
            .update(target, reading.altitude_m, reading.velocity_m_s, dt);
        Some((reading, correction))
    }
}
