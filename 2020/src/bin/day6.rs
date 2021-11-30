#![feature(test)]
extern crate test;

use std::ops::{BitAnd, BitOr};

pub fn read_input() -> Vec<Vec<u128>> {
    return std::fs::read_to_string("input/day6")
        .unwrap()
        .replace("\r", "")
        .split("\n\n")
        .map(|grp| {
            grp.lines()
                .map(|person| person.bytes().fold(0, |acc, bit| acc ^ 1 << bit))
                .collect()
        })
        .collect();
}

pub fn part1(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(0, u128::bitor).count_ones())
        .sum();
}

pub fn part2(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(u128::MAX, u128::bitand).count_ones())
        .sum();
}

fn main() {
    let input = read_input();
    println!("Day6 Part1: {}", part1(&input));
    println!("Day6 Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    bench_input!(read_input());
    bench!(part1() == 6161);
    bench!(part2() == 2971);
}
