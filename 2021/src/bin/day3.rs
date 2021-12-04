#![feature(array_windows)]
#![feature(test)]
extern crate test;
use aoc2021::common::*;

const DAY: usize = 3;
type Parsed = Vec<Vec<bool>>;

fn read_input() -> String {
    read_file(3)
}

fn parse_input(raw: &str) -> Parsed {
    raw.lines().map(|line| line.bytes().map(|c| c == b'1').collect()).collect()
}

fn transpose(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<bool>>())
        .collect()
}

fn to_u32(slice: &Vec<bool>) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn part1(parsed: &Parsed) -> u32 {
    let transposed = transpose(parsed.clone());
    let gamma: Vec<bool> = transposed
        .iter()
        .map(|e| e.iter().filter(|&b| *b).count() >= parsed.len() / 2)
        .collect();
    let epsilon = gamma.iter().map(|b| !b).collect();
    to_u32(&gamma) * to_u32(&epsilon)
}

fn part2(parsed: &Parsed) -> u32 {
    let mut matching_gamma = parsed.clone();
    let mut matching_epsilon = parsed.clone();

    for i in 0..parsed[0].len() {
        if matching_gamma.len() > 1 {
            let more_ones = matching_gamma.iter().filter(|l| l[i]).count() * 2 >= matching_gamma.len();
            matching_gamma.retain(|n| n[i] == more_ones);
        }
        if matching_epsilon.len() > 1 {
            let more_zeros = matching_epsilon.iter().filter(|l| !l[i]).count() * 2 > matching_epsilon.len();
            matching_epsilon.retain(|n| n[i] == more_zeros);
        }
    }
    to_u32(&matching_gamma[0]) * to_u32(&matching_epsilon[0])
}

fn main() {
    let input = parse_input(&read_input());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::*;

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

    test!(part1() == 198);
    test!(part2() == 230);
    bench!(part1() == 3374136);
    bench!(part2() == 4432698);
    bench_input!(Vec::len => 1000);
}
