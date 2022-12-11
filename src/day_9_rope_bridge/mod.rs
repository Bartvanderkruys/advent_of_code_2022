use std::fs;

pub fn part1(input: &str) -> usize {
    0
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
