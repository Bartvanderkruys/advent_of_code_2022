use std::fs;

use itertools::Itertools;

// first item is row, second is column
type TreePos = (usize, usize);

// first item is row, second is column, third is score
type TreeScore = (usize, usize, usize);

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

    fn get_tree_scores(trees: Vec<u32>) -> Vec<(usize, usize)> {
        let mut tree_scores: Vec<(usize, usize)> = vec![];

        trees[0..trees.len() - 1]
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                let maybe_closest_tree = trees[0..i].iter().rev().position(|y| y >= x);

                match maybe_closest_tree {
                    Some(closest) => tree_scores.push((i, closest + 1)),
                    None => tree_scores.push((i, i)),
                }
            });

        tree_scores
    }

    fn get_inner_visible_trees(&self) -> Vec<TreePos> {
        let mut visible: Vec<TreePos> = vec![];

        for row_index in 1..self.row_count - 1 {
            TreeGrid::get_visible_trees(self.get_row(row_index))
                .iter()
                .for_each(|x| visible.push((row_index, *x)));

            TreeGrid::get_visible_trees(self.get_row(row_index).into_iter().rev().collect())
                .iter()
                .for_each(|x| visible.push((row_index, self.row_count - x - 1)));
        }

        for column_index in 1..self.column_count - 1 {
            TreeGrid::get_visible_trees(self.get_column(column_index))
                .iter()
                .for_each(|x| visible.push((*x, column_index)));

            TreeGrid::get_visible_trees(self.get_column(column_index).into_iter().rev().collect())
                .iter()
                .for_each(|x| visible.push((self.column_count - x - 1, column_index)));
        }

        visible.into_iter().unique().collect_vec()
    }

    fn count_visible_trees(&self) -> usize {
        self.get_inner_visible_trees().iter().count() + self.count_trees_on_edge()
    }

    fn count_trees_on_edge(&self) -> usize {
        (self.row_count + self.column_count) * 2 - 4
    }

    fn get_best_tree(&self) -> usize {
        let mut tree_scores: Vec<TreeScore> = vec![];

        let mut add_to_tree_scores = |tree: TreeScore| {
            let maybe_index = tree_scores
                .iter()
                .position(|x| x.0 == tree.0 && x.1 == tree.1);

            match maybe_index {
                Some(i) => tree_scores[i] = (tree.0, tree.1, tree.2 * tree_scores[i].2),
                None => tree_scores.push(tree),
            }
        };

        for row_index in 0..self.row_count {
            TreeGrid::get_tree_scores(self.get_row(row_index))
                .into_iter()
                .for_each(|(column_index, score)| {
                    add_to_tree_scores((row_index, column_index, score))
                });

            TreeGrid::get_tree_scores(self.get_row(row_index).into_iter().rev().collect())
                .into_iter()
                .for_each(|(column_index, score)| {
                    add_to_tree_scores((row_index, self.column_count - column_index - 1, score))
                });
        }

        for column_index in 0..self.column_count {
            TreeGrid::get_tree_scores(self.get_column(column_index))
                .into_iter()
                .for_each(|(row_index, score)| {
                    add_to_tree_scores((row_index, column_index, score))
                });

            TreeGrid::get_tree_scores(self.get_column(column_index).into_iter().rev().collect())
                .into_iter()
                .for_each(|(row_index, score)| {
                    add_to_tree_scores((self.row_count - row_index - 1, column_index, score))
                });
        }

        tree_scores.sort_by(|a, b| b.2.cmp(&a.2));
        tree_scores.iter().nth(0).unwrap().2
    }
}

pub fn part1(input: &str) -> usize {
    let grid = TreeGrid::new(input);

    grid.count_visible_trees()
}

pub fn part2(input: &str) -> usize {
    let grid = TreeGrid::new(input);

    grid.get_best_tree()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_8_treetop_tree_house/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 8);
    }
}
