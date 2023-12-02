use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub id: u32,
    pub revealed_cubes: Vec<CubeCount>,
    pub minimum_required_cubes: CubeCount,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CubeCount {
    pub blue_cubes: u32,
    pub green_cubes: u32,
    pub red_cubes: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ParseError;

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl FromStr for CubeCount {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_count_from_str_ok() {
        let inputs = ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"];

        let expected = vec![
            CubeCount {
                blue_cubes: 3,
                green_cubes: 0,
                red_cubes: 4,
            },
            CubeCount {
                blue_cubes: 6,
                green_cubes: 2,
                red_cubes: 1,
            },
            CubeCount {
                blue_cubes: 0,
                green_cubes: 2,
                red_cubes: 0,
            },
        ];

        let result: Vec<CubeCount> = inputs
            .into_iter()
            .map(|input| input.parse().unwrap())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_game_from_str_ok() {
        let input = "Game 666: 3 blue, 4 red; 111 red, 2 green, 6 blue; 222 green";

        let expected_id = 666;

        let expected_revealed = vec![
            CubeCount {
                blue_cubes: 3,
                green_cubes: 0,
                red_cubes: 4,
            },
            CubeCount {
                blue_cubes: 6,
                green_cubes: 2,
                red_cubes: 111,
            },
            CubeCount {
                blue_cubes: 0,
                green_cubes: 222,
                red_cubes: 0,
            },
        ];

        let expected_minimum = CubeCount {
            blue_cubes: 6,
            green_cubes: 222,
            red_cubes: 111,
        };

        let expected = Game {
            id: expected_id,
            revealed_cubes: expected_revealed,
            minimum_required_cubes: expected_minimum,
        };

        let result: Game = input.parse().unwrap();

        assert_eq!(result, expected);
    }
}
