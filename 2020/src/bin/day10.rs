#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

pub fn read_input() -> Vec<usize> {
    let mut input = std::fs::read_to_string("input/day10")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();
    input.push(0);
    input.sort_unstable();
    input.push(*input.last().unwrap() + 3);
    return input;
}

pub fn part1(input: &Vec<usize>) -> usize {
    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in input.iter().tuple_windows() {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    return ones * threes;
}

pub fn part2(input: &Vec<usize>) -> usize {
    let max = *input.last().unwrap();
    let mut permutations = vec![0; max + 1];
    permutations[0] = 1;
    for &i in input.iter().skip(1) {
        permutations[i] = (1..=3)
            .map(|j| if i >= j { permutations[i - j] } else { 0 })
            .sum();
    }
    return permutations[max];
}

fn main() {
    let input = read_input();
    println!("Day10 Part1: {}", part1(&input));
    println!("Day10 Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    bench_input!(read_input());
    bench!(part1() == 2664);
    bench!(part2() == 148098383347712);
}
