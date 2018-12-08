#![feature(split_ascii_whitespace)]

use lazy_static::lazy_static;

fn main() {
    part1();
    part2();
}

lazy_static! {
    static ref TREE: Node = {
        let mut input = include_str!("input.txt")
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap());
        Node::build(&mut input)
    };
}

fn part1() {
    println!("{}", TREE.sum_metadata());
}

fn part2() {
    println!("{}", TREE.value());
}

type Datum = u8;

struct Node {
    children: Vec<Node>,
    metadata: Vec<Datum>,
}

impl Node {
    fn build<I: Iterator<Item = Datum>>(iter: &mut I) -> Self {
        let num_children = iter.next().unwrap();
        let num_metadata = iter.next().unwrap();
        Node {
            children: (0..num_children).map(|_| Node::build(iter)).collect(),
            metadata: (0..num_metadata).map(|_| iter.next().unwrap()).collect(),
        }
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().map(|&x| u32::from(x)).sum::<u32>();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        sum
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().map(|&x| u32::from(x)).sum::<u32>()
        } else {
            self.metadata
                .iter()
                .filter(|&&i| i > 0 && i as usize <= self.children.len())
                .fold(0, |acc, &i| acc + self.children[i as usize - 1].value())
        }
    }
}
