use std::fs;

fn get_priority_sum(contents: &String) -> u32 {
    contents
        .lines()
        .map(|line| {
            let (compartment_a, compartment_b) = line.split_at(line.chars().count() / 2);
            get_priority_of_dupe(find_duplicate_char(compartment_a, compartment_b))
        })
        .sum()
}

fn get_badge_priority_sum(contents: &String) -> u32 {
    let mut lines = contents.lines();

    let mut badge_priority_sum: u32 = 0;

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let badge = find_badge(a, b, c);

        badge_priority_sum += get_priority_of_dupe(badge);
    }

    badge_priority_sum
}

fn find_duplicate_char(a: &str, b: &str) -> char {
    for char in a.chars() {
        let maybe_index = b.find(char);

        if let Some(_) = maybe_index {
            return char;
        };
    }

    panic!("No dupe!")
}

fn get_priority_of_dupe(dupe: char) -> u32 {
    if dupe.is_uppercase() {
        dupe as u32 - 38
    } else {
        dupe as u32 - 96
    }
}

fn find_badge(a: &str, b: &str, c: &str) -> char {
    let index = a.find(|char| b.contains(char) && c.contains(char)).unwrap();
    a.chars().nth(index).unwrap()
}

pub fn solve() {
    let contents = fs::read_to_string("src/rucksack_reorganization/input.txt").unwrap();

    println!("Sum of priorities: {}", get_priority_sum(&contents));
    println!(
        "Sum of badge priorities: {}",
        get_badge_priority_sum(&contents)
    );
}
