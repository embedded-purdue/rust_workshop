//! Fictional one-axis drone model. No physical hardware or chip emulation.
use crate::sensor::{SensorReading, SensorRegisters};
pub const DT: f64 = 0.02;
pub const TARGET: f64 = 2.0;
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub time: f64,
    pub altitude: f64,
    pub velocity: f64,
    pub throttle: f64,
    pub wind: f64,
    pub reading: Option<SensorReading>,
    pub raw_altitude_mm: i32,
    pub raw_velocity_mm_s: i32,
    pub sample_id: u16,
}
pub fn simulate(
    mut controller: impl FnMut(f64, &SensorRegisters, f64) -> Option<(SensorReading, f64)>,
) -> Vec<Sample> {
    let (mut altitude, mut velocity) = (0.0_f64, 0.0_f64);
    let mut samples = Vec::new();
    let mut registers = SensorRegisters::default();
    for step in 0..1000 {
        let time = f64::from(step) * DT;
        let sample_id = step as u16;
        registers.publish(sample_id, altitude, velocity);
        let output = controller(TARGET, &registers, DT);
        let reading = output.map(|(reading, _)| reading);
        let correction = output.map_or(f64::NAN, |(_, correction)| correction);
        // Invalid student output fails closed instead of corrupting the artifact.
        let throttle = if correction.is_finite() {
            (0.5 + correction.clamp(-8.0, 8.0) / 20.0).clamp(0.0, 1.0)
        } else {
            0.0 // missing sensor data or invalid controller output: motors off
        };
        let wind = if time >= 5.0 { -1.0 } else { 0.0 };
        samples.push(Sample {
            time,
            altitude,
            velocity,
            throttle,
            wind,
            reading,
            sample_id,
            raw_altitude_mm: i32::from_le_bytes(registers.read(crate::sensor::ALT_MM).unwrap()),
            raw_velocity_mm_s: i32::from_le_bytes(registers.read(crate::sensor::VEL_MM_S).unwrap()),
        });
        let acceleration = 20.0 * (throttle - 0.5) - 0.6 * velocity + wind;
        velocity += acceleration * DT;
        altitude += velocity * DT;
        if altitude < 0.0 {
            altitude = 0.0;
            velocity = velocity.max(0.0);
        }
    }
    samples
}
pub fn student_flight() -> Vec<Sample> {
    let mut controller = crate::controller::FlightController::default();
    simulate(|target, registers, dt| controller.update(target, registers, dt))
}
pub fn reference_flight() -> Vec<Sample> {
    let mut controller = crate::reference_controller::FlightController::default();
    simulate(|target, registers, dt| controller.update(target, registers, dt))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_sensor_data_stays_grounded() {
        assert!(simulate(|_, _, _| None).iter().all(|s| s.altitude == 0.0));
    }
    #[test]
    fn student_trace_is_finite_and_bounded() {
        let samples = student_flight();
        assert_eq!(samples.len(), 1000);
        assert!(samples.iter().all(|s| s.altitude.is_finite()
            && s.altitude >= 0.0
            && (0.0..=1.0).contains(&s.throttle)));
    }
    #[test]
    fn reference_recovers_from_constant_disturbance() {
        let samples = reference_flight();
        assert!(
            samples
                .iter()
                .filter(|s| s.time >= 15.0)
                .all(|s| (TARGET - s.altitude).abs() < 0.1 && s.velocity.abs() < 0.15)
        );
    }
}
