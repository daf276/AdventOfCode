#![feature(array_windows)]
#![feature(test)]
extern crate test;
use aoc2021::common::*;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Vector3D(isize, isize, isize);
type Parsed = Vec<Vector3D>;

impl Vector3D{
    fn new(a:isize, b:isize, c:isize) -> Vector3D {
        Vector3D { 0:a, 1:b, 2:c }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            0: self.0+other.0,
            1: self.1+other.1,
            2: self.2+other.2
        }
    }
}

impl From<&str> for Vector3D {
    fn from(item: &str) -> Self {
        let mut split = item.split_whitespace();
        let direction = split.next().unwrap();
        let n = split.next().unwrap().parse().unwrap();
        match direction {
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
    let sum = parsed.iter().fold(Vector3D::new(0,0,0), |sum, val| sum + *val);
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
    use paste::paste;
    use test::black_box;

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
    bench_input!(len == 1000);
}