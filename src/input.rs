use std::io;

pub fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    Ok(input.trim().to_string())
}
