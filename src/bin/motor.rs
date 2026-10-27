use rust_hour::Board;
fn run() -> Result<(), String> {
    let input = std::env::args().nth(1).unwrap_or_else(|| "50".into());
    let duty = input
        .parse::<u8>()
        .map_err(|_| "Enter a whole number from 0 to 100".to_string())?;
    let mut board = Board::default();
    board.set_duty(duty)?;
    println!(
        "PWM duty: {}% | model speed: {} RPM",
        board.duty,
        board.rpm()
    );
    println!(
        "[{}{}]",
        "#".repeat(usize::from(duty / 5)),
        ".".repeat(20 - usize::from(duty / 5))
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
