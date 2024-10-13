use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn main() {
    TermLogger::init(
        LevelFilter::Off,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let result = aoc15::run([
        aoc15::e01::not_quite_lisp,
        aoc15::e02::i_was_told_there_would_be_no_math,
        aoc15::e03::perfectly_spherical_houses_in_a_vacuum,
        aoc15::e04::the_ideal_stocking_stuffer,
        aoc15::e05::doesnt_he_have_intern_elves_for_this,
        aoc15::e06::probably_a_fire_hazard,
        aoc15::e07::some_assembly_required,
        aoc15::e08::matchsticks,
        aoc15::e09::all_in_a_single_night,
        aoc15::e10::elves_look_elves_say,
        aoc15::e11::corporate_policy,
        aoc15::e12::js_abacus_framework_io,
        aoc15::e13::knights_of_the_dinner_table,
    ]);

    if let Err(err) = result {
        eprintln!("Failed to solve puzzles: {:?}", err);
    }
}
