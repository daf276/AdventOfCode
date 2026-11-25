#!/bin/sh
set -eu
cd "$(dirname "$0")"

year=$(basename "$PWD")
number=${1:-$(date +%-d)}
number=${number#0}
day=$(printf %02d "$number")
solution=src/bin/day$day.rs
input=inputs/day$day

mkdir -p src/bin inputs

if [ -e "$solution" ]; then
    echo "$solution already exists, not touching it"
else
    cat > "$solution" <<EOF
#![feature(test)]
extern crate test;
use aoc$year::{boilerplate, common::*};

const DAY: usize = $number;
type Parsed = Vec<usize>;

boilerplate! {
    TEST_INPUT == ""
    for tests: {
        part1: { TEST_INPUT => 0 },
        part2: { TEST_INPUT => 0 },
    },
    bench1 == 0,
    bench2 == 0,
}

fn parse_input(raw: &str) -> Parsed {
    parse_nums(raw)
}

fn part1(parsed: &Parsed) -> usize {
    todo!()
}

fn part2(parsed: &Parsed) -> usize {
    todo!()
}
EOF
fi

if [ ! -s "$input" ]; then
    aocd "$number" "$year" > "$input" || { cat "$input" >&2; rm -f "$input"; exit 1; }
fi
