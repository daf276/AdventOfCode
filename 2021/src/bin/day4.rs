#![feature(array_windows)]
#![feature(test)]
extern crate test;
use aoc2021::common::*;
use std::collections::HashMap;

const DAY: usize = 4;

#[derive(Debug, Clone)]
struct Board {
    map: HashMap<u8, u32>,
    state: u32,
}

#[derive(Debug, Clone)]
struct BingoGame {
    called_numbers: Vec<u8>,
    boards:        Vec<Board>,
}

impl BingoGame {
    fn mark_numbers(&mut self, n: u8) {
        self.boards.iter_mut().for_each(|b| b.mark_number(n));
    }

    fn has_won(&self) -> Option<&Board> {
        self.boards.iter().find(|b| b.has_won())
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.boards.len()
    }
}

impl Board{
    fn has_won(&self) -> bool{
        let line = |i: u32| -> bool {(i..i+5).all(|n| ( self.state&(1<<n)) != 0)};
        let row = |i: u32| -> bool {(i..).step_by(5).take(5).all(|n| ( self.state&(1<<n)) != 0)};

        line(0) || line(5) || line(10) || line(15) || line(20) || row(0) || row(1) || row(2) || row(3) || row(4)
    }

    fn mark_number(&mut self, number: u8){
        let n = self.map.get(&number);
        if let Some(x) = n {
            self.state = self.state|(1<<x)
        }
    }
}

type Parsed = BingoGame;

fn read_input() -> String {
    read_file(4)
}


fn parse_input(raw: &str) -> Parsed {
    let (number_line, boards) = raw.split_once("\n\n").unwrap();
    let called_numbers = number_line.split(",").filter_map(|c| c.parse().ok()).collect();

    let boards:Vec<&str> = boards.split("\n\n").collect();
    let test:Vec<Vec<u8>> = boards.iter().map(|b| b.split_ascii_whitespace().filter_map(|n| n.parse().ok()).collect()).collect();
    let maps:Vec<Board>  = test.iter().map(|b| Board{ map: b.iter().enumerate().map(|(i,&e)| (e,i as u32)).collect(), state:0}).collect();

    BingoGame{ called_numbers, boards:maps }
}

fn part1(parsed: &Parsed) -> u32 {
    let mut game = parsed.to_owned();
    for n in game.called_numbers.clone() {
        game.mark_numbers(n);
        if let Some(board) = game.has_won() {
            return board.map.iter().map(|(&e,i)| if board.state&(1<<i) == 0 {e as u32} else {0}).sum::<u32>() * n as u32;
        }
    }
    unreachable!("I think someone has to win?")
}

fn part2(parsed: &Parsed) -> u32 {
    let mut game = parsed.to_owned();
    for n in game.called_numbers.clone() {
        game.mark_numbers(n);
        if game.boards.len() == 1 && game.boards[0].has_won() {
            return &game.boards[0].map.iter().map(|(&e,i)| if &game.boards[0].state&(1<<i) == 0 {e as u32} else {0}).sum::<u32>() * n as u32;
        }
        game.boards.retain(|b| !b.has_won());
    }
    unreachable!("I think someone has to win?")
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

    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    test!(part1() == 4512);
    test!(part2() == 1924);
    bench!(part1() == 63552);
    bench!(part2() == 9020);
    bench_input!(BingoGame::len => 100);
}