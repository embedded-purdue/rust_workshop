use rust_hour::Board;
use std::{thread, time::Duration};
fn main() {
    let mut board = Board::default();
    // Activity: change the count and delay, then predict the final state.
    for tick in 1..=8 {
        board.toggle();
        println!(
            "tick {tick}: LED {}",
            if board.led { "ON  (*)" } else { "OFF ( )" }
        );
        thread::sleep(Duration::from_millis(250));
    }
}
