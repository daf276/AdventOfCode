#![feature(test)]
extern crate test;

use itertools::Itertools;

#[derive(Copy, Clone)]
pub enum Instruction {
    NOP,
    ACC,
    JMP,
}

#[derive(Copy, Clone)]
pub struct OP {
    instruction: Instruction,
    value: isize,
}

pub fn read_input() -> Vec<OP> {
    return std::fs::read_to_string("input/day8")
        .unwrap()
        .lines()
        .map(|line| parse_line(line))
        .collect();
}

fn parse_line(line: &str) -> OP {
    let (instruction_str, value_str) = line.split(" ").collect_tuple().unwrap();
    let instruction = match instruction_str {
        "jmp" => Instruction::JMP,
        "nop" => Instruction::NOP,
        "acc" => Instruction::ACC,
        _ => unreachable!(),
    };
    let value = value_str.parse::<isize>().unwrap();

    return OP { instruction, value };
}

pub fn part1(input: &Vec<OP>) -> isize {
    let (acc, _) = emulate(input);
    return acc;
}

pub fn part2(
    commands: &Vec<OP>,
    seen: &mut Vec<bool>,
    mut index: isize,
    mut acc: isize,
    changed: bool,
) -> Option<isize> {
    loop {
        if index as usize >= commands.len() {
            return Some(acc);
        }
        if changed && seen[index as usize] {
            return None;
        }
        seen[index as usize] = true;
        match commands[index as usize].instruction {
            Instruction::NOP => {
                if !changed && index > -commands[index as usize].value {
                    if let Some(n) = part2(
                        commands,
                        seen,
                        index + commands[index as usize].value,
                        acc,
                        true,
                    ) {
                        return Some(n);
                    }
                }
                index += 1;
            }
            Instruction::ACC => {
                acc += commands[index as usize].value;
                index += 1
            }
            Instruction::JMP => {
                // is there no way to have regular if and if let in one statement?
                if !changed {
                    if let Some(n) = part2(commands, seen, index + 1, acc, true) {
                        return Some(n);
                    }
                }
                index += commands[index as usize].value;
            }
        }
    }
}

//Returns the accumulator and whether the program terminated normally
fn emulate(ops: &Vec<OP>) -> (isize, bool) {
    let mut reached = vec![false; ops.len()];
    let mut acc = 0;
    let mut pc = 0;

    while pc < ops.len() as isize {
        if pc < 0 {
            pc = 0;
        }
        if reached[pc as usize] {
            return (acc, false);
        } else {
            reached[pc as usize] = true;
            match ops[pc as usize].instruction {
                Instruction::NOP => (),
                Instruction::JMP => pc += ops[pc as usize].value - 1,
                Instruction::ACC => acc += ops[pc as usize].value,
            }
            pc += 1;
        }
    }

    return (acc, true);
}

fn main() {
    let input = read_input();
    println!("Day6 Part1: {}", part1(&input));
    println!(
        "Day6 Part2: {}",
        part2(&input, &mut vec![false; input.len()], 0, 0, false).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    //bench!(read_input());
    bench!(part1() == 1451);
    // bench!(part2() == 1160);
}
