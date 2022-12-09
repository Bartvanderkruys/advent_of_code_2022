use itertools::Itertools;
use std::fs;

fn part1(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .position(|(i, _)| {
            let sub_string: Vec<char> = input[i..i + 4].chars().collect();
            sub_string.iter().unique().count() == 4
        })
        .unwrap()
        + 4
}

pub fn solve() {
    let input = fs::read_to_string("src/day_6_tuning_trouble/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 7);
    }
}
