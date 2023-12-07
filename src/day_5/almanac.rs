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
        let tables: Vec<Table> = input.split(String::is_empty).map(Table::new).collect();

        assert!(
            tables.len() == 7,
            "Input only had {} tables defined.",
            tables.len()
        );

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

    pub fn lowest_location_from_seed_ranges(&self, seed_ranges: &[(u64, u64)]) -> u64 {
        let soil_id_ranges = self.seed_to_soil.mapped_ranges(seed_ranges);
        let fertilizer_id_ranges = self.soil_to_fertilizer.mapped_ranges(&soil_id_ranges);
        let water_id_ranges = self
            .fertilizer_to_water
            .mapped_ranges(&fertilizer_id_ranges);
        let light_id_ranges = self.water_to_light.mapped_ranges(&water_id_ranges);
        let temperature_id_ranges = self.light_to_temperature.mapped_ranges(&light_id_ranges);
        let humidity_id_ranges = self
            .temperature_to_humidity
            .mapped_ranges(&temperature_id_ranges);

        self.humidity_to_location
            .mapped_ranges(&humidity_id_ranges)
            .into_iter()
            .map(|(start, _)| start)
            .min()
            .unwrap()
    }
}

impl Table {
    fn new(input: &[String]) -> Self {
        let title = input[0].replace(" map:", "");

        let mut mappings: Vec<Map> = input[1..].iter().map(|row| row.parse().unwrap()).collect();

        mappings.sort_by(|a, b| a.source_start.cmp(&b.source_start));

        Table { title, mappings }
    }

    fn value_of(&self, input: u64) -> u64 {
        self.mappings
            .iter()
            .fold(None, |acc, map| acc.or(map.value_of(input)))
            .unwrap_or(input)
    }

    fn mapped_ranges(&self, inputs: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut result: Vec<(u64, u64)> = inputs
            .iter()
            .flat_map(|(start, range)| self.mapped_range(*start, *range))
            .collect();

        result.sort_by(|a, b| a.0.cmp(&b.0));

        result
    }

    fn mapped_range(&self, input_start: u64, input_range: u64) -> Vec<(u64, u64)> {
        let input_upper_bound = input_start + input_range;

        let mut result = Vec::new();
        let mut next_start = input_start;
        let mut next_range = input_range;

        for map in &self.mappings {
            let next_start_destination = map.value_of(next_start).unwrap_or(next_start);
            let map_upper_bound = map.source_start + map.range;

            if next_range == 0 {
                break;
            }

            if map.contains(next_start) {
                let temp_range = next_range.min(map_upper_bound - next_start);

                result.push((next_start_destination, temp_range));

                next_range -= temp_range;
                next_start += temp_range;
            } else if map.contains(input_upper_bound - 1) {
                let beginning_range = map.source_start - next_start;
                next_range -= beginning_range;

                result.push((next_start, beginning_range));
                result.push((map.destination_start, next_range));

                next_range = 0;
                next_start = input_upper_bound;
            } else if map.is_contained_by_range(next_start, next_range) {
                let beginning_range = map.source_start - next_start;

                result.push((next_start, beginning_range));
                result.push((map.destination_start, map.range));

                next_range -= beginning_range + map.range;
                next_start = map_upper_bound;
            }
        }

        if next_range != 0 {
            result.push((next_start, next_range));
        }

        result.sort_by(|a, b| a.0.cmp(&b.0));

        result
    }
}

impl Map {
    fn value_of(&self, input: u64) -> Option<u64> {
        if self.contains(input) {
            let input_delta_from_start = input - self.source_start;

            Some(self.destination_start + input_delta_from_start)
        } else {
            None
        }
    }

    fn contains(&self, source_value: u64) -> bool {
        let source_upper_bound = self.source_start + self.range;

        (self.source_start..source_upper_bound).contains(&source_value)
    }

    fn is_contained_by_range(&self, input_start: u64, input_range: u64) -> bool {
        let source_upper_bound = self.source_start + self.range;
        let input_upper_bound = input_start + input_range;

        (input_start <= self.source_start) && (source_upper_bound <= input_upper_bound)
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<Result<u64, _>> = s.split(' ').map(str::parse).collect();

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
                    source_start: 0,
                    destination_start: 39,
                    range: 15,
                },
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
        let expected_1 = 49;
        let expected_2 = 52;
        let expected_3 = 99;
        let expected_4 = 50;
        let expected_5 = 51;

        let result_0 = table.value_of(0);
        let result_1 = table.value_of(49);
        let result_2 = table.value_of(50);
        let result_3 = table.value_of(97);
        let result_4 = table.value_of(98);
        let result_5 = table.value_of(99);

        assert_eq!(result_0, expected_0);
        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
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

    #[test]
    fn test_table_mapped_range() {
        let input = [
            "temp title".to_string(),
            "52 50 48".to_string(),
            "50 98 2".to_string(),
        ];

        let table = Table::new(&input);

        let out_of_range_lower = (0, 50);
        let out_of_range_upper = (100, 50);
        let within_lower_range = (50, 26);
        let partial_within_lower_range = (45, 31);
        let partial_within_both = (75, 25);
        let contains_both = (0, 150);
        let contains_upper = (99, 27);

        let expected_out_of_range_lower = vec![(0, 50)];
        let expected_out_of_range_upper = vec![(100, 50)];
        let expected_within_lower_range = vec![(52, 26)];
        let expected_partial_within_lower_range = vec![(45, 5), (52, 26)];
        let expected_partial_within_both = vec![(50, 2), (77, 23)];
        let expected_contains_both = vec![(0, 50), (50, 2), (52, 48), (100, 50)];
        let expected_contains_upper = vec![(51, 1), (100, 26)];

        let result_out_of_range_lower =
            table.mapped_range(out_of_range_lower.0, out_of_range_lower.1);
        let result_out_of_range_upper =
            table.mapped_range(out_of_range_upper.0, out_of_range_upper.1);
        let result_within_lower_range =
            table.mapped_range(within_lower_range.0, within_lower_range.1);
        let result_partial_within_lower_range =
            table.mapped_range(partial_within_lower_range.0, partial_within_lower_range.1);
        let result_partial_within_both =
            table.mapped_range(partial_within_both.0, partial_within_both.1);
        let result_contains_both = table.mapped_range(contains_both.0, contains_both.1);
        let result_contains_upper = table.mapped_range(contains_upper.0, contains_upper.1);

        assert_eq!(result_out_of_range_lower, expected_out_of_range_lower);
        assert_eq!(result_out_of_range_upper, expected_out_of_range_upper);
        assert_eq!(result_within_lower_range, expected_within_lower_range);
        assert_eq!(
            result_partial_within_lower_range,
            expected_partial_within_lower_range
        );
        assert_eq!(result_partial_within_both, expected_partial_within_both);
        assert_eq!(result_contains_both, expected_contains_both);
        assert_eq!(result_contains_upper, expected_contains_upper);
    }

    #[test]
    fn test_table_mapped_ranges() {
        let input = [
            "temp title".to_string(),
            "0 0 26".to_string(),
            "75 75 101".to_string(),
            "180 180 21".to_string(),
            "250 250 51".to_string(),
        ];

        let table = Table::new(&input);

        let ranges = [(20, 61), (160, 61)];

        let expected = vec![
            (20, 6),
            (26, 49),
            (75, 6),
            (160, 16),
            (176, 4),
            (180, 21),
            (201, 20),
        ];

        let result = table.mapped_ranges(&ranges);

        assert_eq!(result, expected);
    }
}
