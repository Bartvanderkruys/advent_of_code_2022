use core::panic;
use std::fs;

const WIN: i8 = 6;
const DRAW: i8 = 3;
const LOSS: i8 = 0;

// hands:
// 1: rock
// 2: paper
// 3: scissors
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
fn determine_player_hand(strategy: i8, opponent: i8) -> i8 {
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

fn calculate_points_for_round(player: i8, opponent: i8) -> i32 {
    (match_result(player, opponent) + player).into()
}

pub fn solve() {
    let contents = fs::read_to_string("src/day_2_rock_paper_scissors/input.txt").unwrap();

    let score: i32 = contents
        .lines()
        .map(|line| {
            let opponent = get_hand(line.chars().nth(0).unwrap());
            let strategy = get_hand(line.chars().nth(2).unwrap());

            let player = determine_player_hand(strategy, opponent);

            calculate_points_for_round(player, opponent)
        })
        .sum();

    println!("Score: {}", score);
}
