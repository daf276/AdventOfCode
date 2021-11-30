#![feature(test, map_first_last, bool_to_option)]
extern crate test;

use itertools::Itertools;
use std::collections::BTreeSet;

pub fn read_input() -> BTreeSet<u16> {
    return std::fs::read_to_string("input/day5")
        .unwrap()
        .lines()
        .map(|line| parse_line(line))
        .collect();
}

fn parse_line(line: &str) -> u16 {
    return line
        .bytes()
        .map(|symbol| symbol == 'B' as u8 || symbol == 'R' as u8)
        .fold(0, |acc, bit| acc << 1 | bit as u16);
}

pub fn part1(input: &BTreeSet<u16>) -> u16 {
    return *input.last().unwrap();
}

pub fn part2(input: &BTreeSet<u16>) -> u16 {
    return input
        .iter()
        .tuple_windows()
        .find_map(|(a, &b)| (a + 1 != b).then_some(a + 1))
        .unwrap();
}

fn main() {
    let input = read_input();
    println!("Day5 Part1: {}", part1(&input));
    println!("Day5 Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    //bench!(read_input());
    bench!(part1() == 989);
    bench!(part2() == 548);
}
