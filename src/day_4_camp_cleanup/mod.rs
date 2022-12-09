use std::{fs, ops::RangeInclusive};

fn convert_line_to_ranges(line: &str) -> Vec<RangeInclusive<i32>> {
    line.split(',')
        .map(|split| {
            let mut range_split = split.split('-');

            range_split.next().unwrap().parse::<i32>().unwrap()
                ..=range_split.next().unwrap().parse::<i32>().unwrap()
        })
        .collect()
}

fn has_complete_overlap<'a>(line: &'a &str) -> bool {
    let ranges = convert_line_to_ranges(line);

    let range_a = ranges.first().unwrap();
    let range_b = ranges.last().unwrap();

    if range_a.contains(&range_b.start()) && range_a.contains(&range_b.end()) {
        return true;
    }

    if range_b.contains(&range_a.start()) && range_b.contains(&range_a.end()) {
        return true;
    }

    false
}

fn has_partial_overlap<'a>(line: &'a &str) -> bool {
    let ranges = convert_line_to_ranges(line);

    let range_a = ranges.first().unwrap();
    let range_b = ranges.last().unwrap();

    if range_a.contains(&range_b.start()) || range_a.contains(&range_b.end()) {
        return true;
    }

    if range_b.contains(&range_a.start()) || range_b.contains(&range_a.end()) {
        return true;
    }

    false
}

fn part1(input: &str) -> usize {
    input.lines().filter(has_complete_overlap).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(has_partial_overlap).count()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_4_camp_cleanup/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
