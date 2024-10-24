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

    let result = aoc::run([
        aoc::s15::e01::not_quite_lisp,
        aoc::s15::e02::i_was_told_there_would_be_no_math,
        aoc::s15::e03::perfectly_spherical_houses_in_a_vacuum,
        aoc::s15::e04::the_ideal_stocking_stuffer,
        aoc::s15::e05::doesnt_he_have_intern_elves_for_this,
        aoc::s15::e06::probably_a_fire_hazard,
        aoc::s15::e07::some_assembly_required,
        aoc::s15::e08::matchsticks,
        aoc::s15::e09::all_in_a_single_night,
        aoc::s15::e10::elves_look_elves_say,
        aoc::s15::e11::corporate_policy,
        aoc::s15::e12::js_abacus_framework_io,
        aoc::s15::e13::knights_of_the_dinner_table,
        aoc::s15::e14::reindeer_olympics,
        aoc::s15::e15::science_for_hungry_people,
        aoc::s15::e16::aunt_sue,
    ]);

    if let Err(err) = result {
        eprintln!("Failed to solve puzzles: {:?}", err);
    }
}
