use rayon::prelude::*;

fn main() {
    let starting_number = 0;
    let until = 99_999_999;

    run_single_threaded(starting_number, until);
    run_multi_threaded(starting_number, until);
}

fn run_single_threaded(starting_number: u128, until: u128) {
    let instant = std::time::Instant::now();

    let max = (starting_number..=until).into_iter().map(count_steps).max().unwrap_or(0);

    println!(
        "Single threaded ->\tmax: {}\ttime: {}ms",
        max,
        instant.elapsed().as_millis()
    );
}

fn run_multi_threaded(starting_number: u128, until: u128) {
    let instant = std::time::Instant::now();

    let max = (starting_number..=until)
        .into_par_iter()
        .map(count_steps)
        .max()
        .unwrap_or(0);

    println!(
        "Multi threaded ->\tmax: {}\ttime: {}ms",
        max,
        instant.elapsed().as_millis()
    );
}

fn count_steps(starting_number: u128) -> u16 {
    let mut value = starting_number;
    let mut count: u16 = 0;

    while value > 1 {
        if value % 2 == 0 {
            value /= 2;
        } else {
            value = value * 3 + 1;
        }

        count += 1;
    }

    count
}
