use aoc_runner_derive::aoc;

// 55447
#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| get_first_and_last_numbers(l, false))
        .map(parse_pair)
        .sum::<u32>()
}

// 54706
#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| get_first_and_last_numbers(l, true))
        .map(parse_pair)
        .sum::<u32>()
}

fn parse_pair((first, last): (char, char)) -> u32 {
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

fn replace_string_with_number(input: &str) -> String {
    input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn get_first_and_last_numbers(input: &str, include_words: bool) -> (char, char) {
    let mut first: char = '0';
    let mut acc_first: String = "".to_string();
    let mut last: char = '0';
    let mut acc_last: String = "".to_string();

    let mut first_iter = input.chars();
    while first == '0' {
        first_iter.next().map(|c| {
            if c.is_numeric() {
                first = c;
            } else {
                if include_words {
                    acc_first.push(c);
                    acc_first = replace_string_with_number(&acc_first);
                    first = match acc_first
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<Vec<char>>()
                        .first()
                    {
                        Some(c) => *c,
                        None => '0',
                    }
                }
            }
        });
    }

    let mut last_iter = input.chars().rev();
    while last == '0' {
        last_iter.next().map(|c| {
            if c.is_numeric() {
                last = c;
            } else {
                if include_words {
                    // prepend the character to the string
                    acc_last = format!("{}{}", c, acc_last);
                    acc_last = replace_string_with_number(&acc_last);
                    last = match acc_last
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<Vec<char>>()
                        .first()
                    {
                        Some(c) => *c,
                        None => '0',
                    }
                }
            }
        });
    }

    (first, last)
}
