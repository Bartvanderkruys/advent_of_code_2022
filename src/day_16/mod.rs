use std::fs;

pub fn part1(_: &str) -> u32 {
    0
}

pub fn solve() {
    let input = fs::read_to_string("src/*/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 0);
    }
}
