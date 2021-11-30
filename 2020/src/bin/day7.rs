#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;

use petgraph::algo::has_path_connecting;
use petgraph::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\w+\s\w+)\sbags\scontain").unwrap();
    static ref CONTENT_RE: Regex = Regex::new(r"\s(\d)\s(\w+\s\w+)\sbags?[,.]").unwrap();
}

pub fn read_input() -> Graph<String, u32> {
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = Graph::<String, u32>::new();

    let input = std::fs::read_to_string("input/day7").unwrap();
    let lines = input.lines();
    for line in lines {
        let b1 = &BAG_RE.captures(line).unwrap()[1];
        if !nodes.contains_key(b1) {
            let n = graph.add_node(b1.to_string());
            nodes.insert(b1.to_string(), n);
        }

        for content in CONTENT_RE.captures_iter(line) {
            let b2 = &content[2];
            let weight = content[1].parse::<u32>().unwrap();

            if !nodes.contains_key(b2) {
                let node = graph.add_node(b2.to_string());
                nodes.insert(b2.to_string(), node);
            }

            graph.add_edge(nodes[b1], nodes[b2], weight);
        }
    }

    return graph;
}

pub fn part1(gr: &Graph<String, u32>) -> usize {
    let shiny_gold = gr.node_indices().find(|i| gr[*i] == "shiny gold").unwrap();
    return gr
        .node_indices()
        .collect::<Vec<NodeIndex>>()
        .par_iter()
        .map(|node| has_path_connecting(&gr, *node, shiny_gold, None) as usize)
        .sum::<usize>()
        - 1; //-1 Because the node counts itself as a neighbour
}

pub fn part2(gr: &Graph<String, u32>) -> u32 {
    let shiny_gold = gr.node_indices().find(|i| gr[*i] == "shiny gold").unwrap();
    return count_bags(gr, shiny_gold);
}

fn count_bags(gr: &Graph<String, u32>, node: NodeIndex) -> u32 {
    return if gr.neighbors_directed(node, Outgoing).next() == None {
        0
    } else {
        let mut bag_count = 0;
        let mut walker = gr.neighbors_directed(node, Outgoing).detach();
        while let Some((edge, node)) = walker.next(&gr) {
            bag_count += (gr[edge] * count_bags(gr, node)) + gr[edge];
        }
        bag_count
    };
}

fn main() {
    let input = read_input();
    println!("Day7 Part1: {}", part1(&input));
    println!("Day7 Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020::*;
    use paste::paste;
    use test::black_box;

    //bench!(read_input());
    bench!(part1() == 197);
    bench!(part2() == 85324);
}
