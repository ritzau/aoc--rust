use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleResult};
use regex::Regex;
use std::cmp::max;
use std::sync::LazyLock;

const DAY: Day = Day(15);

pub fn science_for_hungry_people(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Science for Hungry People");

    let mut ingredients = Vec::<Ingredient>::new();
    for line in aoc.get_input(YEAR, DAY)?.lines()? {
        ingredients.push(Ingredient::parse(&line));
    }

    let max_score = get_max_score(&ingredients, false);
    let max_restricted_score = get_max_score(&ingredients, true);

    println!("aoc15e15a: {}", max_score);
    println!("aoc15e15b: {}", max_restricted_score);

    Ok(max_score == 21367368 || max_restricted_score == 1766400)
}

fn get_max_score(ingredients: &[Ingredient], exactly_500: bool) -> i64 {
    let mut max_score = 0i64;

    generate_combinations(ingredients.len(), 100, |combination| {
        let capacity = sum_prop(ingredients, combination, |i| i.capacity);
        let durability = sum_prop(ingredients, combination, |i| i.durability);
        let flavor = sum_prop(ingredients, combination, |i| i.flavor);
        let texture = sum_prop(ingredients, combination, |i| i.texture);
        let calories = sum_prop(ingredients, combination, |i| i.calories);

        if exactly_500 && calories != 500 {
            return;
        }

        let score = max(0, capacity) * max(0, durability) * max(0, flavor) * max(0, texture);
        max_score = max(max_score, score);
    });

    max_score
}

fn sum_prop(ingredients: &[Ingredient], combination: &[i64], x: fn(&Ingredient) -> i64) -> i64 {
    ingredients
        .iter()
        .zip(combination.iter())
        .map(|(i, a)| x(i) * a)
        .sum()
}

fn foreach_combination<F>(max: i64, ingredients: &mut [i64], index: usize, callback: &mut F)
where
    F: FnMut(&[i64]),
{
    let remaining = max - ingredients[..index].iter().sum::<i64>();
    if index == ingredients.len() - 1 {
        ingredients[index] = remaining;
        callback(ingredients);
    } else {
        for i in 1..=remaining {
            ingredients[index] = i;
            foreach_combination(max, ingredients, index + 1, callback);
        }
    }
}

fn generate_combinations<F: FnMut(&[i64])>(n: usize, max: i64, mut callback: F) {
    foreach_combination(max, &mut vec![0i64; n], 0, &mut callback);
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

static INGREDIENTS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let re = r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)";
    Regex::new(re).unwrap()
});

impl Ingredient {
    #[allow(dead_code)] // Used for tests
    fn new(capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64) -> Ingredient {
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }

    fn parse(s: &str) -> Ingredient {
        if let Some(caps) = INGREDIENTS_REGEX.captures(s) {
            let capacity = caps.get(2).unwrap().as_str().parse().unwrap();
            let durability = caps.get(3).unwrap().as_str().parse().unwrap();
            let flavor = caps.get(4).unwrap().as_str().parse().unwrap();
            let texture = caps.get(5).unwrap().as_str().parse().unwrap();
            let calories = caps.get(6).unwrap().as_str().parse().unwrap();

            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        } else {
            panic!("Invalid input: {}", s);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ingredient_parse() {
        let i = Ingredient::parse(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
        );

        assert_eq!(i.capacity, -1);
        assert_eq!(i.durability, -2);
        assert_eq!(i.flavor, 6);
        assert_eq!(i.texture, 3);
        assert_eq!(i.calories, 8);
    }

    #[test]
    fn test_get_max_score() {
        let ingredients = vec![
            Ingredient::new(-1, -2, 6, 3, 8),
            Ingredient::new(2, 3, -2, -1, 3),
        ];

        assert_eq!(get_max_score(&ingredients, false), 62842880);
        assert_eq!(get_max_score(&ingredients, true), 57600000);
    }
}
