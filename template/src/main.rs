use log::debug;
use std::io;

mod business;

fn read_from_stdin() -> std::io::Result<String> {
    let input = io::read_to_string(io::stdin().lock())?;
    debug!("Received input:\n\"\"\"\n{}\n\"\"\"", input);
    Ok(input)
}

fn main() {
    env_logger::init();
    // Read input from standard input
    let input = read_from_stdin().expect("Could not read from stdin");

    // Do business logic here
    let output = business::business_logic(&input).expect("Business logic failed");

    // Output
    println!("{}", output);
}
