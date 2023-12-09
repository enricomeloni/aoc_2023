use utils::{read_lines, get_input_path};

fn main() {
    let input_path = get_input_path(1, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let calibrations: Vec<u32> = lines
            .map(|line| line.unwrap())
            .map(|line| get_calibration(&line))
            .collect();

        let sum: u32 = calibrations.iter().sum();
        println!("The sum is {}", sum)
    }
    else {
        println!("File cannot be found")
    }

}

fn get_calibration(line: &str) -> u32 {

    let digits: Vec<u32> = line.chars()
        .filter_map(|character| character.to_digit(10))
        .collect();

    if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {    
        return (*first_digit * 10 + *last_digit) as u32;
    }
    else {
        panic!("Cannot find digits");
    }

}