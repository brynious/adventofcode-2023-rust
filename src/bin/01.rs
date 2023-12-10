advent_of_code::solution!(1);

const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_digit(line: &str, include_words: bool) -> Option<&str> {
    let patterns: Vec<&str> = if include_words {
        [DIGITS, DIGIT_WORDS].concat()
    } else {
        DIGITS.to_vec()
    };

    let mut first_index = None;
    let mut first_digit = None;

    for &pattern in &patterns {
        if let Some(index) = line.find(pattern) {
            if first_index.map_or(true, |first_index| index < first_index) {
                first_index = Some(index);
                first_digit = Some(pattern);
            }
        }
    }

    first_digit
}

fn get_last_digit(line: &str, include_words: bool) -> Option<&str> {
    let patterns: Vec<&str> = if include_words {
        [DIGITS, DIGIT_WORDS].concat()
    } else {
        DIGITS.to_vec()
    };

    let mut last_index = None;
    let mut last_digit = None;

    for &pattern in &patterns {
        if let Some(index) = line.rfind(pattern) {
            if last_index.map_or(true, |last_index| index > last_index) {
                last_index = Some(index);
                last_digit = Some(pattern);
            }
        }
    }

    last_digit
}

fn map_word_to_digit(digit: Option<&str>) -> Option<&str> {
    match digit {
        Some(digit_str) => match digit_str {
            "zero" => Some("0"),
            "one" => Some("1"),
            "two" => Some("2"),
            "three" => Some("3"),
            "four" => Some("4"),
            "five" => Some("5"),
            "six" => Some("6"),
            "seven" => Some("7"),
            "eight" => Some("8"),
            "nine" => Some("9"),
            _ => Some(digit_str),
        },
        None => None,
    }
}

fn combine_and_convert(first_digit: Option<&str>, last_digit: Option<&str>) -> Option<u32> {
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let combined = format!("{}{}", first, last);
            combined.parse::<u32>().ok()
        }
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit: Option<&str> = get_first_digit(line, false);
        let last_digit: Option<&str> = get_last_digit(line, false);
        if let Some(number) = combine_and_convert(first_digit, last_digit) {
            sum += number;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit: Option<&str> = map_word_to_digit(get_first_digit(line, true));
        let last_digit: Option<&str> = map_word_to_digit(get_last_digit(line, true));
        if let Some(number) = combine_and_convert(first_digit, last_digit) {
            sum += number;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
