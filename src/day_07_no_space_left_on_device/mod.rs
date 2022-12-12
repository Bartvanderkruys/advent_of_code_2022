use std::fs;

enum Command {
    ChangeDirectoryRoot,
    ChangeDirectoryIn(String),
    ChangeDirectoryOut,
    List,
}

struct Node {
    name: String,
    size: Option<u32>,
    children: Vec<Node>,
}

impl Node {
    fn find_child_node<'a, 'b>(node: &'a mut Node, child_name: &'b String) -> &'a mut Node {
        node.children
            .iter_mut()
            .find(|child| child.name == *child_name)
            .unwrap()
    }

    fn find_node<'a, 'b>(&'a mut self, path: &'b Vec<String>) -> &'a mut Node {
        let mut path_iterator = path.iter();
        let mut ref_node: &mut Node = self;

        while let Some(name) = path_iterator.next() {
            ref_node = Node::find_child_node(ref_node, &name);
        }

        ref_node
    }

    fn fill_sizes(&mut self, dir_sizes: &mut Vec<u32>) {
        if self.children.len() > 0 {
            for node in self.children.iter_mut() {
                node.fill_sizes(dir_sizes)
            }

            dir_sizes.push(self.sum_children());
        }
    }

    fn sum_children(&mut self) -> u32 {
        self.size = Some(self.children.iter().map(|x| x.size.unwrap()).sum());
        self.size.unwrap()
    }
}

fn parse_command(line: &str) -> Command {
    match line.chars().nth(2).unwrap() {
        'c' => {
            let dir = &line[5..];

            match dir {
                ".." => return Command::ChangeDirectoryOut,
                "/" => return Command::ChangeDirectoryRoot,
                dir => return Command::ChangeDirectoryIn(dir.into()),
            }
        }
        'l' => Command::List,
        _ => panic!("unknown command"),
    }
}

fn parse_node(line: &str) -> Node {
    match line.chars().next().unwrap() {
        'd' => Node {
            name: line[4..].into(),
            size: None,
            children: vec![],
        },
        _ => {
            let line_parts: Vec<&str> = line.split(' ').collect();

            Node {
                name: line_parts.iter().nth(1).unwrap().to_string(),
                size: Some(line_parts.iter().nth(0).unwrap().parse::<u32>().unwrap()),
                children: vec![],
            }
        }
    }
}

fn construct_file_tree(input: &str) -> Node {
    let mut file_tree = Node {
        name: "root".to_string(),
        size: None,
        children: vec![],
    };

    let mut path: Vec<String> = vec![];

    input.lines().for_each(|line| {
        if line.chars().next().unwrap() == '$' {
            match parse_command(line) {
                Command::ChangeDirectoryRoot => path = vec![],
                Command::ChangeDirectoryOut => {
                    path.pop();
                }
                Command::ChangeDirectoryIn(dir) => {
                    path.push(dir);
                }
                Command::List => {}
            }
        } else {
            let active_node = file_tree.find_node(&path);
            active_node.children.push(parse_node(line))
        }
    });

    file_tree
}

fn get_dir_sizes(input: &str) -> Vec<u32> {
    let mut file_tree = construct_file_tree(input);
    let mut dir_sizes: Vec<u32> = vec![];

    file_tree.fill_sizes(&mut dir_sizes);

    dir_sizes
}

fn part1(input: &str) -> u32 {
    let dir_sizes = get_dir_sizes(input);

    dir_sizes.iter().filter(|x| **x <= 100000).sum()
}

fn part2(input: &str) -> u32 {
    let mut dir_sizes = get_dir_sizes(input);

    dir_sizes.sort();

    let unused_space = 70000000 - dir_sizes.last().unwrap();
    let space_to_free_up = 30000000 - unused_space;

    *dir_sizes.iter().find(|x| **x >= space_to_free_up).unwrap()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_07_no_space_left_on_device/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 24933642);
    }
}
