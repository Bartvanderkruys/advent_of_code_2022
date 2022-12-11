use std::fs;

use itertools::Itertools;

type Pos = (i32, i32);

fn get_next_tail_position(tail: Pos, head: Pos) -> Pos {
    let mut next_pos: Pos = tail.clone();

    let diff_x = head.0.abs_diff(tail.0);
    let diff_y = head.1.abs_diff(tail.1);

    if diff_x > 1 {
        if head.0 > tail.0 {
            next_pos.0 += 1;
        } else {
            next_pos.0 -= 1;
        }

        if diff_y > 0 {
            if head.1 > tail.1 {
                next_pos.1 += 1;
            } else {
                next_pos.1 -= 1;
            }
        }

        return next_pos;
    }

    if diff_y > 1 {
        if head.1 > tail.1 {
            next_pos.1 += 1;
        } else {
            next_pos.1 -= 1;
        }

        if diff_x > 0 {
            if head.0 > tail.0 {
                next_pos.0 += 1;
            } else {
                next_pos.0 -= 1;
            }
        }

        return next_pos;
    }

    next_pos
}

fn part1(input: &str) -> usize {
    let mut head_pos: Pos = (0, 0);
    let mut tail_path: Vec<Pos> = vec![(0, 0)];

    input.lines().for_each(|line| {
        let command: Vec<&str> = line.split(" ").collect();
        let count = command.iter().nth(1).unwrap().parse::<u8>().unwrap();

        for _ in 0..count {
            match command.iter().nth(0).unwrap() {
                &"U" => head_pos.1 += 1,
                &"D" => head_pos.1 -= 1,
                &"L" => head_pos.0 -= 1,
                &"R" => head_pos.0 += 1,
                _ => {
                    panic!("unknown direction")
                }
            }

            tail_path.push(get_next_tail_position(
                *tail_path.iter().last().unwrap(),
                head_pos,
            ))
        }
    });

    tail_path.iter().unique().count()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_9_rope_bridge/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13);
    }
}
