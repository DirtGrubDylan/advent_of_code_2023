use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub id: u32,
    pub revealed_cubes: Vec<CubeCount>,
    pub minimum_required_cubes: CubeCount,
}

impl Game {
    pub fn new(id: u32, revealed_cubes: &[CubeCount]) -> Game {
        Game {
            id,
            revealed_cubes: revealed_cubes.to_vec(),
            minimum_required_cubes: Self::get_minimum_required(revealed_cubes),
        }
    }

    pub fn can_contain(&self, total_cubes: CubeCount) -> bool {
        self.minimum_required_cubes.blue_cubes <= total_cubes.blue_cubes
            && self.minimum_required_cubes.green_cubes <= total_cubes.green_cubes
            && self.minimum_required_cubes.red_cubes <= total_cubes.red_cubes
    }

    fn get_minimum_required(revealed_cubes: &[CubeCount]) -> CubeCount {
        revealed_cubes
            .iter()
            .fold(CubeCount::new(0, 0, 0), |acc, x| acc.total_maximum(x))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CubeCount {
    pub blue_cubes: u32,
    pub green_cubes: u32,
    pub red_cubes: u32,
}

impl CubeCount {
    pub fn new(blue_cubes: u32, green_cubes: u32, red_cubes: u32) -> Self {
        CubeCount {
            blue_cubes,
            green_cubes,
            red_cubes,
        }
    }

    pub fn total_maximum(&self, other: &Self) -> Self {
        let max_blue = self.blue_cubes.max(other.blue_cubes);
        let max_green = self.green_cubes.max(other.green_cubes);
        let max_red = self.red_cubes.max(other.red_cubes);

        CubeCount::new(max_blue, max_green, max_red)
    }

    pub fn power(&self) -> u32 {
        self.blue_cubes * self.green_cubes * self.red_cubes
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ParseError;

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (game_id_str, revealed_str) = input.split_once(": ").unwrap();

        let id: u32 = game_id_str.split_once(' ').unwrap().1.parse().unwrap();

        let revealed: Vec<CubeCount> = revealed_str
            .split("; ")
            .map(|split| split.parse().unwrap())
            .collect();

        Ok(Game::new(id, &revealed))
    }
}

impl FromStr for CubeCount {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let color_to_value: Vec<(String, u32)> = input
            .split(", ")
            .map(|item| item.split_once(' ').unwrap())
            .map(|(value_str, color)| (color.to_string(), value_str.parse().unwrap()))
            .collect();

        let mut blue_count = 0;
        let mut green_count = 0;
        let mut red_count = 0;

        for (color, value) in color_to_value {
            match color.as_str() {
                "blue" => blue_count += value,
                "green" => green_count += value,
                "red" => red_count += value,
                _ => return Err(ParseError),
            }
        }

        Ok(CubeCount::new(blue_count, green_count, red_count))
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

    #[test]
    fn test_game_can_contain_true() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let game: Game = input.parse().unwrap();

        let total_cubes = CubeCount::new(14, 13, 12);

        assert!(game.can_contain(total_cubes));
    }

    #[test]
    fn test_game_can_contain_false() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";

        let game: Game = input.parse().unwrap();

        let total_cubes = CubeCount::new(14, 13, 12);

        assert!(!game.can_contain(total_cubes));
    }
}
