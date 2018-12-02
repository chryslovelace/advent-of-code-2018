use advent_of_code_2018::d2::parsed_input;

fn main() {
    for id1 in parsed_input() {
        for id2 in parsed_input() {
            let unmatched_pairs = id1
                .chars()
                .zip(id2.chars())
                .filter(|(c, d)| c != d)
                .collect::<Vec<_>>();
            if unmatched_pairs.len() == 1 {
                println!("{}", id1.replace(unmatched_pairs[0].0, ""));
                return;
            }
        }
    }
}
