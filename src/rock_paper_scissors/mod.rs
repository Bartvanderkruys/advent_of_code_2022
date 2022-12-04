use core::panic;
use std::fs;

const WIN: i8 = 6;
const DRAW: i8 = 3;
const LOSS: i8 = 0;

fn get_hand(x: char) -> i8 {
    match x {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Illegal hand!"),
    }
}

fn match_result(player: i8, opponent: i8) -> i8 {
    match player {
        1 => match opponent {
            1 => DRAW,
            2 => LOSS,
            3 => WIN,
            _ => 0,
        },
        2 => match opponent {
            1 => WIN,
            2 => DRAW,
            3 => LOSS,
            _ => 0,
        },
        3 => match opponent {
            1 => LOSS,
            2 => WIN,
            3 => DRAW,
            _ => 0,
        },
        _ => 0,
    }
}

fn calculate_points_for_round(player: i8, opponent: i8) -> i32 {
    (match_result(player, opponent) + player).into()
}

pub fn solve() {
    let contents = fs::read_to_string("src/rock_paper_scissors/input.txt").unwrap();

    let mut score: i32 = 0;

    for line in contents.lines() {
        score += calculate_points_for_round(
            get_hand(line.chars().nth(2).unwrap()),
            get_hand(line.chars().nth(0).unwrap()),
        );
    }

    println!("Score: {}", score);
}
