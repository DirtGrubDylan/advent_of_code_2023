mod pipes;

use crate::util::file_reader::to_string_vector;

use pipes::PipeNetwork;

pub fn run() {
    let input = to_string_vector("inputs/day_10.txt").expect("Something went wrong with Day 10!");

    let network = PipeNetwork::new(&input);

    println!("Day 10 Part 1: {}", part_1(&network));
    println!("Day 10 Part 2: {}", part_2(&network));
}

fn part_1(network: &PipeNetwork) -> usize {
    network.length_of_network_loop() / 2
}

fn part_2(network: &PipeNetwork) -> usize {
    network.number_of_enclosed_tiles()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_10_part_1.txt")
            .expect("Something went wrong with Day 10 Part 1 Test!");

        let network = PipeNetwork::new(&input);

        let expected = 8;

        let result = part_1(&network);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_10_part_2.txt")
            .expect("Something went wrong with Day 10 Part 2 Test!");

        let network = PipeNetwork::new(&input);

        let expected = 8;

        let result = part_2(&network);

        assert_eq!(result, expected);
    }
}
