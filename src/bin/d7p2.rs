use advent_of_code_2018::d7::steps;

#[derive(Clone, Default)]
struct Worker {
    step: Option<&'static str>,
    elapsed: u8,
}

fn main() {
    let steps = steps();
    let mut worked_on = String::new();
    let mut completed = String::new();
    let mut time = 0;
    let mut workers = vec![Worker::default(); 5];

    while completed.len() < 26 {
        for worker in &mut workers {
            if let Some(step) = worker.step {
                if worker.elapsed >= time_needed(step) {
                    completed.push_str(step);
                    worker.step = None;
                    worker.elapsed = 0;
                }
            }
        }
        for worker in &mut workers {
            if worker.step.is_none() {
                worker.step = steps
                    .iter()
                    .find(|step| step.is_ready(&worked_on, &completed))
                    .map(|step| step.name);
            }
            if let Some(step) = worker.step {
                if !worked_on.contains(step) {
                    worked_on.push_str(step);
                }
                worker.elapsed += 1;
            }
        }
        time += 1;
    }
    println!("{}", time - 1);
}

fn time_needed(step: &str) -> u8 {
    step.as_bytes()[0] - b'A' + 61
}
