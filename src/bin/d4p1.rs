use advent_of_code_2018::d4::*;
use std::collections::HashMap;

fn main() {
    let shifts = parsed_input();
    let mut sleepytime = HashMap::new();
    for shift in shifts {
        let minutes = sleepytime.entry(shift.id).or_insert_with(HashMap::new);
        for nap in shift.sleeping {
            for minute in nap {
                *minutes.entry(minute).or_insert(0) += 1;
            }
        }
    }
    let (id, minutes) = sleepytime.iter().max_by_key(|(_, minutes)| minutes.values().sum::<u32>()).unwrap();
    let (minute, _) = minutes.iter().max_by_key(|(_, &count)| count).unwrap();
    println!("{}", id * minute);
}