use aoc_runner_derive::aoc;
use std::iter::Sum;

struct Game {
    number: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

// When summing a collection of `Game`s, we want to sum the `number` field of each `Game`.
impl Sum<Game> for u32 {
    fn sum<I: Iterator<Item = Game>>(iter: I) -> Self {
        iter.map(|game| game.number).sum()
    }
}

// 2541
#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .filter(is_game_small_enough)
        .sum::<u32>()
}

// 66016
#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .map(power_set_of_cubes)
        .sum::<u32>()
}

/// Line example: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_game(line: &str) -> Game {
    let first_separator = line.find(':').unwrap();
    // All games are preceded by `Game `, so we can skip the first 5 characters
    let number = line[5..first_separator].parse::<u32>().unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    // All games start after `; `, so we can skip the first 2 characters (separator + space)
    line[first_separator + 2..]
        .split("; ")
        .for_each(|game_iteration| {
            // game_iteration example: "3 blue, 4 red"
            game_iteration.split(", ").for_each(|handful_of_cubes| {
                // handful_of_cubes example: "3 blue"
                let amount_color_separator = handful_of_cubes.find(' ').unwrap();
                let amount = handful_of_cubes[..amount_color_separator]
                    .parse::<u32>()
                    .unwrap();
                let color = &handful_of_cubes[amount_color_separator + 1..];
                match color {
                    "red" => max_red = max_red.max(amount),
                    "green" => max_green = max_green.max(amount),
                    "blue" => max_blue = max_blue.max(amount),
                    _ => panic!("Invalid color"),
                }
            });
        });

    Game {
        number,
        max_blue,
        max_red,
        max_green,
    }
}

fn is_game_small_enough(game: &Game) -> bool {
    game.max_blue <= MAX_BLUE && game.max_red <= MAX_RED && game.max_green <= MAX_GREEN
}

fn power_set_of_cubes(game: Game) -> u32 {
    game.max_blue * game.max_red * game.max_green
}
