use advent_of_code_2018::d7::steps;

fn main() {
    let steps = steps();
    let mut order = String::new();
    while let Some(next_step) = steps
        .iter()
        .find(|step| step.is_ready(&order, &order))
        .map(|step| step.name)
    {
        order.push_str(next_step);
    }
    println!("{}", order);
}
