#![feature(array_windows)]
#![feature(test)]
extern crate test;
use aoc2021::common::*;
use std::iter::Sum;

const DAY: usize = 2;

#[derive(Debug, Copy, Clone)]
struct Vector3D(isize, isize, isize);
type Parsed = Vec<Vector3D>;

impl Vector3D{
    fn new(a:isize, b:isize, c:isize) -> Vector3D {
        Vector3D { 0:a, 1:b, 2:c }
    }
}

impl<'a> Sum<&'a Self> for Vector3D {
    fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self { 0: 0, 1: 0, 2: 0 }, |a, b| Self {
            0: a.0 + b.0,
            1: a.1 + b.1,
            2: a.2 + b.2,
        })
    }
}

impl From<&str> for Vector3D {
    fn from(item: &str) -> Self {
        let (dir, number) = item.split_once(' ').unwrap();
        let n = number.parse::<isize>().unwrap();
        match dir {
            "forward" => Vector3D::new(n,0,0),
            "down" => Vector3D::new(0,0,n),
            "up" => Vector3D::new(0,0,-n),
            _ => unreachable!(),
        }
    }
}

fn weird_calc(a : Vector3D, b: Vector3D) -> Vector3D {
    Vector3D {
        0: a.0+b.0,
        1: a.1+b.0*a.2,
        2: a.2+b.2
    }
}

fn read_input() -> String {
    read_file(2)
}

fn parse_input(raw: &str) -> Parsed {
    raw.lines().map(|n| Vector3D::from(n)).collect()
}

fn part1(parsed: &Parsed) -> isize {
    let sum: Vector3D = parsed.iter().sum();
    sum.0 * sum.2
}

fn part2(parsed: &Parsed) -> isize {
    let sum = parsed.iter().fold(Vector3D::new(0,0,0), |sum, val| weird_calc(sum, *val));
    sum.0 * sum.1
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

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    test!(part1() == 150);
    test!(part2() == 900);
    bench!(part1() == 1648020);
    bench!(part2() == 1759818555);
    bench_input!(Vec::len => 1000);
}