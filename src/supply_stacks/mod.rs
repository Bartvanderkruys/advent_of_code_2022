use std::fs;

type Stacks = Vec<Vec<char>>;

fn get_stacks(contents: &String) -> Stacks {
    let mut stacks: Stacks = vec![];

    let stack_base_line = contents
        .lines()
        .position(|x| x.chars().nth(1) == Some('1'))
        .unwrap();

    let number_of_stacks = contents
        .lines()
        .nth(stack_base_line)
        .unwrap()
        .chars()
        .filter(|x| x != &' ')
        .count();

    for i in 0..number_of_stacks {
        let column_index = 1 + (i * 4);

        stacks.push(
            contents
                .lines()
                .take(stack_base_line)
                .map(|line| line.chars().nth(column_index).unwrap())
                .filter(|x| x != &' ')
                .collect(),
        );
    }

    stacks
}

pub fn solve() {
    let contents = fs::read_to_string("src/supply_stacks/input.txt").unwrap();

    let stacks: Stacks = get_stacks(&contents);
}
