use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn main() {
    setup_log();

    if let Err(err) = go_nuts() {
        eprintln!("Failed to solve puzzles: {:?}", err);
    }
}

fn setup_log() {
    TermLogger::init(
        LevelFilter::Off,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn go_nuts() -> aoc::PuzzleResult<()> {
    let start = std::time::Instant::now();

    aoc24::solve()?;

    let elapsed = start.elapsed();
    println!("\nTotal duration: {:.0?}", elapsed);

    Ok(())
}
