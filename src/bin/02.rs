advent_of_code::solution!(2);

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

fn check_hand_valid(hand: &Hand) -> bool {
    if hand.red > RED || hand.green > GREEN || hand.blue > BLUE {
        false
    } else {
        true
    }
}

fn build_hand(group: &str) -> Hand {
    let mut hand: Hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };
    for color_count in group.split(',') {
        let color_parts: Vec<&str> = color_count.trim().split_whitespace().collect();
        if color_parts.len() < 2 {
            continue;
        }
        let count: u8 = color_parts[0].parse::<u8>().unwrap_or(0);
        match color_parts[1] {
            "red" => hand.red = count,
            "green" => hand.green = count,
            "blue" => hand.blue = count,
            _ => {}
        }
    }
    hand
}

fn get_game_id_if_hand_valid(line: &str) -> Option<u16> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let game_parts: Vec<&str> = parts[0].split_whitespace().collect();
    let game_number: Option<u16> = if game_parts.len() >= 2 {
        game_parts[1].parse::<u16>().ok()
    } else {
        None
    };

    for group in parts[1].split(';') {
        let hand: Hand = build_hand(group);
        if !check_hand_valid(&hand) {
            return None;
        }
    }

    game_number
}

fn get_part_one_answer(input: &str) -> Option<u32> {
    let mut sum: u16 = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(value) = get_game_id_if_hand_valid(line) {
            sum += value;
        }
    }
    Some(sum as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    get_part_one_answer(input)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
