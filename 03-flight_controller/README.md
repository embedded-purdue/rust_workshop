# 03: Fake flight-controller firmware
## Goal

Read sensor registers, store decoded measurements, add I and D to an altitude controller, and use the build/flash/test cycle to make a fictional drone hover at 2 m through a gust.

## Files

- [TF01-DATASHEET.md](TF01-DATASHEET.md): pin map, units, timing, and firmware interface.
- [controller.rs](controller.rs): TODOs for reading registers, decoding, and storing samples.
- [pid.rs](pid.rs): your student firmware. This file is the module used by the firmware builder.
- [Board app](../web/drone.html): clickable pins, fake bootloader, and flight playback.

## 25–33 minutes: read the datasheet and implement the sensor driver

Read the datasheet's pin table and sensor register map. Find ALT, VEL, M1-M4, and BOOT in the app. Identify which register contains a signed measurement and which units the PID expects.

Open `controller.rs`. The `read_sensor` function borrows the register bank with `&SensorRegisters`. Complete its TODOs:

1. Import the provided register constants from `crate::sensor`.
2. Read STATUS with `registers.read::<1>(STATUS)?[0]`. If `status & READY == 0`, return `None`.
3. Read two SAMPLE_ID bytes and decode using `u16::from_le_bytes`.
4. Read four bytes each from ALT_MM and VEL_MM_S and decode using `i32::from_le_bytes`.
5. Convert to f64 and divide by 1000.0, then construct `Some(SensorReading { ... })`.
6. In `FlightController::update`, assign `self.latest = read_sensor(registers);` instead of the `None` placeholder. Remove the unused register placeholder.

`Option` represents a present or absent reading. The `?` operator on `self.latest` returns `None` immediately when no reading exists. The provided glue passes the **stored** fields into PID. `self.latest` is owned memory and remains available after the temporary register borrow ends.

The given `struct Pid` also holds the integral between calls. `&mut self` permits updating both measurement memory and PID memory. The final expression without a semicolon returns the correction.

Checkpoint: explain `bytes -> signed integer -> meters -> stored struct -> PID`. If you build before completing the sensor TODOs, it reports 0/1000 stored readings and the drone stays on the ground.

## first build and fake flash

Run from the repository root:

```sh
cargo run --bin firmware
```

Open `web/drone.html` directly in a browser, then:

1. Enter BOOT.
2. Select `target/drone.fakefw`.
3. Flash image and wait for READY.
4. Arm & run :0 

<!-- go later and add if i think there is enough time -->
After the sensor driver is connected, P-only firmware intentionally misses the hover goal. The builder should now report 1000/1000 stored sensor readings. At 5 simulated seconds a sustained downward gust begins, and P needs a lasting height error to push back against it.

## implement PID controller 

Edit `03-flight_controller/pid.rs`. Keep `error = target - altitude` and `kp = 3.0`.

1. Accumulate `error * dt` in `self.integral`, clamping it between −3 and +3. This is the same operation you practiced with a local variable in exercise 02.
2. Add `ki * self.integral`, with `ki = 1.0`.
3. Subtract `kd * velocity`, with `kd = 2.5`.
4. Clamp the total correction to −8 through +8. (min. max.)

```text
error = target - altitude
integral = clamp(integral + error * dt, -3, 3)
correction = clamp(kp * error + ki * integral - kd * velocity, -8, 8)
```

P reacts to current error. I remembers error to reject the sustained gust. D opposes motion to reduce oscillation. Velocity is the derivative of measured altitude, so this D term uses the supplied velocity sensor.

Build again, select the new file again, and reflash. The browser retains an already selected image until you select its rebuilt version. Editing Rust alone cannot change the flashed image.

## 48–55 minutes: test and compare

Goal: every sample in the last 5 simulated seconds stays within 0.10 m of target and below 0.15 m/s vertical speed. The builder and app should report PASS.

If time remains, try setting ki to 0, then restore it. Try kd = 0, then restore it. Predict the change before each run

```sh
cargo run --bin firmware -- target/my-pid.fakefw
```
