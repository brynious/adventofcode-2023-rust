advent_of_code::solution!(2);

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

#[derive(Debug)]
struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    hands: Vec<Hand>,
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

fn get_is_game_valid(hands: &[Hand]) -> bool {
    for hand in hands {
        if check_hand_valid(&hand) == false {
            return false;
        }
    }
    true
}

fn get_min_colors(hands: &[Hand]) -> Hand {
    let mut min_hand: Hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };
    for hand in hands {
        if hand.red > min_hand.red {
            min_hand.red = hand.red;
        }
        if hand.green > min_hand.green {
            min_hand.green = hand.green;
        }
        if hand.blue > min_hand.blue {
            min_hand.blue = hand.blue;
        }
    }
    min_hand
}

fn build_game(line: &str) -> Option<Game> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let game_parts: Vec<&str> = parts[0].split_whitespace().collect();
    let game_id: Option<u8> = if game_parts.len() >= 2 {
        game_parts[1].parse::<u8>().ok()
    } else {
        None
    };

    let mut hands: Vec<Hand> = Vec::new();
    for group in parts[1].split(';') {
        let hand: Hand = build_hand(group);
        hands.push(hand);
    }

    Some(Game {
        id: game_id?,
        hands,
    })
}

fn get_part_one_answer(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let game: Game = build_game(line)?;

        let game_is_valid: bool = get_is_game_valid(&game.hands);
        if game_is_valid == true {
            sum += game.id as u32;
        }
    }
    Some(sum as u32)
}

fn get_part_two_answer(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let game: Game = build_game(line)?;

        let min_hand: Hand = get_min_colors(&game.hands);
        let sum_min_hand: u32 = (if min_hand.red != 0 {
            min_hand.red as u32
        } else {
            1
        }) * (if min_hand.green != 0 {
            min_hand.green as u32
        } else {
            1
        }) * (if min_hand.blue != 0 {
            min_hand.blue as u32
        } else {
            1
        });
        answer += sum_min_hand as u32;
    }
    Some(answer)
}

pub fn part_one(input: &str) -> Option<u32> {
    get_part_one_answer(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_part_two_answer(input)
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
        assert_eq!(result, Some(2286));
    }
}
