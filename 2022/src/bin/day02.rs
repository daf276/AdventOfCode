#![feature(test)]
extern crate test;

use aoc2022::{boilerplate, common::*};

const DAY: usize = 02;

boilerplate! {
    TEST_INPUT == "A Y
B X
C Z",
    tests: {
        part1: { TEST_INPUT => 15 },
        part2: { TEST_INPUT => 12 },
    },
    bench1 == 13675,
    bench2 == 14184,
    bench_parse: str::len => 12500,
}

fn parse_input(raw: &str) -> &str {
    raw
}

fn part1(parsed: &str) -> usize {
    parsed.lines().map(|round| points_1(round)).sum()
}

fn points_1(round: &str) -> usize {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => unreachable!()
    }
}

fn part2(parsed: &str) -> usize {
    parsed.lines().map(|round| points_2(round)).sum()
}

fn points_2(round: &str) -> usize {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => unreachable!()
    }
}