use std::collections::{HashMap, HashSet, VecDeque};
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
        let mut steps: u32 = 0;

        let mut instructions_copy = self.instructions.clone();
        let mut current_node = self.network.get(start_label);

        while current_node.map_or("", |node| node.label.as_str()) != end_label {
            steps += 1;

            current_node = instructions_copy
                .front()
                .and_then(|instruction| self.get_child_node(current_node, *instruction));

            instructions_copy.rotate_left(1);
        }

        steps
    }

    pub fn steps_between_all(&self, start_ends_with: char, end_ends_with: char) -> u32 {
        let mut steps = 0;

        let mut instructions_copy = self.instructions.clone();
        let mut current_nodes: Vec<&Node> = self
            .network
            .iter()
            .filter(|(label, _)| label.ends_with(start_ends_with))
            .map(|(_, node)| node)
            .collect();

        loop {
            if current_nodes
                .iter()
                .all(|node| node.label.ends_with(end_ends_with))
            {
                break;
            }

            steps += 1;

            let instruction = *instructions_copy.front().unwrap();

            current_nodes = current_nodes
                .iter()
                .filter_map(|node| self.get_child_node(Some(node), instruction))
                .collect();

            instructions_copy.rotate_left(1);
        }

        steps
    }

    pub fn get_endings_info(&self, start_label: &str, end_ends_with: char) -> EndingsInfo {
        let mut steps = 0;

        let mut instructions_copy: VecDeque<(usize, Instruction)> = self
            .instructions
            .iter()
            .map(|inst| *inst)
            .enumerate()
            .collect();
        let mut seen_nodes: HashMap<(String, usize), usize> = HashMap::new();
        let mut current_node = self.network.get(start_label).unwrap();

        let mut instruction_index = instructions_copy.len() - 1;

        while !seen_nodes.contains_key(&(current_node.label.clone(), instruction_index)) {
            seen_nodes.insert((current_node.label.clone(), instruction_index), steps);

            steps += 1;

            let instruction = instructions_copy.front().unwrap().1;

            instruction_index = instructions_copy.front().unwrap().0;

            current_node = self
                .get_child_node(Some(current_node), instruction)
                .unwrap();

            instructions_copy.rotate_left(1);
        }

        let target_endings_steps: Vec<usize> = seen_nodes
            .iter()
            .filter(|((label, _), _)| label.ends_with(end_ends_with))
            .map(|(_, step)| *step)
            .collect();

        let repeating_start = *seen_nodes
            .get(&(current_node.label.clone(), instruction_index))
            .unwrap();
        let repeating_length = steps - repeating_start;

        EndingsInfo::new(&target_endings_steps, repeating_start, repeating_length)
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

#[derive(Debug, PartialEq)]
pub struct EndingsInfo {
    endings_steps: HashSet<usize>,
    repeating_start: usize,
    repeating_length: usize,
}

impl EndingsInfo {
    pub fn step_contains_ending(&self, step: usize) -> bool {
        self.endings_steps.contains(&self.adjust_step(step))
    }

    fn new(endings_steps: &[usize], repeating_start: usize, repeating_length: usize) -> Self {
        EndingsInfo {
            endings_steps: endings_steps.into_iter().map(|val| *val).collect(),
            repeating_start,
            repeating_length,
        }
    }

    fn adjust_step(&self, step: usize) -> usize {
        step.checked_sub(self.repeating_start)
            .map(|value| (value % self.repeating_length) + self.repeating_start)
            .unwrap_or(step)
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

    #[test]
    fn test_map_steps_between_all() {
        let inputs = [
            "LR".to_string(),
            String::new(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];

        let map = Map::new(&inputs);

        let expected = 6;

        let result = map.steps_between_all('A', 'Z');

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_get_endings_info() {
        let inputs = [
            "LR".to_string(),
            String::new(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];

        let map = Map::new(&inputs);

        let expected = EndingsInfo::new(&[3, 6], 1, 6);

        let result = map.get_endings_info("22A", 'Z');

        assert_eq!(result, expected);
    }

    #[test]
    fn test_endings_info_adjust_step() {
        let input = [0, 1, 3, 5, 13, 22, 49, 50];

        let info = EndingsInfo::new(&[4, 7], 2, 6);

        let expected = vec![0, 1, 3, 5, 7, 4, 7, 2];

        let result: Vec<usize> = input.into_iter().map(|val| info.adjust_step(val)).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_endings_step_contains_ending() {
        let input = [0, 1, 3, 5, 14, 23, 50, 51];

        let info = EndingsInfo::new(&[5, 8], 3, 6);

        let expected = vec![false, false, false, true, true, true, true, false];

        let result: Vec<bool> = input
            .into_iter()
            .map(|val| info.step_contains_ending(val))
            .collect();

        assert_eq!(result, expected);
    }
}
