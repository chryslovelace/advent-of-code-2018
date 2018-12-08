const INPUT: &str = include_str!("input.txt");

type Datum = u8;

fn input() -> impl Iterator<Item = Datum> {
    INPUT.split_ascii_whitespace().map(|s| s.parse().unwrap())
}

pub struct Node {
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

    pub fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().map(|&x| x as u32).sum::<u32>();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        sum
    }

    pub fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata.iter().map(|&x| x as u32).sum::<u32>()
        } else {
            self.metadata
                .iter()
                .filter(|&&i| i > 0 && i as usize <= self.children.len())
                .fold(0, |acc, &i| acc + self.children[i as usize - 1].value())
        }
    }
}

pub fn tree() -> Node {
    Node::build(&mut input())
}
