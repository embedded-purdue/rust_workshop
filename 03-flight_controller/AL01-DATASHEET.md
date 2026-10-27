# Arman+Luke AL-01: datasheet

ENTIRELY FICTIONAL hardware for this course.

## What it does

AF-01 reads altitude and vertical speed, runs your controller 50 times per simulated second, and sends equal commands to four virtual motors. It controls height only. It starts on the ground and aims for 2 meters. At 5 seconds, a constant downward gust begins.

## Pinout

Top view, USB at the top. Click the corresponding pins in the app.

```text
                USB
      ┌─────────────────────┐
  1 ──┤ 3V3             M1  ├── 6
  2 ──┤ GND             M2  ├── 7
  3 ──┤ LED    TF-01    M3  ├── 8
  4 ──┤ ALT             M4  ├── 9
  5 ──┤ VEL           BOOT  ├── 10
      └─────────────────────┘
```

| Pin | Role | Units / meaning |
| --- | --- | --- |
| 1 / 3V3 | Fake power | Fictional 3.3 V supply |
| 2 / GND | Reference | Common reference |
| 3 / LED | Status output | Lit when playback is armed |
| 4 / ALT | Sensor input | Altitude in meters, positive upward |
| 5 / VEL | Sensor input | Vertical speed in m/s, positive upward |
| 6 / M1 | Front-left motor | Normalized duty 0–1 |
| 7 / M2 | Front-right motor | Same duty as M1 |
| 8 / M3 | Rear-right motor | Same duty as M1 |
| 9 / M4 | Rear-left motor | Same duty as M1 |
| 10 / BOOT | Loader input | Enter BOOT before flashing |

These are named logical signals. There are no real voltages, wiring checks, ADCs, physical GPIO registers, or PWM waveforms. The safe byte array below models a sensor register bank. Motor duty in the app represents the Rust model output, not a physical electrical signal.

## Fake sensor register map

The simulator publishes a fresh, coherent snapshot before every controller call. Firmware receives `&SensorRegisters`, a shared borrow of a read-only `[u8; 12]` register bank. This array is ordinary Rust memory, not an actual address on the computer or a physical device. No `unsafe`, pointer casts, or volatile reads are needed.

Offsets are relative to the start of this fake sensor bank. Read using `registers.read::<N>(offset)`, which returns `Option<[u8; N]>`. An invalid range returns `None`. `?` inside a function returning `Option` returns early on a failed read.

| Offset | Name | Width | Format | Meaning |
| --- | --- | --- | --- | --- |
| `0x00` | STATUS | 1 byte | u8 | Bit 0 = READY, other bits reserved |
| `0x01` | RESERVED | 1 byte | u8 | Ignore |
| `0x02` | SAMPLE_ID | 2 bytes | u16, little endian | Tick number, 0–999 |
| `0x04` | ALT_MM | 4 bytes | i32, little endian | Altitude in millimeters |
| `0x08` | VEL_MM_S | 4 bytes | i32, little endian | Vertical speed in mm/s, negative means down |

All registers are read-only from firmware. READY stays set for the published snapshot, and reading does not clear it. The snapshot cannot change while a controller borrows it. A default/unpublished bank has READY clear. Treat that as no reading, clear `latest`, and leave motors off.

### Decode and store

At 2 m altitude and −0.25 m/s vertical speed, the bank might contain:

```text
Offset       Bytes             Decoded value
0x00         01                READY
0x02         07 00             sample 7
0x04         D0 07 00 00       2000 mm = 2.000 m
0x08         06 FF FF FF       -250 mm/s = -0.250 m/s
```

Little endian means the least significant byte comes first. Use `u16::from_le_bytes` for SAMPLE_ID and `i32::from_le_bytes` for both sensor measurements. Signed decoding matters for downward velocity. Divide the integer measurements by `1000.0` after converting to `f64`. Measurements round to the nearest millimeter or mm/s.

Store the result in the provided owned struct:

```rust
pub struct SensorReading {
    pub sample_id: u16,
    pub altitude_m: f64,
    pub velocity_m_s: f64,
}
```

The firmware's `FlightController` owns `latest: Option<SensorReading>` and `pid: Pid`. `latest` retains the last successfully decoded sample between calls. Each new read replaces it. On a failed read, assign `None` rather than reusing stale data.

```text
fake sensor byte array
    read through &SensorRegisters
    decode signed integers and convert units
    store Some(SensorReading) in self.latest
    pass stored altitude_m and velocity_m_s to self.pid.update(...)
    convert correction to equal motor duty
```

Implement the register driver and storage in `03-flight_controller/controller.rs`; complete PID in `03-flight_controller/pid.rs`. The simulator never supplies true altitude or velocity directly to the student controller.

## Control interface

Implement `Pid::update` in `03-flight_controller/pid.rs`:

```rust
pub fn update(&mut self, target: f64, altitude: f64, velocity: f64, dt: f64) -> f64
```

Inputs are the altitude target, sensor values decoded from `self.latest`, and elapsed simulated seconds. Return an acceleration correction in m/s², limited to −8 through +8. Persistent controller memory belongs in `self`, such as `self.integral`.

| Constant | Value |
| --- | --- |
| Control period | 0.02 s / 50 Hz |
| Target | 2.0 m |
| Trace duration | 1000 samples / 20 seconds |
| Model hover duty | 0.5 / 50% |
| Allowed correction | −8 to +8 m/s² |
| Integral bound | −3 to +3 m·s (suggested exercise clamp) |
| Gust | −1 m/s² from t = 5 s onward |

## Fake physics

```text
throttle = clamp(0.5 + correction / 20, 0, 1)
acceleration = 20 * (throttle - 0.5) - 0.6 * velocity + wind
velocity = velocity + acceleration * dt
altitude = altitude + velocity * dt
```

The floor clamps altitude to zero and prevents negative velocity on contact. Rust records each sample before advancing the physics. All four motors receive the same throttle. The model has quantized sensors without noise and omits roll, pitch, yaw, battery drain, actuator delay, and motor dynamics.

## Fake flashing protocol

1. `cargo run --bin firmware` compiles and executes your host Rust controller against the model.
2. It writes `target/drone.fakefw`, a JSON telemetry image identified by `TINYFLIGHT-1`. It also records raw sensor registers and the actual reading stored by the controller for each tick.
3. Select the image in the app. Enter BOOT, then Flash image.
4. The loader validates the format, animates erase/copy, and stores the trace in page memory.
5. Arm replays that image. Pause freezes simulation time. Disarm/reset restarts the trace. Reload clears fake flash.

The image contains data, not machine instructions. The browser never executes uploaded Rust or native code. This workflow mimics the edit/build/flash/test cycle without CPU emulation. An older image keeps its old behavior even after you change the Rust source, until you rebuild and reflash.
