use std::fs;

pub fn get_cycles(input: &str) -> Vec<i32> {
    let mut cycle_states: Vec<i32> = vec![];
    let mut current_state: i32 = 1;

    input
        .lines()
        .for_each(|line| match line.chars().next().unwrap() {
            'a' => {
                cycle_states.push(current_state);
                cycle_states.push(current_state);
                current_state += line[5..].parse::<i32>().unwrap();
            }
            'n' => {
                cycle_states.push(current_state);
            }
            _ => panic!("unknown command"),
        });

    cycle_states
}

pub fn part1(input: &str) -> i32 {
    let cycle_states = get_cycles(input);

    let relevant_cycles = [
        cycle_states.iter().nth(19).unwrap() * 20,
        cycle_states.iter().nth(59).unwrap() * 60,
        cycle_states.iter().nth(99).unwrap() * 100,
        cycle_states.iter().nth(139).unwrap() * 140,
        cycle_states.iter().nth(179).unwrap() * 180,
        cycle_states.iter().nth(219).unwrap() * 220,
    ];

    relevant_cycles.iter().sum()
}

pub fn part2(input: &str) -> String {
    let cycle_states = get_cycles(input);
    let mut pixels = String::from("");

    cycle_states.iter().enumerate().for_each(|(i, x)| {
        if i > 1 && i % 40 == 0 {
            pixels += "\n";
        }

        match (x - 1..=x + 1).find(|y| *y as usize == i % 40) {
            Some(_) => pixels += "#",
            None => pixels += ".",
        }
    });

    pixels
}

pub fn solve() {
    let input = fs::read_to_string("src/day_10_cathode_ray_tube/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
