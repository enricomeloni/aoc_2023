use itertools::Itertools;
use std::iter::zip;
use utils::{get_input_path, read_lines};

struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn get_victory_range_size(&self) -> f64 {
        let time: f64 = self.time.into();
        let distance: f64 = (self.distance + 1.).into();

        let delta_squared: f64 = time.powf(2.) - 4. * (distance);
        let delta = delta_squared.sqrt();


        let upper = ((time + delta) / 2.).floor() as i32;
        let lower = ((time - delta) / 2.).ceil() as i32;

        (upper - lower + 1) as f64
    }
}

fn main() {
    let input_path = get_input_path(6, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        fn get_value(line: &String) -> f64 {
            line.split(":")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .replace(' ', "")
                .parse::<f64>()
                .unwrap()
        }

        let time = get_value(&lines[0]);
        let distance = get_value(&lines[1]);

        let race = Race { time, distance };

        let range = race.get_victory_range_size();

        println!("Range: {:?}", &range);
    }
}
