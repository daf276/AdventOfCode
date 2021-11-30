#![feature(test)]
extern crate test;

use itertools::Itertools;

pub fn read_input() -> Vec<usize> {
    return std::fs::read_to_string("input/day9")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
}

pub fn part1(input: &Vec<usize>, window_size: usize) -> Option<usize> {
    for i in window_size..input.len() {
        let is_sum = input[i - window_size..i]
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == input[i]);

        if !is_sum {
            return Some(input[i]);
        }
    }
    return None;
}

pub fn part2(input: &Vec<usize>, part1_solution: usize) -> Option<usize> {
    for i in 0..input.len() {
        let mut sum = input[i];
        for j in i + 1..input.len() {
            sum += input[j];
            if sum == part1_solution {
                return Some(input[i] + input[j]);
            }
            if sum > part1_solution {
                break;
            }
        }
    }
    return None;
}

fn main() {
    let input = read_input();
    let p1 = part1(&input, 25).unwrap();
    println!("Day9 Part1: {}", p1);
    println!("Day9 Part2: {}", part2(&input, p1).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    bench_input!(read_input());
    bench!(part1(25) == Some(36845998));
    bench!(part2(36845998) == Some(4830226));
}
