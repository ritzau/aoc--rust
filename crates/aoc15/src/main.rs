fn main() {
    let result = aoc15::run([
        aoc15::e01_not_quite_lisp,
        aoc15::e02_i_was_told_there_would_be_no_math,
        aoc15::e03_perfectly_spherical_houses_in_a_vacuum,
        aoc15::e04_the_ideal_stocking_stuffer,
        aoc15::e05_doesnt_he_have_intern_elves_for_this,
        aoc15::e06_probably_a_fire_hazard,
    ]);

    if let Err(err) = result {
        eprintln!("Failed to solve puzzles: {:?}", err);
    }
}
