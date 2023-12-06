use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seed_to_soil: Table,
    soil_to_fertilizer: Table,
    fertilizer_to_water: Table,
    water_to_light: Table,
    light_to_temperature: Table,
    temperature_to_humidity: Table,
    humidity_to_location: Table,
}

#[derive(Debug, PartialEq, Clone)]
struct Table {
    title: String,
    mappings: Vec<Map>,
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl Almanac {
    /// Creates a new Almanac
    ///
    /// # Panics
    ///
    /// If there isn't 7 tables defined.
    pub fn new(input: &[String]) -> Self {
        let tables: Vec<Table> = input.split(|row| row.is_empty()).map(Table::new).collect();

        if tables.len() != 7 {
            panic!("Input only had {} tables defined.", tables.len());
        }

        Almanac {
            seed_to_soil: tables[0].clone(),
            soil_to_fertilizer: tables[1].clone(),
            fertilizer_to_water: tables[2].clone(),
            water_to_light: tables[3].clone(),
            light_to_temperature: tables[4].clone(),
            temperature_to_humidity: tables[5].clone(),
            humidity_to_location: tables[6].clone(),
        }
    }

    pub fn seed_location(&self, seed_id: u64) -> u64 {
        let soil_id = self.seed_to_soil.value_of(seed_id);
        let fertilizer_id = self.soil_to_fertilizer.value_of(soil_id);
        let water_id = self.fertilizer_to_water.value_of(fertilizer_id);
        let light_id = self.water_to_light.value_of(water_id);
        let temperature_id = self.light_to_temperature.value_of(light_id);
        let humidity_id = self.temperature_to_humidity.value_of(temperature_id);

        self.humidity_to_location.value_of(humidity_id)
    }
}

impl Table {
    fn new(input: &[String]) -> Self {
        let title = input[0].replace(" map:", "");

        let mappings = input[1..].iter().map(|row| row.parse().unwrap()).collect();

        Table { title, mappings }
    }

    fn value_of(&self, input: u64) -> u64 {
        self.mappings
            .iter()
            .fold(None, |acc, map| acc.or(map.value_of(input)))
            .unwrap_or(input)
    }
}

impl Map {
    fn value_of(&self, input: u64) -> Option<u64> {
        let source_upper_bound = self.source_start + self.range;

        if (self.source_start..source_upper_bound).contains(&input) {
            let input_delta_from_start = input - self.source_start;

            Some(self.destination_start + input_delta_from_start)
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<Result<u64, _>> = s.split(' ').map(|value| value.parse()).collect();

        if values.len() != 3 {
            return Err(format!("{s} does not have 3 distinct values."));
        }

        match (&values[1], &values[0], &values[2]) {
            (&Ok(source_start), &Ok(destination_start), &Ok(range)) => Ok(Map {
                source_start,
                destination_start,
                range,
            }),
            _ => Err(format!("{s} does not have valid u64 values.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::almanac;

    use super::*;

    #[test]
    fn test_map_from_str() {
        let map_str = "0 15 37";

        let expected = Map {
            source_start: 15,
            destination_start: 0,
            range: 37,
        };

        let result: Map = map_str.parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_table_new() {
        let input = [
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
        ];

        let expected = Table {
            title: "soil-to-fertilizer".to_string(),
            mappings: vec![
                Map {
                    source_start: 15,
                    destination_start: 0,
                    range: 37,
                },
                Map {
                    source_start: 52,
                    destination_start: 37,
                    range: 2,
                },
                Map {
                    source_start: 0,
                    destination_start: 39,
                    range: 15,
                },
            ],
        };

        let result = Table::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_value_of_none() {
        let map = Map {
            source_start: 15,
            destination_start: 0,
            range: 37,
        };

        let result_for_less_than_start = map.value_of(14);
        let result_for_equal_to_start_plus_range = map.value_of(52);
        let result_for_greater_than_start_plus_range = map.value_of(53);

        assert!(result_for_less_than_start.is_none());
        assert!(result_for_equal_to_start_plus_range.is_none());
        assert!(result_for_greater_than_start_plus_range.is_none());
    }

    #[test]
    fn test_map_value_of_some() {
        let map = Map {
            source_start: 15,
            destination_start: 0,
            range: 37,
        };

        let expected_equal_to_start = Some(0);
        let expected_one_less_than_start_plus_range = Some(36);

        let result_equal_to_start = map.value_of(15);
        let result_one_less_than_start_plus_range = map.value_of(51);

        assert_eq!(result_equal_to_start, expected_equal_to_start);
        assert_eq!(
            result_one_less_than_start_plus_range,
            expected_one_less_than_start_plus_range
        );
    }

    #[test]
    fn test_table_value_of() {
        let input = [
            "temp title".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
        ];

        let table = Table::new(&input);

        let expected_0 = 0;
        let expected_49 = 49;
        let expected_50 = 52;
        let expected_97 = 99;
        let expected_98 = 50;
        let expected_99 = 51;

        let result_0 = table.value_of(0);
        let result_49 = table.value_of(49);
        let result_50 = table.value_of(50);
        let result_97 = table.value_of(97);
        let result_98 = table.value_of(98);
        let result_99 = table.value_of(99);

        assert_eq!(result_0, expected_0);
        assert_eq!(result_49, expected_49);
        assert_eq!(result_50, expected_50);
        assert_eq!(result_97, expected_97);
        assert_eq!(result_98, expected_98);
        assert_eq!(result_99, expected_99);
    }

    #[test]
    fn test_almanac_seed_location() {
        let input = [
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            String::new(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            String::new(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            String::new(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            String::new(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            String::new(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            String::new(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];

        let almanac = Almanac::new(&input);

        let expected_seed_79 = 82;
        let expected_seed_14 = 43;
        let expected_seed_55 = 86;
        let expected_seed_13 = 35;

        let result_seed_79 = almanac.seed_location(79);
        let result_seed_14 = almanac.seed_location(14);
        let result_seed_55 = almanac.seed_location(55);
        let result_seed_13 = almanac.seed_location(13);

        assert_eq!(result_seed_79, expected_seed_79);
        assert_eq!(result_seed_14, expected_seed_14);
        assert_eq!(result_seed_55, expected_seed_55);
        assert_eq!(result_seed_13, expected_seed_13);
    }
}
