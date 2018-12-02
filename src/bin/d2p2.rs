use advent_of_code_2018::{d2::parsed_input, util::IteratorExt};

fn main() {
    for id1 in parsed_input() {
        for id2 in parsed_input() {
            let unmatched_pair = id1
                .chars()
                .zip(id2.chars())
                .filter(|(c, d)| c != d)
                .single();
            if let Some((c, _)) = unmatched_pair {
                println!("{}", id1.replace(c, ""));
                return;
            }
        }
    }
}
