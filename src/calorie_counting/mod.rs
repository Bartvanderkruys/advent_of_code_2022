use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("src/calorie_counting/input.txt").unwrap();

    let mut calorie_sums: Vec<i32> = vec![];
    let mut current_cals = 0;

    for line in contents.lines() {
        if line == "" {
            calorie_sums.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<i32>().unwrap();
        }
    }

    calorie_sums.sort();
    calorie_sums.reverse();
    calorie_sums.truncate(3);

    println!("Elf highscores: {:?}", calorie_sums);
    println!("Total: {}", calorie_sums.iter().sum::<i32>());
}
