# Embedded Rust Workshop

https://docs.google.com/presentation/d/17qqcQKzA-OLw6lpu3uTMDN6e2AuPj4YBUoLNR_LbsRI/edit?usp=sharing


Print first Rust message, explore ownership and borrowing, then read simulated sensor registers and write an altitude controller for a fictional drone.

[Start exercise 01](01-print_loops/README.md) · [Flight worksheet](03-flight_controller/README.md) · [Sensor datasheet](03-flight_controller/TF01-DATASHEET.md)

## Setup

Install Rust and Cargo using [rustup](https://rustup.rs/). Use a toolchain that supports Rust edition 2024 (Rust 1.85 or later). Open this repository in your editor and keep a browser available for the flight app.

Run all Cargo commands from the repository root. The exercises share one Cargo package; their folders do not contain separate manifests.

```sh
rustc --version
cargo --version
cargo check --all-targets
```

The workshop has no external Rust dependencies and requires no hardware, account, or local web server. Open the flight HTML files directly in your browser rather than using GitHub's file preview.

## Exercises

| Exercise | What you will do | Run from the repository root |
| --- | --- | --- |
| [01: Print and loops](01-print_loops/README.md) | Print a message and an author variable; optionally repeat a message 17 times using a loop | `cargo run --bin hello` |
| [02: Memory](02-memory/README.md) | Compare copy and move, borrow text, update a counter through a mutable reference, and accumulate error | `cargo run --bin memory` |
| [03: Flight controller](03-flight_controller/README.md) | Decode and store sensor readings, implement PID, and build and fake-flash a flight image | `cargo run --bin firmware` |

The exercise 01 starter prints nothing until you add your code. In exercise 02, deliberately uncommenting the old-owner print demonstrates a compiler error; comment it again before continuing.

## Build, flash, and test your controller

Follow the [flight worksheet](03-flight_controller/README.md) and [datasheet](03-flight_controller/TF01-DATASHEET.md). First complete `03-flight_controller/controller.rs`, then add the integral and derivative terms in `03-flight_controller/pid.rs`.

```sh
cargo run --bin firmware
```

This writes `target/drone.fakefw`, containing 20 simulated seconds of flight data.

1. Open [web/drone.html](web/drone.html) locally in your browser.
2. Enter BOOT and select `target/drone.fakefw`.
3. Flash the image and wait for READY.
4. Arm and run the flight.

The target is 2 meters. A sustained downward gust starts at 5 simulated seconds. To pass, every sample in the final 5 seconds must have altitude error below 0.10 m and vertical speed below 0.15 m/s. Both the firmware builder and browser app report the result.

After editing Rust, rebuild, select the newly built image again, and reflash. The browser retains the previously selected image until you select a new one.

The unfinished sensor driver intentionally stores no readings and leaves the drone grounded. Once the driver works, the starter P-only controller still needs I and D to meet the hover goal.

## Solutions and reference flight

Each exercise keeps its solution in a `sol/` subfolder:

- [Print and loops solution](01-print_loops/sol/main.rs)
- [Memory solution](02-memory/sol/main.rs)
- [Sensor driver solution](03-flight_controller/sol/controller.rs) and [PID solution](03-flight_controller/sol/pid.rs)

```sh
cargo run --bin hello-solution
cargo run --bin memory-solution
cargo run --bin firmware -- --reference
```

The reference firmware command uses the solution driver and PID without changing student code. Select its `target/reference.fakefw` image in the student flight app to compare results.

For an already embedded reference flight with automatic flashing and playback, open [web/reference.html](web/reference.html) locally. That page contains a saved reference image; building new firmware does not update it.

## Repository layout

```text
01-print_loops/          First prints and optional loop exercise
  sol/                  Completed example
02-memory/              Ownership and borrowing exercise
  sol/                  Completed example
03-flight_controller/   Sensor driver, PID exercise, and datasheet
  sol/                  Reference driver and PID
src/                    Shared models, register bank, tests, and binaries
web/drone.html          Offline student flight app
web/drone.js            Flight image validation and browser playback
web/reference.html      Self-contained reference flight demo
web/index.html          Legacy LED/motor page; requires an absent server
```

## Extra terminal examples

```sh
cargo run --bin basics
cargo run --bin blink
cargo run --bin motor -- 75
```

These demonstrate Rust syntax, a simulated blinking LED, and a simple motor model. The legacy `web/index.html` page expects HTTP endpoints whose server is not included in this repository; use the terminal examples for these activities.

## Validation

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --bin firmware
cargo run --bin firmware -- --reference
```

Unit tests check the simulation, register decoding, reference controller, and basic student trace properties. Passing tests does not certify a completed student controller: use the builder's hover check and the app's PASS result to evaluate your flight.

## Simulation scope

Rust runs a one-axis drone model and writes telemetry to a `.fakefw` file. The browser's fake bootloader loads that data for playback. The board, pinout, and sensor hardware are fictional; this workflow does not emulate MCU instructions or flash physical hardware. Reloading the student app clears its fake flash.
