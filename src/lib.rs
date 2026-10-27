//! Device behavior shared by the terminal and browser activities.
#[derive(Debug, Default)]
pub struct Board {
    pub led: bool,
    pub duty: u8,
}
impl Board {
    pub fn toggle(&mut self) {
        self.led = !self.led;
    }
    pub fn set_duty(&mut self, duty: u8) -> Result<(), &'static str> {
        if duty > 100 {
            return Err("duty must be 0..=100");
        }
        self.duty = duty;
        Ok(())
    }
    /// Teaching model: speed proportional to duty, no inertia or electrical physics.
    pub fn rpm(&self) -> u32 {
        u32::from(self.duty) * 30
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn led_toggles_both_ways() {
        let mut board = Board::default();
        board.toggle();
        assert!(board.led);
        board.toggle();
        assert!(!board.led);
    }
    #[test]
    fn motor_boundaries_and_rejection() {
        let mut board = Board::default();
        for duty in [0, 50, 100] {
            board.set_duty(duty).unwrap();
            assert_eq!(board.rpm(), u32::from(duty) * 30);
        }
        assert!(board.set_duty(101).is_err());
        assert_eq!(board.duty, 100);
    }
}

pub mod flight;
#[path = "../03-flight_controller/pid.rs"]
pub mod pid;
#[path = "../03-flight_controller/sol/pid.rs"]
pub mod reference_pid;

#[path = "../03-flight_controller/controller.rs"]
pub mod controller;
#[path = "../03-flight_controller/sol/controller.rs"]
pub mod reference_controller;
pub mod sensor;
