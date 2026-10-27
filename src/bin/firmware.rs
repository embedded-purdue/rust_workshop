//! Build a trace image from the Rust controller for the offline fake bootloader.
use rust_hour::flight::{TARGET, reference_flight, student_flight};
use std::{fmt::Write as _, fs, io};
fn main() -> io::Result<()> {
    let reference = std::env::args().any(|arg| arg == "--reference");
    let path = std::env::args()
        .skip(1)
        .find(|arg| arg != "--reference")
        .unwrap_or_else(|| {
            if reference {
                "target/reference.fakefw".into()
            } else {
                "target/drone.fakefw".into()
            }
        });
    let samples = if reference {
        reference_flight()
    } else {
        student_flight()
    };
    let name = if reference {
        "reference-pid"
    } else {
        "student-pid"
    };
    let mut image = format!(
        r#"{{"magic":"TINYFLIGHT-1","name":"{name}","target":{TARGET},"dt":0.02,"samples":["#
    );
    for (i, s) in samples.iter().enumerate() {
        if i > 0 {
            image.push(',');
        }
        write!(
            image,
            "[{:.2},{:.6},{:.6},{:.6},{:.1}]",
            s.time, s.altitude, s.velocity, s.throttle, s.wind
        )
        .unwrap();
    }
    image.push_str("],\"sensor_samples\":[");
    for (i, s) in samples.iter().enumerate() {
        if i > 0 {
            image.push(',');
        }
        let stored = s.reading.map_or_else(
            || "null".to_owned(),
            |reading| {
                format!(
                    "[{}, {:.6}, {:.6}]",
                    reading.sample_id, reading.altitude_m, reading.velocity_m_s
                )
            },
        );
        write!(
            image,
            "[{},{},{},{}]",
            s.sample_id, s.raw_altitude_mm, s.raw_velocity_mm_s, stored
        )
        .unwrap();
    }
    image.push_str("]}");
    fs::write(&path, image)?;
    let final_error = (TARGET - samples.last().unwrap().altitude).abs();
    let pass = samples
        .iter()
        .filter(|s| s.time >= 15.0)
        .all(|s| (TARGET - s.altitude).abs() < 0.1 && s.velocity.abs() < 0.15);
    println!("Built {path}: 1000 samples, 20 simulated seconds");
    println!(
        "Stored sensor readings: {}/1000",
        samples.iter().filter(|s| s.reading.is_some()).count()
    );
    println!(
        "Final error: {final_error:.3} m. Last 5 s hover check: {}",
        if pass { "PASS" } else { "KEEP TUNING" }
    );
    println!("Open web/drone.html, enter BOOT, select this file, then Flash and Arm.");
    Ok(())
}
