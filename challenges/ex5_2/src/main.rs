use std::collections::HashMap;

use itertools::Itertools;
use utils::{get_input_path, read_lines};

struct Range {
    source_start: u64,
    destination_start: u64,
    range_size: u64,
}

struct Map {
    ranges: Vec<Range>,
}

struct Maps {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Range {
    fn process(&self, seed: u64) -> Result<u64, &str> {
        if (self.source_start..self.source_start + self.range_size).contains(&seed) {
            let offset = seed - self.source_start;
            return Ok(self.destination_start + offset);
        }
        return Err("Not in range");
    }

    fn get_source_range(&self) -> std::ops::Range<u64> {
        return self.source_start..self.source_start + self.range_size;
    }
}

impl Map {
    fn process(&self, seed: u64) -> u64 {
        for range in &self.ranges {
            if let Ok(mapped) = range.process(seed) {
                return mapped;
            }
        }
        return seed;
    }

    fn get_containing_range(&self, seed: u64) -> Option<&Range> {
        for range in &self.ranges {
            if range.get_source_range().contains(&seed) {
                return Some(&range);
            }
        }
        return None;
    }

    fn process_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let mut curr_seed_range = seed_range.clone();
        let mut new_ranges: Vec<SeedRange> = Vec::new();

        loop {
            if let Some(containing_range) = self.get_containing_range(curr_seed_range.start) {
                let next_start = containing_range.process(curr_seed_range.start).unwrap();
                let available_size = containing_range.range_size
                    - (curr_seed_range.start - containing_range.source_start);
                if curr_seed_range.size < available_size {
                    new_ranges.push(SeedRange {
                        start: next_start,
                        size: curr_seed_range.size,
                    });
                    break;
                } else {
                    new_ranges.push(SeedRange {
                        start: next_start,
                        size: available_size,
                    });

                    curr_seed_range.start = curr_seed_range.start + available_size;
                    curr_seed_range.size = curr_seed_range.size - available_size;
                    continue;
                }
            } else {
                let candidate_end = curr_seed_range.start + curr_seed_range.size;

                if let Some(containing_range) = self.get_containing_range(candidate_end) {
                    let new_range = SeedRange {
                        start: curr_seed_range.start,
                        size: containing_range.source_start - curr_seed_range.start,
                    };

                    curr_seed_range.start = containing_range.source_start;
                    curr_seed_range.size = curr_seed_range.size - new_range.size;

                    new_ranges.push(new_range);
                } else {
                    new_ranges.push(curr_seed_range.clone());
                    break;
                }
            }
        }

        return new_ranges;
    }
}

fn apply_map_to_ranges(ranges: Vec<SeedRange>, map: &Map) -> Vec<SeedRange> {
    let mapped_ranges = ranges
        .iter()
        .flat_map(|range| map.process_range(range))
        .collect_vec();

    let mapped_size = mapped_ranges.iter().map(|range| range.size).sum::<u64>();
    let original_size = ranges.iter().map(|range| range.size).sum::<u64>();

    assert!(mapped_size == original_size);

    return mapped_ranges;
}

impl Maps {
    fn process(&self, seed: u64) -> u64 {
        let x1 = self.seed_to_soil.process(seed);
        let x2 = self.soil_to_fertilizer.process(x1);
        let x3 = self.fertilizer_to_water.process(x2);
        let x4 = self.water_to_light.process(x3);
        let x5 = self.light_to_temperature.process(x4);
        let x6 = self.temperature_to_humidity.process(x5);
        let x7 = self.humidity_to_location.process(x6);

        println!(
            "{} s=>s {} s=>f {} f=>w {} w=>l {} l=>t {} t=>h {} h=>l {}",
            seed, x1, x2, x3, x4, x5, x6, x7
        );

        return x7;
    }

    fn process_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let start_ranges = vec![seed_range.clone()];
        let x1 = apply_map_to_ranges(start_ranges, &self.seed_to_soil);
        let x2 = apply_map_to_ranges(x1, &self.soil_to_fertilizer);
        let x3 = apply_map_to_ranges(x2, &self.fertilizer_to_water);
        let x4 = apply_map_to_ranges(x3, &self.water_to_light);
        let x5 = apply_map_to_ranges(x4, &self.light_to_temperature);
        let x6 = apply_map_to_ranges(x5, &self.temperature_to_humidity);
        let x7 = apply_map_to_ranges(x6, &self.humidity_to_location);
        x7
    }
}

fn parse_map(range_lines: &[&String]) -> Map {
    /* The line is in format X Y Z */
    let ranges = range_lines
        .iter()
        .map(|range_line| {
            let (destination_start, source_start, range_size) = range_line
                .split_whitespace()
                .map(|number_str| number_str.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            // let source_range = source_start..source_start + range_size;
            Range {
                source_start,
                destination_start,
                range_size,
            }
        })
        .collect_vec();

    Map { ranges }
}

#[derive(Clone)]
struct SeedRange {
    start: u64,
    size: u64,
}

fn main() {
    let input_path = get_input_path(5, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let mut sections: Vec<Vec<&String>> = Vec::new();

        let mut curr_section: Vec<&String> = Vec::new();
        for line in lines[2..].iter() {
            if line != "" {
                curr_section.push(line);
            } else {
                sections.push(curr_section.clone());
                curr_section.clear();
            }
        }
        // push the last section
        sections.push(curr_section);

        let maps = Maps {
            seed_to_soil: parse_map(&sections[0][1..]),
            soil_to_fertilizer: parse_map(&sections[1][1..]),
            fertilizer_to_water: parse_map(&sections[2][1..]),
            water_to_light: parse_map(&sections[3][1..]),
            light_to_temperature: parse_map(&sections[4][1..]),
            temperature_to_humidity: parse_map(&sections[5][1..]),
            humidity_to_location: parse_map(&sections[6][1..]),
        };

        let seeds_line = &lines[0]["seeds: ".len()..].trim();
        let seeds_numbers = seeds_line
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect_vec();

        let mut seed_ranges: Vec<SeedRange> = Vec::new();
        let mut curr_range: [u64; 2] = [0, 0];
        for (idx, seed_number) in seeds_numbers.iter().enumerate() {
            let offset = idx % 2;

            curr_range[offset] = *seed_number;

            if offset == 1 {
                seed_ranges.push(SeedRange {
                    start: curr_range[0],
                    size: curr_range[1],
                })
            }
        }

        let mut final_ranges: Vec<SeedRange> = Vec::new();
        // for (idx, seed) in seeds.iter().enumerate() {
        for (idx, seed_range) in seed_ranges.iter().enumerate() {
            let ranges = maps.process_range(seed_range);
            println!("how many ranges {}", ranges.len());
            final_ranges.extend(ranges);
        }

        let min = final_ranges
            .iter()
            .min_by_key(|seed_range| seed_range.start)
            .unwrap()
            .start;

        println!("Minimum is {}", min)
    }
}
