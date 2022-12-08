#![feature(test)]
extern crate test;

use std::iter::repeat;
use aoc2022::{boilerplate, common::*};

const DAY: usize = 08;

struct Forest {
    trees: Vec<Vec<usize>>,
    size: usize,
}

impl Forest {
    fn len(&self) -> usize {
        self.trees.len()
    }

    fn position_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.size - 1 || y == self.size - 1 {
            return true;
        }
        let height = self.trees[y][x];

        if self.trees[..y].iter().all(|line| line[x] < height) {
            return true;
        };
        if self.trees[y + 1..].iter().all(|line| line[x] < height) {
            return true;
        };
        if self.trees[y][..x].iter().all(|h| *h < height) {
            return true;
        };
        if self.trees[y][x + 1..].iter().all(|h| *h < height) {
            return true;
        };

        return false;
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.trees[y][x];

        let up = {
            let mut distance = self.trees[..y].iter().len();
            for (line, i) in self.trees[..y].iter().rev().zip(1..y + 1) {
                if line[x] >= height {
                    distance = i;
                    break;
                }
            }
            distance
        };

        let down = {
            let mut distance = self.trees[y + 1..].iter().len();
            for (line, i) in self.trees[y + 1..].iter().zip(1..self.size-y) {
                if line[x] >= height {
                    distance = i;
                    break;
                }
            }
            distance
        };

        let left = {
            let mut distance = self.trees[y][..x].iter().len();
            for (h, i) in self.trees[y][..x].iter().rev().zip(1..x + 1) {
                if *h >= height {
                    distance = i;
                    break;
                }
            }
            distance
        };

        let right = {
            let mut distance = self.trees[y][x + 1..].iter().len();
            for (h, i) in self.trees[y][x + 1..].iter().zip(1..self.size-x) {
                if *h >= height {
                    distance = i;
                    break;
                }
            }
            distance
        };

        return up * down * left * right;
    }
}
boilerplate! {
    TEST_INPUT == "30373
25512
65332
33549
35390",
    tests: {
        part1: { TEST_INPUT => 21 },
        part2: { TEST_INPUT => 8 },
    },
    bench1 == 1693,
    bench2 == 422059,
    bench_parse: Forest::len => 99,
}

fn parse_input(raw: &str) -> Forest {
    let trees = raw.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();
    let size = raw.lines().next().unwrap().len();
    Forest { trees, size }
}

fn part1(parsed: &Forest) -> usize {
    (0..parsed.size).flat_map(|i| repeat(i).zip(0..parsed.size)).map(|(i, j)| parsed.position_visible(i, j)).filter(|x| *x).count()
}

fn part2(parsed: &Forest) -> usize {
    (0..parsed.size).flat_map(|i| repeat(i).zip(0..parsed.size)).map(|(i, j)| parsed.scenic_score(i, j)).max().unwrap()
}
