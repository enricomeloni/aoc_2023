use itertools::Itertools;
use utils::{get_input_path, read_lines};

fn get_differences(seq: &Vec<i32>) -> Vec<i32> {
    let mut diff_seq = Vec::with_capacity(seq.len() - 1);

    for idx in (1..seq.len()).rev() {
        diff_seq.push(seq[idx] - seq[idx - 1]);
    }

    diff_seq.reverse();
    diff_seq
}

fn get_prev_value(seq: &Vec<i32>) -> i32 {
    if seq.iter().all(|&v| v == 0) {
        return 0;
    } else {
        let first = seq.first().unwrap();
        let differences = get_differences(seq);
        let prev_value = get_prev_value(&differences);
        return first - prev_value;
    }
}

fn main() {
    let input_path = get_input_path(9, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let sequences = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|number_str| number_str.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let next_values = sequences.iter().map(get_prev_value).collect_vec();

        let sum: i32 = next_values.iter().sum();

        println!("{:?}", next_values);

        println!("The sum is {sum}");
    }
}
