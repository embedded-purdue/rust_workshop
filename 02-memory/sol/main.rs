fn label(name: &str) {
    println!("Borrowed device name: {name}");
}
fn count_sample(samples: &mut u32) {
    *samples += 1;
}
fn main() {
    let first: u32 = 3;
    let copied = first;
    println!("Copy: first = {first}, copied = {copied}");
    let name = String::from("TF-01");
    let owner = name;
    println!("Move: new owner = {owner}");
    label(&owner);
    println!("Still owned here: {owner}");
    let mut samples = 0;
    count_sample(&mut samples);
    count_sample(&mut samples);
    println!("Samples collected: {samples} (goal: 2)");
    let mut integral = 0.0_f64;
    let error = 2.0;
    let dt = 0.02;
    integral += error * dt;
    println!("Controller memory: {integral:.2}");
}
