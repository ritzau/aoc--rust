use aoc::{s15, s24};
use itertools::Itertools;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::env::args;

fn main() {
    let old_stuff = args().contains(&"--old-stuff".to_string());

    setup_log();

    if let Err(err) = go_nuts(old_stuff) {
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

fn go_nuts(old_stuff: bool) -> aoc::PuzzleResult<()> {
    let start = std::time::Instant::now();

    if old_stuff {
        s15::solve()?;
    }
    s24::solve()?;

    let elapsed = start.elapsed();
    println!("\nTotal duration: {:?}", elapsed);

    Ok(())
}
