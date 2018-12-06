use advent_of_code_2018::d6::{coordinates, manhattan};

fn main() {
    let mut safe_region_size = 0;
    for i in 0..500 {
        for j in 0..500 {
            if coordinates()
                .map(|point| manhattan(point, (i, j)))
                .sum::<i32>()
                < 10000
            {
                safe_region_size += 1;
            }
        }
    }
    println!("{}", safe_region_size);
}
