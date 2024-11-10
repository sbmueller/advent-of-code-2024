use log::debug;

use std::fs::File;

use std::io;
use std::io::prelude::*;

fn read_from_stdin() -> std::io::Result<String> {
    let input = io::read_to_string(io::stdin().lock())?;
    debug!("Received input:\n\"\"\"\n{}\n\"\"\"", input);
    Ok(input)
}

fn write_output_file(output: &str, filename: &str) -> std::io::Result<()> {
    debug!("Output:\n\"\"\"\n{}\n\"\"\"", output);
    let mut file = File::create(filename)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn main() {
    env_logger::init();
    // Read input from standard input
    let input = read_from_stdin().expect("Could not read from stdin");

    // Do business logic here
    let output = input;

    // Output
    write_output_file(&output, "output.txt").expect("Could not write to output file");
}
