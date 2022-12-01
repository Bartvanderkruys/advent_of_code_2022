use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("src/calorie_counting/input.txt").unwrap();

    let mut highest_cals = 0;
    let mut current_cals = 0;

    for line in contents.lines() {
        if line == "" {
            if current_cals > highest_cals {
                highest_cals = current_cals;
            }
            current_cals = 0;
        } else {
            current_cals += line.parse::<i32>().unwrap();
        }
    }

    println!("Most caleries: {}", highest_cals)
}
