# 03: Flight-controller solution

[controller.rs](controller.rs) is the complete register decoder and measurement-storage integration. [pid.rs](pid.rs) is the reference PID. It keeps the student's implementation separate.

```sh
cargo run --bin firmware -- --reference
python3 scripts/build-reference-demo.py
```

Open [the reference demo](../web/reference.html) for automatic fake flashing and playback, or flash `target/reference.fakefw` into [the student app](../web/drone.html).

Expected result with kp = 3, ki = 1, kd = 2.5: initial climb and some overshoot, a dip after the gust at 5 s, then recovery to 2 m. Every sample in the final 5 seconds satisfies the hover checks. At equilibrium after the gust the model needs about 55% motor duty, compared with its 50% no-wind hover duty.

To use this implementation as the student's own controller, copy this folder's `pid.rs` into `03-flight_controller/pid.rs` and `controller.rs` into `03-flight_controller/controller.rs`. In the copied controller, change `use crate::reference_pid::Pid;` to `use crate::pid::Pid;`. Then run `cargo run --bin firmware`. That overwrites their exercise, so instructors should normally use `--reference` instead.

## Facilitator notes

- Ask students to predict each experiment before flashing.
- If they get a compiler error, relate `&mut self` to the counter from exercise 02.
- If tests pass but the mission fails, remind them the model/reference tests do not certify the student's gains.
- Leave a few minutes for the distinction between data playback, chip emulation, and physical hardware.

The reference uses the same register bank and signed unit conversions as students. The app shows both the register values and actual stored measurements passed to PID. Sensor quantization is 1 mm and 1 mm/s. Missing sensor data disables motor output.
