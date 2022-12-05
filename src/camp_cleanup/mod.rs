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

fn has_overlap<'a>(line: &'a &str) -> bool {
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

pub fn solve() {
    let contents = fs::read_to_string("src/camp_cleanup/input.txt").unwrap();

    let solution = contents.lines().filter(has_overlap).count();

    println!("Number of complete overlaps: {}", solution);
}