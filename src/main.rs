use std::process;

use resolve_algorithms::hello::hello;
use resolve_algorithms::input::get_input;

// Do solution
fn main() {
    let input = match get_input() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        }
    };
    
    hello(input)
}
