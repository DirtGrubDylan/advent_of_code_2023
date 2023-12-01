use std::fs::File;
use std::io::{BufRead, BufReader};

/// Takes a file and returns it's contents as a vector of strings.
///
/// # Errors
///
/// N/A
///
/// # Panics
///
/// If line couldn't be read and/or file doesn't exist.
pub fn to_string_vector(file_name: &str) -> Result<Vec<String>, String> {
    let file = BufReader::new(File::open(file_name).expect("File not found!"));

    Ok(file
        .lines()
        .map(|line| line.expect("The file is bad!"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_vector() {
        let expected = vec![
            String::from("1000"),
            String::from("2000"),
            String::from("3000"),
            String::new(),
            String::from("4000"),
            String::new(),
            String::from("5000"),
            String::from("6000"),
            String::new(),
            String::from("7000"),
            String::from("8000"),
            String::from("9000"),
            String::new(),
            String::from("10000"),
        ];

        let result = to_string_vector("test_inputs/example_file.txt").unwrap();

        assert_eq!(result, expected);
    }
}
