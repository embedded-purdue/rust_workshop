fn label(name: &str) {
    println!("Borrowed device name: {name}");
}

fn count_sample(samples: &mut u32) {
    // I want you to increment the caller's counter using *samples += 1;
    // Free candy for the first person who can tell me why we use the *
    let _ = samples;
}

fn main() {
    let first: u32 = 3;
    let copied = first;
    println!("Copy: first = {first}, copied = {copied}");

    let name = String::from("TF-01");
    let owner = name;
    println!("Move: new owner = {owner}");
    //uncomment this line, run cargo check --bin memory, and read the error.
    //println!("Old owner: {name}");
    //then comment it again so the program compiles.

    label(&owner);
    println!("Still owned here: {owner}");

    let mut samples = 0;
    count_sample(&mut samples);
    count_sample(&mut samples);
    println!("Samples collected: {samples} (goal: 2)");

    let mut integral = 0.0_f64;
    let error = 2.0;
    let dt = 0.02;

    // PRE PID EXERSIZE
    //accumulate error * dt in integral. Goal: print 0.04.
    let _ = (error, dt, &mut integral);
    println!("Controller memory: {integral:.2}");
}
