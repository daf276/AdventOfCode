#![feature(test)]
extern crate test;
use aoc2021::common::*;

const DAY: usize = 6;
type Parsed = Vec<usize>;

fn read_input() -> String {
    read_file(6)
}

fn parse_input(raw: &str) -> Parsed {
    raw.trim().split(",").filter_map(|n| n.parse().ok()).collect()
}

fn part1(parsed: &Parsed) -> usize {
    let mut fish = vec![0;9];
    parsed.iter().for_each(|&e| fish[e]+=1);
    for _ in 0..80{
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    return fish.into_iter().sum()
}

fn part2(parsed: &Parsed) -> usize {
    let mut fish = vec![0;9];
    parsed.iter().for_each(|&e| fish[e]+=1);
    for _ in 0..256{
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    return fish.into_iter().sum()
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

    const TEST_INPUT: &str = "3,4,3,1,2";

    test!(part1() == 5934);
    test!(part2() == 26984457539);
    bench!(part1() == 394994);
    bench!(part2() == 1765974267455);
    bench_input!(Vec::len => 300);
}
