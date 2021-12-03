#![feature(array_windows)]
#![feature(test)]
extern crate test;
use aoc2021::common::*;
use bitvec::prelude::*;

type Parsed = Vec<BitVec>;

fn read_input() -> String {
    read_file(3)
}

fn parse_input(raw: &str) -> Parsed {
    raw.lines().map(|line| BitVec::from_element(usize::from_str_radix(line, 2).unwrap())).collect()
}

fn transpose(v:&Vec<BitVec>) -> Vec<BitVec> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<BitVec>()
        })
        .collect()
}

fn to_u32(slice: Vec<bool>) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

fn part1(parsed: &Parsed, length: usize) -> u32 {
    let transposed = transpose(parsed)[0..length].to_vec();
    let gamma:Vec<bool> = transposed.iter().map(|e| e.count_ones() >= transposed[0].len()/2).collect();
    let epsilon = gamma.iter().map(|b| !b).collect();
    to_u32(gamma) * to_u32(epsilon)
}

fn part2(parsed: &Parsed) -> usize {
    1
}

fn main() {
    let input = parse_input(&read_input());
    println!("Part 1: {}", part1(&input, 12));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::*;
    use paste::paste;
    use test::black_box;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    test!(part1(5) == 198);
    test!(part2() == 5);
    bench!(part1(12) == 3374136);
    bench!(part2() == 1728);
    bench_input!(len == 1000);
}