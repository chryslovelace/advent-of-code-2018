use advent_of_code_2018::d4::sleepytime;

fn main() {
    let sleepytime = sleepytime();
    let (id, minutes) = sleepytime
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        .unwrap();
    let minute = (0..60).max_by_key(|&i| minutes[i as usize]).unwrap();
    println!("{}", id * minute);
}
