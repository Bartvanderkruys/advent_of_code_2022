use itertools::Itertools;
use std::fs;

fn find_unique_character_index(input: &str, number_of_characters: usize) -> usize {
    input
        .chars()
        .enumerate()
        .position(|(i, _)| {
            let sub_string: Vec<char> = input[i..i + number_of_characters].chars().collect();
            sub_string.iter().unique().count() == number_of_characters
        })
        .unwrap()
        + number_of_characters
}

fn part1(input: &str) -> usize {
    find_unique_character_index(input, 4)
}

fn part2(input: &str) -> usize {
    find_unique_character_index(input, 14)
}

pub fn solve() {
    let input = fs::read_to_string("src/day_6_tuning_trouble/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 19);
    }
}
