pub fn business_logic(input: &str) -> Result<String, String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::path::Path;

    #[test]
    fn test_business_logic_with_file_io() -> io::Result<()> {
        // Read the contents of `input.txt`
        let input_path = Path::new("input.txt");
        let input_data = fs::read_to_string(input_path)?;

        // Run the business logic function on the input data
        let output = business_logic(&input_data).expect("Business logic failed");

        // Read the expected output from `output.txt`
        let expected_output_path = Path::new("output.txt");
        let expected_output = fs::read_to_string(expected_output_path)?;

        // Compare the actual output to the expected output
        assert_eq!(
            output, expected_output,
            "Output did not match expected output"
        );

        Ok(())
    }
}
