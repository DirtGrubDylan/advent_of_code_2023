use std::collections::HashSet;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct PipeNetwork {
    network: Vec<Vec<Pipe>>,
    network_loop: HashSet<(usize, usize)>,
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

        let start = *network
            .iter()
            .flatten()
            .find(|pipe| pipe.pipe_type == PipeType::Start)
            .unwrap();

        let mut pipe_network = PipeNetwork {
            network,
            network_loop: HashSet::new(),
            start_x_location: start.x_location,
            start_y_location: start.y_location,
        };

        pipe_network.convert_start_in_place();

        pipe_network.network_loop = pipe_network.get_loop_locations();

        pipe_network.replace_non_loop_tiles();

        pipe_network
    }

    pub fn length_of_network_loop(&self) -> usize {
        self.network_loop.len()
    }

    pub fn number_of_enclosed_tiles(&self) -> usize {
        let mut result = 0;
        let mut previous_type = PipeType::Ground;

        for row in &self.network {
            let mut is_outside = true;

            for current_type in row.iter().map(|pipe| pipe.pipe_type) {
                match (current_type, previous_type, is_outside) {
                    (PipeType::Horizontal, _, _)
                    | (PipeType::SouthWestBend, PipeType::NorthEastBend, _)
                    | (PipeType::NorthWestBend, PipeType::SouthEastBend, _)
                    | (PipeType::Ground, _, true) => {}
                    (PipeType::Ground, _, false) => {
                        result += 1;
                    }
                    _ => {
                        is_outside = !is_outside;
                    }
                }

                if current_type != PipeType::Horizontal {
                    previous_type = current_type;
                }
            }
        }

        result
    }

    fn replace_non_loop_tiles(&mut self) {
        for (row_idx, row) in self.network.iter_mut().enumerate() {
            for (col_idx, pipe) in row.iter_mut().enumerate() {
                if !self.network_loop.contains(&(col_idx, row_idx)) {
                    pipe.set_type(PipeType::Ground);
                }
            }
        }
    }

    fn convert_start_in_place(&mut self) {
        let new_start_type = self.convert_start();

        if let Some(start) = self.get_pipe_mut(self.start_x_location, self.start_y_location) {
            start.set_type(new_start_type);
        }
    }

    fn get_loop_locations(&self) -> HashSet<(usize, usize)> {
        let start_location = (self.start_x_location, self.start_y_location);
        let start_pipe = self
            .get_pipe(self.start_x_location, self.start_y_location)
            .unwrap();

        let mut result = HashSet::from([start_location]);
        let mut current_direction = start_pipe.can_go().unwrap().0;
        let mut current_location = start_pipe.next_location_going(current_direction).unwrap();
        let mut current_pipe = self
            .get_pipe(current_location.0, current_location.1)
            .unwrap();

        while current_location != start_location {
            result.insert(current_location);

            current_direction = current_pipe
                .next_direction_going(current_direction)
                .unwrap();
            current_location = current_pipe.next_location_going(current_direction).unwrap();
            current_pipe = self
                .get_pipe(current_location.0, current_location.1)
                .unwrap();
        }

        result
    }

    fn convert_start(&self) -> PipeType {
        let start_pipe = self
            .get_pipe(self.start_x_location, self.start_y_location)
            .unwrap();

        if start_pipe.pipe_type != PipeType::Start {
            return start_pipe.pipe_type;
        }

        let north_location = start_pipe.next_location_going(Direction::North).unwrap();
        let east_location = start_pipe.next_location_going(Direction::East).unwrap();
        let south_location = start_pipe.next_location_going(Direction::South).unwrap();
        let west_location = start_pipe.next_location_going(Direction::West).unwrap();

        let mut can_go_north = self.start_y_location != 0;
        // let mut can_go_south = self.start_y_location < self.network.len();
        let mut can_go_west = self.start_x_location != 0;
        // let mut can_go_west = self.start_x_location < self.network.get(0).unwrap().len();

        let north_pipe = if can_go_north {
            self.get_pipe(north_location.0, north_location.1)
        } else {
            None
        };
        let west_pipe = if can_go_west {
            self.get_pipe(west_location.0, west_location.1)
        } else {
            None
        };
        let south_pipe = self.get_pipe(south_location.0, south_location.1);
        let east_pipe = self.get_pipe(east_location.0, east_location.1);

        can_go_north = north_pipe.map_or(false, |pipe| pipe.can_be_entered_from(Direction::South));
        can_go_west = west_pipe.map_or(false, |pipe| pipe.can_be_entered_from(Direction::East));
        let can_go_south =
            south_pipe.map_or(false, |pipe| pipe.can_be_entered_from(Direction::North));
        let can_go_east = east_pipe.map_or(false, |pipe| pipe.can_be_entered_from(Direction::West));

        if can_go_north && can_go_east {
            PipeType::NorthEastBend
        } else if can_go_north && can_go_west {
            PipeType::NorthWestBend
        } else if can_go_south && can_go_east {
            PipeType::SouthEastBend
        } else if can_go_south && can_go_west {
            PipeType::SouthWestBend
        } else if can_go_north && can_go_south {
            PipeType::Vertical
        } else if can_go_east && can_go_west {
            PipeType::Horizontal
        } else {
            PipeType::Start
        }
    }

    fn get_pipe_mut(&mut self, x_location: usize, y_location: usize) -> Option<&mut Pipe> {
        self.network
            .get_mut(y_location)
            .and_then(|row| row.get_mut(x_location))
    }

    fn get_pipe(&self, x_location: usize, y_location: usize) -> Option<&Pipe> {
        self.network
            .get(y_location)
            .and_then(|row| row.get(x_location))
    }

    #[allow(dead_code)]
    fn to_map(&self) -> Vec<String> {
        self.network
            .iter()
            .map(|row| Self::row_to_map(row))
            .collect()
    }

    #[allow(dead_code)]
    fn row_to_map(row: &[Pipe]) -> String {
        row.iter().map(|pipe| char::from(pipe.pipe_type)).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
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

impl From<PipeType> for char {
    fn from(p: PipeType) -> Self {
        match p {
            PipeType::Vertical => '|',
            PipeType::Horizontal => '-',
            PipeType::NorthEastBend => 'L',
            PipeType::NorthWestBend => 'J',
            PipeType::SouthEastBend => 'F',
            PipeType::SouthWestBend => '7',
            PipeType::Ground => '.',
            PipeType::Start => 'S',
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

    fn set_type(&mut self, new_type: PipeType) {
        self.pipe_type = new_type;
    }

    fn next_location_going(&self, direction: Direction) -> Option<(usize, usize)> {
        let y_north = self.y_location.saturating_sub(1);
        let y_south = self.y_location + 1;
        let x_east = self.x_location + 1;
        let x_west = self.x_location.saturating_sub(1);

        match (self.pipe_type, direction) {
            (
                PipeType::Vertical
                | PipeType::NorthEastBend
                | PipeType::NorthWestBend
                | PipeType::Start,
                Direction::North,
            ) => Some((self.x_location, y_north)),
            (
                PipeType::Vertical
                | PipeType::SouthEastBend
                | PipeType::SouthWestBend
                | PipeType::Start,
                Direction::South,
            ) => Some((self.x_location, y_south)),
            (
                PipeType::Horizontal
                | PipeType::NorthEastBend
                | PipeType::SouthEastBend
                | PipeType::Start,
                Direction::East,
            ) => Some((x_east, self.y_location)),
            (
                PipeType::Horizontal
                | PipeType::NorthWestBend
                | PipeType::SouthWestBend
                | PipeType::Start,
                Direction::West,
            ) => Some((x_west, self.y_location)),
            _ => None,
        }
    }

    fn next_direction_going(&self, direction: Direction) -> Option<Direction> {
        match (self.pipe_type, direction) {
            (PipeType::Vertical, Direction::North)
            | (PipeType::NorthEastBend, Direction::West)
            | (PipeType::NorthWestBend, Direction::East) => Some(Direction::North),
            (PipeType::Vertical, Direction::South)
            | (PipeType::SouthEastBend, Direction::West)
            | (PipeType::SouthWestBend, Direction::East) => Some(Direction::South),
            (PipeType::Horizontal, Direction::East)
            | (PipeType::NorthEastBend, Direction::South)
            | (PipeType::SouthEastBend, Direction::North) => Some(Direction::East),
            (PipeType::Horizontal, Direction::West)
            | (PipeType::NorthWestBend, Direction::South)
            | (PipeType::SouthWestBend, Direction::North) => Some(Direction::West),
            _ => None,
        }
    }

    fn can_go(&self) -> Option<(Direction, Direction)> {
        match self.pipe_type {
            PipeType::Vertical => Some((Direction::North, Direction::South)),
            PipeType::Horizontal => Some((Direction::East, Direction::West)),
            PipeType::NorthEastBend => Some((Direction::North, Direction::East)),
            PipeType::NorthWestBend => Some((Direction::North, Direction::West)),
            PipeType::SouthEastBend => Some((Direction::South, Direction::East)),
            PipeType::SouthWestBend => Some((Direction::South, Direction::West)),
            _ => None,
        }
    }

    fn can_be_entered_from(&self, direction: Direction) -> bool {
        matches!(
            (self.pipe_type, direction),
            (
                PipeType::Vertical | PipeType::NorthEastBend | PipeType::NorthWestBend,
                Direction::North
            ) | (
                PipeType::Vertical | PipeType::SouthEastBend | PipeType::SouthWestBend,
                Direction::South
            ) | (
                PipeType::Horizontal | PipeType::NorthEastBend | PipeType::SouthEastBend,
                Direction::East
            ) | (
                PipeType::Horizontal | PipeType::NorthWestBend | PipeType::SouthWestBend,
                Direction::West
            ) | (PipeType::Start, _)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_next_location_going() {
        let vertical = Pipe::new(1, 1, '|');
        let horizontal = Pipe::new(1, 1, '-');
        let north_east_bend = Pipe::new(1, 1, 'L');
        let north_west_bend = Pipe::new(1, 1, 'J');
        let south_east_bend = Pipe::new(1, 1, 'F');
        let south_west_bend = Pipe::new(1, 1, '7');
        let ground = Pipe::new(1, 1, '.');
        let start = Pipe::new(1, 1, 'S');

        let vertical_next = vertical.next_location_going(Direction::North);
        let vertical_next_none = vertical.next_location_going(Direction::East);
        let horizontal_next = horizontal.next_location_going(Direction::East);
        let north_east_bend_next = north_east_bend.next_location_going(Direction::East);
        let north_east_bend_next_none = north_east_bend.next_location_going(Direction::South);
        let north_west_bend_next = north_west_bend.next_location_going(Direction::North);
        let south_east_bend_next = south_east_bend.next_location_going(Direction::South);
        let south_west_bend_next = south_west_bend.next_location_going(Direction::West);
        let ground_next = ground.next_location_going(Direction::North);
        let start_next = start.next_location_going(Direction::North);

        assert_eq!(vertical_next, Some((1, 0)));
        assert_eq!(vertical_next_none, None);
        assert_eq!(horizontal_next, Some((2, 1)));
        assert_eq!(north_east_bend_next, Some((2, 1)));
        assert_eq!(north_east_bend_next_none, None);
        assert_eq!(north_west_bend_next, Some((1, 0)));
        assert_eq!(south_east_bend_next, Some((1, 2)));
        assert_eq!(south_west_bend_next, Some((0, 1)));
        assert_eq!(ground_next, None);
        assert_eq!(start_next, Some((1, 0)));
    }

    #[test]
    fn test_pipe_can_be_entered_from() {
        let vertical = Pipe::new(1, 1, '|');
        let horizontal = Pipe::new(1, 1, '-');
        let north_east_bend = Pipe::new(1, 1, 'L');
        let north_west_bend = Pipe::new(1, 1, 'J');
        let south_east_bend = Pipe::new(1, 1, 'F');
        let south_west_bend = Pipe::new(1, 1, '7');
        let ground = Pipe::new(1, 1, '.');
        let start = Pipe::new(1, 1, 'S');

        assert!(vertical.can_be_entered_from(Direction::South));
        assert!(!vertical.can_be_entered_from(Direction::West));
        assert!(horizontal.can_be_entered_from(Direction::West));
        assert!(north_east_bend.can_be_entered_from(Direction::North));
        assert!(!north_east_bend.can_be_entered_from(Direction::West));
        assert!(north_west_bend.can_be_entered_from(Direction::West));
        assert!(south_east_bend.can_be_entered_from(Direction::East));
        assert!(south_west_bend.can_be_entered_from(Direction::South));
        assert!(!ground.can_be_entered_from(Direction::West));
        assert!(start.can_be_entered_from(Direction::East));
    }

    #[test]
    fn test_pipe_network_convert_start_1() {
        let input = [
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = PipeType::SouthEastBend;

        let result = network.convert_start();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_convert_start_2() {
        let input = [
            "7-F7-".to_string(),
            ".FJ|7".to_string(),
            "SJLL7".to_string(),
            "|F--J".to_string(),
            "LJ.LJ".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = PipeType::SouthEastBend;

        let result = network.convert_start();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_get_loop_locations() {
        let input = [
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = HashSet::from([
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (3, 2),
            (3, 1),
            (2, 1),
        ]);

        let result = network.network_loop;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_replace_non_loop_tiles() {
        let input = [
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected_map = vec![
            ".....".to_string(),
            ".F-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ];

        let result_map = network.to_map();

        assert_eq!(result_map, expected_map);
    }

    #[test]
    fn test_pipe_network_length_of_network_loop_1() {
        let input = [
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = 8;

        let result = network.length_of_network_loop();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_length_of_network_loop_2() {
        let input = [
            "7-F7-".to_string(),
            ".FJ|7".to_string(),
            "SJLL7".to_string(),
            "|F--J".to_string(),
            "LJ.LJ".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = 16;

        let result = network.length_of_network_loop();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_number_of_enclosed_tiles_1() {
        let input = [
            "7-F7-".to_string(),
            ".FJ|7".to_string(),
            "SJLL7".to_string(),
            "|F--J".to_string(),
            "LJ.LJ".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = 1;

        let result = network.number_of_enclosed_tiles();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pipe_network_number_of_enclosed_tiles_2() {
        let input = [
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        ];

        let network = PipeNetwork::new(&input);

        let expected = 4;

        let result = network.number_of_enclosed_tiles();

        assert_eq!(result, expected);
    }
}
