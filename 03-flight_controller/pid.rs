//! Your firmware exercise. Rebuild the .fakefw file after changing this code.
#[derive(Default)]
pub struct Pid {
    pub integral: f64,
}
impl Pid {
    //returns vertical acceleration correction in m/s², bounded to 8.
    //start with P only, then implement I and D using the worksheet.
    pub fn update(&mut self, target: f64, altitude: f64, velocity: f64, dt: f64) -> f64 {
        let error = target - altitude;
        let kp = 3.0;
        // accumulate error * dt in self.integral, clamp to -3..3.
        // add ki * self.integral (try ki = 1.0).
        // subtract kd * velocity (try kd = 2.5).
        // Velocity is the derivative of measured altitude. Subtracting it damps motion.
        let _ = (velocity, dt); // remove once used
        (kp * error).clamp(-8.0, 8.0)
    }
}
