#![feature(test)]
#![feature(iter_next_chunk)]
extern crate test;

use aoc2022::{boilerplate, common::*};

const DAY: usize = 04;
type Parsed = Vec<[usize; 4]>;

boilerplate! {
    TEST_INPUT == "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    tests: {
        part1: { TEST_INPUT => 2 },
        part2: { TEST_INPUT => 4 },
    },
    bench1 == 511,
    bench2 == 821,
    bench_parse: Vec::len => 1000,
}

fn parse_input(raw: &str) -> Parsed {
    raw.lines().map(|pair| pair.split(&[',', '-'][..]).next_chunk::<4>().unwrap().map(parse_num::<usize>)).collect()
}

fn part1(parsed: &Parsed) -> usize {
    parsed.iter().filter(|n| (n[0] >= n[2] && n[1] <= n[3]) || (n[2] >= n[0] && n[3] <= n[1]) ).count()
}

fn part2(parsed: &Parsed) -> usize {
    parsed.iter().filter(|n| !(n[3] < n[0] || n[2] > n[1])).count()
}
