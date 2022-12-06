#![feature(test)]
#![feature(array_windows)]
extern crate test;

use std::iter::repeat;
use aoc2022::{boilerplate, common::*};

const DAY: usize = 06;

boilerplate! {
    TEST_INPUT == "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    tests: {
        part1: { TEST_INPUT => 7 },
        part2: { TEST_INPUT => 19 },
    },
    bench1 == 1142,
    bench2 == 2803,
    bench_parse: str::len => 4097,
}

fn parse_input(raw: &str) -> &str {
    raw
}

fn part1(parsed: &str) -> usize {
    parsed.as_bytes()
        .array_windows::<4>()
        .zip(4..)
        .find_map(|(arr, position)| (0..4).flat_map(|i| repeat(i).zip((i + 1)..4)).all(|(i, j)| arr[i] != arr[j]).then_some(position))
        .unwrap()
}

fn part2(parsed: &str) -> usize {
    parsed.as_bytes()
        .array_windows::<14>()
        .zip(14..)
        .find_map(|(arr, position)| (0..14).flat_map(|i| repeat(i).zip((i + 1)..14)).all(|(i, j)| arr[i] != arr[j]).then_some(position))
        .unwrap()
}
