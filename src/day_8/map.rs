use std::collections::{HashMap, VecDeque};
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    instructions: VecDeque<Instruction>,
    network: HashMap<String, Node>,
}

impl Map {
    pub fn new(input: &[String]) -> Self {
        let instructions: VecDeque<Instruction> = input[0].chars().map(Instruction::from).collect();

        let network = input
            .iter()
            .skip(2)
            .filter_map(|row| row.parse::<Node>().ok())
            .map(|node| (node.label.clone(), node))
            .collect();

        Map {
            instructions,
            network,
        }
    }

    pub fn steps_between(&self, start_label: &str, end_label: &str) -> u32 {
        let mut instructions_copy = self.instructions.clone();
        let mut current_node = self.network.get(start_label);
        let mut steps: u32 = 0;

        while current_node.map_or("", |node| node.label.as_str()) != end_label {
            steps += 1;

            current_node = instructions_copy
                .front()
                .and_then(|instruction| self.get_child_node(current_node, *instruction));

            instructions_copy.rotate_left(1);
        }

        steps
    }

    fn get_child_node(&self, node_opt: Option<&Node>, instruction: Instruction) -> Option<&Node> {
        node_opt
            .map(|node| node.get_child_label_from(instruction))
            .and_then(|child_label| self.network.get(&child_label))
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub label: String,
    left_label: String,
    right_label: String,
}

impl Node {
    pub fn get_child_label_from(&self, instruction: Instruction) -> String {
        match instruction {
            Instruction::Left => self.left_label.clone(),
            Instruction::Right => self.right_label.clone(),
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let label_opt = input.get(..3).map(str::to_string);
        let left_label_opt = input.get(7..10).map(str::to_string);
        let right_label_opt = input.get(12..15).map(str::to_string);

        match (label_opt, left_label_opt, right_label_opt) {
            (Some(label), Some(left_label), Some(right_label)) => Ok(Node {
                label,
                left_label,
                right_label,
            }),
            _ => Err(format!("{input} cannot be parsed to a Node!")),
        }
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

    #[test]
    fn test_instruction_from_char() {
        let input = "LLR";

        let expected = vec![Instruction::Left, Instruction::Left, Instruction::Right];

        let result: Vec<Instruction> = input.chars().map(Instruction::from).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_new() {
        let inputs = [
            "LLR".to_string(),
            String::new(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];

        let instructions =
            VecDeque::from([Instruction::Left, Instruction::Left, Instruction::Right]);
        let network = HashMap::from([
            (
                "AAA".to_string(),
                Node {
                    label: "AAA".to_string(),
                    left_label: "BBB".to_string(),
                    right_label: "BBB".to_string(),
                },
            ),
            (
                "BBB".to_string(),
                Node {
                    label: "BBB".to_string(),
                    left_label: "AAA".to_string(),
                    right_label: "ZZZ".to_string(),
                },
            ),
            (
                "ZZZ".to_string(),
                Node {
                    label: "ZZZ".to_string(),
                    left_label: "ZZZ".to_string(),
                    right_label: "ZZZ".to_string(),
                },
            ),
        ]);

        let expected = Map {
            instructions,
            network,
        };

        let result = Map::new(&inputs);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_node_get_child_label_from() {
        let node: Node = "AAA = (BBB, CCC)".parse().unwrap();

        let expected_left = "BBB".to_string();
        let expected_right = "CCC".to_string();

        let result_left = node.get_child_label_from(Instruction::Left);
        let result_right = node.get_child_label_from(Instruction::Right);

        assert_eq!(result_left, expected_left);
        assert_eq!(result_right, expected_right);
    }

    #[test]
    fn test_map_steps_between() {
        let inputs = [
            "RL".to_string(),
            String::new(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];

        let map = Map::new(&inputs);

        let expected = 2;

        let result = map.steps_between("AAA", "ZZZ");

        assert_eq!(result, expected);
    }
}
