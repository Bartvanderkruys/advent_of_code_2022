use std::fs;

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

pub fn solve() {
    let contents = fs::read_to_string("src/rucksack_reorganization/input.txt").unwrap();

    let priority_sum: u32 = contents
        .lines()
        .map(|line| {
            let (compartment_a, compartment_b) = line.split_at(line.chars().count() / 2);
            get_priority_of_dupe(find_duplicate_char(compartment_a, compartment_b))
        })
        .sum();

    println!("Sum of priorities: {}", priority_sum);
}
