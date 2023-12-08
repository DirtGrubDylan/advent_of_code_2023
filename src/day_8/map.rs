use std::collections::HashMap;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(input: char) -> Self {
        match input {
            'L' | 'l' => Instruction::Left,
            'R' | 'r' => Instruction::Right,
            _ => panic!("Cannot convert '{input}' to Instruction!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    network: HashMap<String, Node>,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    label: String,
    left_label: String,
    right_label: String,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_from_str() {
        let input = "AAA = (BBB, CCC)";

        let expected = Node {
            label: "AAA".to_string(),
            left_label: "BBB".to_string(),
            right_label: "CCC".to_string(),
        };

        let result: Node = input.parse().unwrap();

        assert_eq!(result, expected);
    }
}
