use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct PipeNetwork {
    network: Vec<Vec<Pipe>>,
    start_x_location: usize,
    start_y_location: usize,
}

impl PipeNetwork {
    pub fn new(map: &[String]) -> Self {
        let mut network: Vec<Vec<Pipe>> = Vec::new();

        for (row_idx, row) in map.iter().enumerate() {
            let network_row = row
                .chars()
                .enumerate()
                .map(|(col_idx, value)| Pipe::new(col_idx, row_idx, value))
                .collect();

            network.push(network_row);
        }

        let start = network
            .iter()
            .flatten()
            .filter(|pipe| pipe.pipe_type == PipeType::Start)
            .next()
            .unwrap();

        let (start_x_location, start_y_location) = (start.x_location, start.y_location);

        PipeNetwork {
            network,
            start_x_location,
            start_y_location,
        }
    }

    pub fn length_of_network_loop(&self) -> usize {
        unimplemented!()
    }

    fn convert_start(&self) -> PipeType {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthEastBend,
    SouthWestBend,
    Ground,
    Start,
}

impl From<char> for PipeType {
    fn from(c: char) -> Self {
        match c {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NorthEastBend,
            'J' => PipeType::NorthWestBend,
            'F' => PipeType::SouthEastBend,
            '7' => PipeType::SouthWestBend,
            '.' => PipeType::Ground,
            'S' => PipeType::Start,
            _ => panic!("Cannot convert '{c}' to a PipeType!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pipe {
    pipe_type: PipeType,
    x_location: usize,
    y_location: usize,
}

impl Pipe {
    fn new(x_location: usize, y_location: usize, value: char) -> Self {
        let pipe_type = PipeType::from(value);

        Pipe {
            pipe_type,
            x_location,
            y_location,
        }
    }

    fn next_clockwise_location(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn next_counter_clockwise_location(&self) -> (usize, usize) {
        unimplemented!()
    }

    fn can_be_entered_from(&self, from_x: usize, from_y) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_next_clockwise_location() {
        unimplemented!()
    }

    #[test]
    fn test_pipe_next_counter_clockwise_location() {
        unimplemented!()
    }

    #[test]
    fn test_pipe_can_be_entered_from() {
        unimplemented!()
    }

    #[test]
    fn test_pipe_network_convert_start() {
        unimplemented!()
    }

    #[test]
    fn test_pipe_network_length_of_network_loop_1() {
        unimplemented!()
    }

    #[test]
    fn test_pipe_network_length_of_network_loop_2() {
        unimplemented!()
    }
}
