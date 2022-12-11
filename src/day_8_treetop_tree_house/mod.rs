use std::fs;

use itertools::Itertools;

// first item is row, second is column
type TreePos = (usize, usize);

struct TreeGrid {
    row_count: usize,
    column_count: usize,
    trees: Vec<u32>,
}

impl TreeGrid {
    fn new(input: &str) -> TreeGrid {
        let lines: Vec<&str> = input.lines().collect();

        TreeGrid {
            column_count: lines.len(),
            row_count: lines.iter().nth(0).unwrap().chars().count(),
            trees: input
                .chars()
                .filter(|x| *x != '\n')
                .map(|x| x.to_digit(10).unwrap())
                .collect(),
        }
    }

    fn get_row(&self, row_index: usize) -> Vec<u32> {
        let start = row_index * self.column_count;
        let end = start + self.column_count;
        self.trees[start..end].to_vec()
    }

    fn get_column(&self, column_index: usize) -> Vec<u32> {
        self.trees
            .iter()
            .enumerate()
            .filter(|(i, _)| i % self.row_count == column_index)
            .map(|(_, x)| x.to_owned())
            .collect()
    }

    fn get_visible_trees(trees: Vec<u32>) -> Vec<usize> {
        let mut tallest_tree = trees.iter().next().unwrap();
        let mut visible_trees: Vec<usize> = vec![];

        trees[0..trees.len() - 1]
            .iter()
            .enumerate()
            .skip(1)
            .for_each(|(i, x)| {
                if x > tallest_tree {
                    visible_trees.push(i);
                    tallest_tree = x;
                }
            });

        visible_trees
    }

    fn get_horizontally_visible_trees(&self) -> Vec<TreePos> {
        let mut visible: Vec<TreePos> = vec![];

        for row_index in 1..self.row_count - 1 {
            TreeGrid::get_visible_trees(self.get_row(row_index))
                .iter()
                .for_each(|x| visible.push((row_index, *x)));

            TreeGrid::get_visible_trees(self.get_row(row_index).into_iter().rev().collect())
                .iter()
                .for_each(|x| visible.push((row_index, self.row_count - x - 1)));
        }

        visible.into_iter().unique().collect()
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = TreeGrid::new(input);

    let horizontally_visible = grid.get_horizontally_visible_trees();
    let vertically_visible = grid.get_vertically_visible_trees();

    // get unique trees sum
    // count border trees
    // add together

    0
}

pub fn solve() {
    let input = fs::read_to_string("src/day_8_treetop_tree_house/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 21);
    }
}
