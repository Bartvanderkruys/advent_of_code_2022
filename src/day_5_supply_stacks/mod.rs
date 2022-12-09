use std::fs;

type Stack = Vec<char>;
type Instruction = (usize, usize, usize);

fn get_stack_base_line(contents: &str) -> usize {
    contents
        .lines()
        .position(|x| x.chars().nth(1) == Some('1'))
        .unwrap()
}

fn parse_instruction(string: &str) -> usize {
    string.parse::<usize>().unwrap()
}

fn get_instructions(contents: &str, stack_base_line: usize) -> Vec<Instruction> {
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

fn get_stacks(contents: &str, stack_base_line: usize) -> Vec<Stack> {
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

fn execute_instructions_9000(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        let blocks = drain_blocks(stacks, instruction.1 - 1, instruction.0);

        for block in blocks {
            stacks.get_mut(instruction.2 - 1).unwrap().insert(0, block);
        }
    }
}

fn execute_instructions_9001(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        let blocks = drain_blocks(stacks, instruction.1 - 1, instruction.0);

        for block in blocks.iter().rev() {
            stacks.get_mut(instruction.2 - 1).unwrap().insert(0, *block);
        }
    }
}

fn part1(input: &str) -> String {
    let stack_base_line = get_stack_base_line(input);
    let mut stacks = get_stacks(input, stack_base_line);
    let instructions = get_instructions(input, stack_base_line);

    execute_instructions_9000(&mut stacks, &instructions);

    stacks
        .iter()
        .map(|stack| stack.first().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let stack_base_line = get_stack_base_line(input);
    let mut stacks = get_stacks(input, stack_base_line);
    let instructions = get_instructions(input, stack_base_line);

    execute_instructions_9001(&mut stacks, &instructions);

    stacks
        .iter()
        .map(|stack| stack.first().unwrap())
        .collect::<String>()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_5_supply_stacks/input.txt").unwrap();

    println!("Part 2: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
