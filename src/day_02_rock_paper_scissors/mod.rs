use core::panic;
use std::fs;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSS: u8 = 0;

// hands:
// 1: rock
// 2: paper
// 3: scissors
fn get_hand(x: char) -> u8 {
    match x {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Illegal hand!"),
    }
}

fn match_result(player: u8, opponent: u8) -> u8 {
    match player {
        1 => match opponent {
            2 => LOSS,
            3 => WIN,
            _ => DRAW,
        },
        2 => match opponent {
            1 => WIN,
            3 => LOSS,
            _ => DRAW,
        },
        3 => match opponent {
            1 => LOSS,
            2 => WIN,
            _ => DRAW,
        },
        _ => 0,
    }
}

// strategy:
// 1: lose
// 2: draw
// 3: win
fn determine_player_hand(strategy: u8, opponent: u8) -> u8 {
    match opponent {
        1 => match strategy {
            1 => 3,
            3 => 2,
            _ => 1,
        },
        2 => match strategy {
            1 => 1,
            3 => 3,
            _ => 2,
        },
        3 => match strategy {
            1 => 2,
            3 => 1,
            _ => 3,
        },
        _ => 0,
    }
}

fn calculate_points_for_round(player: u8, opponent: u8) -> u32 {
    (match_result(player, opponent) + player).into()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let opponent = get_hand(line.chars().nth(0).unwrap());
            let player = get_hand(line.chars().nth(2).unwrap());

            calculate_points_for_round(player, opponent)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let opponent = get_hand(line.chars().nth(0).unwrap());
            let strategy = get_hand(line.chars().nth(2).unwrap());

            let player = determine_player_hand(strategy, opponent);

            calculate_points_for_round(player, opponent)
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_02_rock_paper_scissors/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 12);
    }
}
