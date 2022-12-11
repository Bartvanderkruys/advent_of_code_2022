use std::fs;

use itertools::Itertools;

type Pos = (i32, i32);

fn get_next_tail_position(tail: &Pos, head: &Pos) -> Pos {
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
                tail_path.iter().last().unwrap(),
                &head_pos,
            ))
        }
    });

    tail_path.iter().unique().count()
}

fn part2(input: &str) -> usize {
    let mut head_pos: Pos = (0, 0);

    let mut knots: Vec<Vec<Pos>> = (0..9).map(|_| vec![(0, 0)]).collect();

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

            let first_knot = knots.iter_mut().nth(0).unwrap();

            first_knot.push(get_next_tail_position(
                first_knot.iter().last().unwrap(),
                &head_pos,
            ));

            (1..9).for_each(|i| {
                let previous = knots.iter().nth(i - 1).unwrap().clone();
                let knot = knots.iter_mut().nth(i).unwrap();

                knot.push(get_next_tail_position(
                    knot.iter().last().unwrap(),
                    previous.iter().last().unwrap(),
                ))
            })
        }
    });

    knots.iter().last().unwrap().iter().unique().count()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_9_rope_bridge/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT_PART_1);
        assert_eq!(result, 13);
    }

    const INPUT_PART_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part2_works() {
        let result = part2(INPUT_PART_2);
        assert_eq!(result, 36);
    }
}
