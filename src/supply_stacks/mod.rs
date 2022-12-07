use std::fs;

type Stack = Vec<char>;
type Instruction = (usize, usize, usize);

fn get_stack_base_line(contents: &String) -> usize {
    contents
        .lines()
        .position(|x| x.chars().nth(1) == Some('1'))
        .unwrap()
}

fn parse_instruction(string: &str) -> usize {
    string.parse::<usize>().unwrap()
}

fn get_instructions(contents: &String, stack_base_line: usize) -> Vec<Instruction> {
    contents
        .lines()
        .skip(stack_base_line + 2)
        .map(|line| {
            let mut split = line.split(" ");

            (
                parse_instruction(&split.nth(1).unwrap()),
                parse_instruction(&split.nth(1).unwrap()),
                parse_instruction(&split.nth(1).unwrap()),
            )
        })
        .collect()
}

fn get_stacks(contents: &String, stack_base_line: usize) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![];

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

fn drain_blocks(
    stacks: &mut Vec<Stack>,
    origin_index: usize,
    number_of_blocks: usize,
) -> Vec<char> {
    stacks
        .get_mut(origin_index)
        .unwrap()
        .drain(0..number_of_blocks)
        .collect()
}

fn execute_instructions(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        let blocks = drain_blocks(stacks, instruction.1 - 1, instruction.0);

        for block in blocks {
            stacks.get_mut(instruction.2 - 1).unwrap().insert(0, block);
        }
    }
}

pub fn solve() {
    let contents = fs::read_to_string("src/supply_stacks/input.txt").unwrap();

    let stack_base_line = get_stack_base_line(&contents);
    let mut stacks = get_stacks(&contents, stack_base_line);
    let instructions = get_instructions(&contents, stack_base_line);

    execute_instructions(&mut stacks, &instructions);

    println!(
        "{}",
        stacks
            .iter()
            .map(|stack| { stack.first().unwrap() })
            .collect::<String>()
    )
}
