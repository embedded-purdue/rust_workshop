fn label(name: &str) -> String {
    format!("Device: {name}") // final expression returns a value
}
fn main() {
    let name = String::from("virtual board");
    println!("{}", label(&name)); // borrow instead of moving name
    let mut blinks: u32 = 3;
    blinks += 1;
    let delays = [100, 200, 300];
    for delay in delays {
        let speed = if delay < 200 { "fast" } else { "slow" };
        println!("{name}: {blinks} blinks, {delay} ms, {speed}");
    }
    let mut readings = vec![21, 24];
    readings.push(27);
    match readings.first() {
        Some(value) => println!("First reading: {value}"),
        None => println!("No readings"),
    }
    match "75".parse::<u8>() {
        Ok(duty) => println!("Parsed duty: {duty}%"),
        Err(error) => println!("Invalid number: {error}"),
    }
}
