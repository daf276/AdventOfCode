#![feature(test)]
extern crate test;

use aoc2022::{boilerplate, common::*};
use std::collections::BinaryHeap;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

const DAY: usize = 01;

type Parsed = Vec<usize>;

lazy_static! {
   static ref NL: Regex = Regex::new(r"\r\n\r\n|\n\n").unwrap();
}

boilerplate! {
    TEST_INPUT == "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    tests: {
        part1: { TEST_INPUT => 24000 },
        part2: { TEST_INPUT => 45000 },
    },
    bench1 == 66306,
    bench2 == 195292,
    bench_parse: Vec::len => 242,
}

fn parse_input(raw: &str) -> Parsed {
    NL.split(raw).map(|elf| elf.lines().map(parse_num::<usize>).sum()).collect()
}

fn part1(parsed: &Parsed) -> usize {
    *parsed.iter().max().unwrap()
}

fn part2(parsed: &Parsed) -> usize {
    let mut elf_heap = parsed.iter().copied().collect::<BinaryHeap<usize>>();
    (0..3).map(|_| elf_heap.pop().unwrap()).sum()
}
