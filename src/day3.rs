use aoc_runner_derive::aoc;

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const MAX_WITDH_OR_HEIGHT: usize = 140;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let engine_schematic: Vec<&str> = input.lines().collect();
    let mut accumulator: u32 = 0;

    for (line, &schematic) in engine_schematic.iter().enumerate() {
        let mut position: u32 = 0;
        let line_as_chars: Vec<char> = schematic.chars().collect();

        while let Some((number_start_pos, number_end_pos)) =
            get_next_number_positions(position, &line_as_chars)
        {
            // Set min and max bounds for the next checks
            let min_start_pos = if number_start_pos > 0 {
                number_start_pos - 1
            } else {
                0
            };
            let max_end_pos = if number_end_pos < MAX_WITDH_OR_HEIGHT {
                number_end_pos + 1
            } else {
                MAX_WITDH_OR_HEIGHT
            };

            // Check if the upper row has a symbol between the `number_start_pos` - 1 and `number_end_pos`
            let upper_has_symbol = if line > 0 {
                let upper_row = engine_schematic[line - 1];
                str_has_symbol(&upper_row[min_start_pos..max_end_pos])
            } else {
                false
            };

            // Check if the lower row has a symbol between the `number_start_pos` - 1 and `number_end_pos`
            let lower_has_symbol = if line < engine_schematic.len() - 1 {
                let lower_row = engine_schematic[line + 1];
                str_has_symbol(&lower_row[min_start_pos..max_end_pos])
            } else {
                false
            };

            // Check if the char to the left of the number is a symbol
            let left_has_symbol = if number_start_pos > 0 {
                str_has_symbol(&engine_schematic[line][min_start_pos..number_start_pos])
            } else {
                false
            };

            // Check if the char to the right of the number is a symbol
            let right_has_symbol = if number_end_pos < MAX_WITDH_OR_HEIGHT {
                str_has_symbol(&engine_schematic[line][number_end_pos..max_end_pos])
            } else {
                false
            };

            // If any of the previous checks are true, add the number to the accumulator
            if upper_has_symbol || lower_has_symbol || left_has_symbol || right_has_symbol {
                let number = engine_schematic[line][number_start_pos..number_end_pos]
                    .parse::<u32>()
                    .unwrap();
                accumulator += number;
            }

            if number_end_pos == MAX_WITDH_OR_HEIGHT {
                break;
            } else {
                position = number_end_pos as u32;
            }
        }
    }

    accumulator
}

fn str_has_symbol(input: &str) -> bool {
    input.chars().any(|c| c != '.' && !NUMBERS.contains(&c))
}

fn get_next_number_positions(starting_pos: u32, line: &[char]) -> Option<(usize, usize)> {
    let mut start_pos: usize = starting_pos as usize;
    // Iterate over the string until we find a number
    while !NUMBERS.contains(&line[start_pos]) {
        start_pos += 1;

        // If we reach the end of the string while searching for the starting position, return None
        if start_pos == line.len() {
            return None;
        }
    }

    // Once we find a number, we iterate until we find a non-number
    let mut end_pos: usize = start_pos;
    while (end_pos < line.len()) && NUMBERS.contains(&line[end_pos]) {
        end_pos += 1;
    }

    Some((start_pos, end_pos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("...", false)]
    #[case(".1.", false)]
    #[case(".1#", true)]
    #[case(".1%", true)]
    #[case(".1/", true)]
    #[case(".1*", true)]
    #[case(".1=", true)]
    fn test_str_has_symbol(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(str_has_symbol(input), expected);
    }

    #[rstest]
    #[case(0, "......", None)]
    #[case(0, "..123.", Some((2, 5)))]
    #[case(0, ".....1", Some((5, 6)))]
    #[case(0, "...463", Some((3, 6)))]
    fn test_get_next_number_positions(
        #[case] input: u32,
        #[case] line: &str,
        #[case] expected: Option<(usize, usize)>,
    ) {
        let line_as_chars: Vec<char> = line.chars().collect();
        assert_eq!(get_next_number_positions(input, &line_as_chars), expected);
    }
}
