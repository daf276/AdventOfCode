#![feature(test)]
extern crate test;
use aoc2021::common::*;

const DAY: usize = 5;
type Parsed = Vec<Vec<usize>>;

fn read_input() -> String {
    read_file(5)
}

fn parse_input(raw: &str) -> Parsed {
    raw.lines()
        .filter_map(|line| line.split_once(" -> "))
        .filter_map(|(c1, c2)| c1.split_once(',').zip(c2.split_once(',')))
        .map(|((x1, y1), (x2, y2))| vec![x1.parse().unwrap(), y1.parse().unwrap(), x2.parse().unwrap(), y2.parse().unwrap()])
        .collect()
}

fn check_map(start_end: &Vec<Vec<usize>>) -> usize {
    let max = *start_end.iter().flatten().max().unwrap() + 1;
    let mut map: Vec<Vec<u8>> = vec![vec![0; max]; max];

    for line in start_end {
        let mut x = line[0];
        let mut y = line[1];

        loop {
            map[x][y] += 1;
            if x == line[2] && y == line[3] {
                break;
            }
            if line[0] < line[2] {
                x += 1;
            } else if line[0] > line[2] {
                x -= 1;
            }
            if line[1] < line[3] {
                y += 1;
            } else if line[1] > line[3] {
                y -= 1;
            }
        }
    }

    map.into_iter().flatten().filter(|&e| e > 1).count()
}

fn part1(parsed: &Parsed) -> usize {
    let filtered = parsed
        .clone()
        .into_iter()
        .filter(|line| line[0] == line[2] || line[1] == line[3])
        .collect();
    check_map(&filtered)
}

fn part2(parsed: &Parsed) -> usize {
    check_map(&parsed)
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

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    test!(part1() == 5);
    test!(part2() == 12);
    bench!(part1() == 7297);
    bench!(part2() == 21038);
    bench_input!(Vec::len => 500);
}
