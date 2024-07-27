use itertools::Itertools;
use std::iter::zip;
use utils::{get_input_path, read_lines};

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn get_victory_range_size(&self) -> u32 {
        let delta_squared: f64 = (self.time.pow(2) - 4 * (self.distance+1)).into();
        let delta = delta_squared.sqrt();

        let time: f64 = self.time.into();

        let upper = ((time + delta) / 2_f64).floor() as i32;
        let lower = ((time - delta) / 2_f64).ceil() as i32;

        (upper - lower + 1) as u32
    }
}

fn main() {
    let input_path = get_input_path(6, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        fn get_values(line: &String) -> Vec<u32> {
            line.split(":")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n_str| n_str.parse::<u32>().unwrap())
                .collect_vec()
        }

        let times = get_values(&lines[0]);
        let distances = get_values(&lines[1]);

        let races = zip(times, distances).map(|(time, distance)| Race { time, distance }).collect_vec();

        let ranges = races.iter().map(|race| race.get_victory_range_size()).collect_vec();

        println!("Ranges: {:?}", &ranges);

        let combined = ranges.into_iter().reduce(|a, b| a*b).unwrap();

        println!("Result: {}", combined);
    }
}
