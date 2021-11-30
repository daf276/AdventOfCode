#![feature(test)]
extern crate test;

use itertools::Itertools;

pub fn read_input() -> Vec<usize> {
    return std::fs::read_to_string("input/day1")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
}

pub fn part1(input: &Vec<usize>, target_number: usize) -> Option<usize> {
    return input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == target_number)
        .map(|(a, b)| a * b)
        .next();
}

pub fn part2(input: &Vec<usize>, target_number: usize) -> Option<usize> {
    let mut sorted = input.clone();
    sorted.sort();

    for i in 0..sorted.len() - 2 {
        let mut j = i + 1;
        let mut k = sorted.len() - 1;

        let a = sorted[i];
        while j < k {
            let b = sorted[j];
            let c = sorted[k];

            if a + b + c == target_number {
                return Some(a * b * c);
            } else if a + b + c > target_number {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
    return None;
}

fn main() {
    let input = read_input();
    println!("Day1 Part1: {}", part1(&input, 2020).unwrap());
    println!("Day1 Part2: {}", part2(&input, 2020).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    bench_input!(read_input());
    bench!(part1(2020) == Some(1018944));
    bench!(part2(2020) == Some(8446464));
}
