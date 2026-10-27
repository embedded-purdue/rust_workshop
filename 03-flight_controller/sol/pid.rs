//! Reference PID for the fictional TF-01 altitude model.
#[derive(Default)]
pub struct Pid {
    pub integral: f64,
}
impl Pid {
    pub fn update(&mut self, target: f64, altitude: f64, velocity: f64, dt: f64) -> f64 {
        let error = target - altitude;
        self.integral = (self.integral + error * dt).clamp(-3.0, 3.0);
        let kp = 3.0;
        let ki = 1.0;
        let kd = 2.5;
        let p = kp * error;
        let i = ki * self.integral;
        let d = -kd * velocity;
        (p + i + d).clamp(-8.0, 8.0)
    }
}
