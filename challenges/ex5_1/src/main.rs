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
}

impl Maps {
    fn process(&self, seed: u64) -> u64 {

        let x = self.seed_to_soil.process(seed);
        let x = self.soil_to_fertilizer.process(x);
        let x = self.fertilizer_to_water.process(x);
        let x = self.water_to_light.process(x);
        let x = self.light_to_temperature.process(x);
        let x = self.temperature_to_humidity.process(x);
        let x = self.humidity_to_location.process(x);

        return x

        // self.humidity_to_location.process(
        //     self.temperature_to_humidity.process(
        //         self.light_to_temperature.process(
        //             self.water_to_light.process(
        //                 self.fertilizer_to_water.process(
        //                     self.soil_to_fertilizer
        //                         .process(self.seed_to_soil.process(seed)),
        //                 ),
        //             ),
        //         ),
        //     ),
        // )
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

            Range {
                source_start,
                destination_start,
                range_size,
            }
        })
        .collect_vec();

    Map { ranges }
}

fn main() {
    let input_path = get_input_path(5, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let seeds_line = &lines[0]["seeds: ".len()..].trim();
        let seeds = seeds_line
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect_vec();

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

        // println!("seeds {:?}", &seeds);

        let locations = seeds.into_iter().map(|seed| maps.process(seed)).collect_vec();

        // println!("locations {:?}", &locations);
        println!("Minimum is {}", locations.iter().min().unwrap())
    }
}
