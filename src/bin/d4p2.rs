use advent_of_code_2018::d4::sleepytime;

fn main() {
    let sleepytime = sleepytime();
    let (id, minutes) = sleepytime.iter().max_by_key(|(_, minutes)| minutes.values().max()).unwrap();
    let (minute, _) = minutes.iter().max_by_key(|(_, &count)| count).unwrap();
    println!("{}", id * minute);
}