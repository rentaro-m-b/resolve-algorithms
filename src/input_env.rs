use std::env;

pub fn input_numbers() -> Vec<isize> {
    let args = env::args();
    let numbers = args.skip(1)
        .filter_map(|arg| arg.parse::<isize>().ok())
        .collect();

    numbers
}
