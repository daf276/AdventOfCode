#![feature(test, slice_take, get_many_mut)]
extern crate test;

use itertools::Itertools;
use lazy_static::lazy_static;
use aoc2022::{boilerplate, common::*};
use regex::Regex;

const DAY: usize = 05;

struct Haven {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Haven {
    fn len(&self) -> usize {
        self.moves.len()
    }
}

lazy_static! {
   static ref NL: Regex = Regex::new(r"\r\n\r\n|\n\n").unwrap();
}

boilerplate! {
    TEST_INPUT == "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    tests: {
        part1: { TEST_INPUT => "CMZ" },
        part2: { TEST_INPUT => "MCD" },
    },
    bench1 == "MQTPGLLDN",
    bench2 == "LVZPSTTCZ",
    bench_parse: Haven::len => 502,
}

fn parse_input(raw: &str) -> Haven {
    let split: Vec<String> = NL.split(raw).map(|s| s.to_owned()).collect();

    let num_of_stacks: usize = split[0].lines().last().unwrap().split("   ").last().unwrap().trim().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_of_stacks];

    for line in split[0].lines().rev() {
        if line == split[0].lines().last().unwrap() {
            continue;
        }

        let cargo = line.chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .collect::<Vec<String>>();

        for (stack_num, c) in cargo.iter().enumerate() {
            let x = c.chars().nth(1).unwrap();
            if x != ' ' {
                stacks[stack_num].push(x)
            }
        }
    }

    let moves = split[1].lines().map(|line| get_moves(line.split_whitespace().collect())).collect();
    Haven { stacks, moves }
}

fn get_moves(split: Vec<&str>) -> (usize, usize, usize) {
    (split[1].parse().unwrap(), split[3].parse().unwrap(), split[5].parse().unwrap())
}

fn part1(parsed: &Haven) -> String {
    let mut stacks = parsed.stacks.to_owned();

    for (num, stack_from, stack_to) in &parsed.moves {
        let start_index = stacks[*stack_from - 1].len() - *num;
        let [src, dst] = stacks.get_many_mut([*stack_from - 1, *stack_to - 1]).unwrap();
        dst.extend(src.drain(start_index..).rev());
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(parsed: &Haven) -> String {
    let mut stacks = parsed.stacks.to_owned();

    for (num, stack_from, stack_to) in &parsed.moves {
        let start_index = stacks[*stack_from - 1].len() - *num;
        let [src, dst] = stacks.get_many_mut([*stack_from - 1, *stack_to - 1]).unwrap();
        dst.extend(src.drain(start_index..));
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
