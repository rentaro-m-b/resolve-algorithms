use core::num;
use std::iter::Sum;
use std::process;

use resolve_algorithms::hello::hello;
use resolve_algorithms::input::get_input;
use resolve_algorithms::input_env::input_numbers;

// Do solution
fn main() {
    // let input = match get_input() {
    //     Ok(s) => s,
    //     Err(e) => {
    //         eprintln!("Error reading input: {}", e);
    //         process::exit(1);
    //     }
    // };
    
    // hello(input)
    let numbers = input_numbers();
    let total: isize = numbers.iter().sum();
    println!("{}", total);

}
