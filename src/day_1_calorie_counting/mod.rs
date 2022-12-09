use std::fs;

pub fn part1(input: &str) -> u32 {
    let mut calorie_sums: Vec<u32> = vec![];
    let mut current_cals = 0;

    for line in input.lines() {
        if line == "" {
            calorie_sums.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }

    calorie_sums.push(current_cals);
    *calorie_sums.iter().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut calorie_sums: Vec<u32> = vec![];
    let mut current_cals = 0;

    for line in input.lines() {
        if line == "" {
            calorie_sums.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }

    calorie_sums.push(current_cals);
    calorie_sums.sort();
    calorie_sums.iter().rev().take(3).sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_1_calorie_counting/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 45000);
    }
}
