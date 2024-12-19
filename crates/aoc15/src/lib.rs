use aoc::Year;

pub mod e01;
pub mod e02;
pub mod e03;
pub mod e04;
pub mod e05;
pub mod e06;
pub mod e07;
pub mod e08;
pub mod e09;
pub mod e10;
pub mod e11;
pub mod e12;
pub mod e13;
pub mod e14;
pub mod e15;
pub mod e16;
pub mod e17;
pub mod e18;
pub mod e19;
pub mod e20;

const YEAR: Year = Year(2015);

pub fn solve() -> aoc::PuzzleResult<()> {
    aoc::run([
        e01::not_quite_lisp,
        e02::i_was_told_there_would_be_no_math,
        e03::perfectly_spherical_houses_in_a_vacuum,
        e04::the_ideal_stocking_stuffer,
        e05::doesnt_he_have_intern_elves_for_this,
        e06::probably_a_fire_hazard,
        e07::some_assembly_required,
        e08::matchsticks,
        e09::all_in_a_single_night,
        e10::elves_look_elves_say,
        e11::corporate_policy,
        e12::js_abacus_framework_io,
        e13::knights_of_the_dinner_table,
        e14::reindeer_olympics,
        e15::science_for_hungry_people,
        e16::aunt_sue,
        e17::no_such_thing_as_too_much,
        e18::like_a_gif_for_your_yard,
        e19::medicine_for_rudolph,
        e20::infinite_elves_and_infinite_houses,
    ])
}
