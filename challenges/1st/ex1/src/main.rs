use std::fs::File;
// use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path; 

fn main() {
    let input_path = Path::new("input.txt");

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
    let bytes = line.as_bytes();

    let digits: Vec<u8> = bytes
        .iter()
        .map(|&character| character)
        .filter(|&character| (character >= ('0' as u8)) && (character <= ('9' as u8)))
        .map(|ascii_val| ascii_val - '0' as u8)
        .collect();

    if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {    
        return (*first_digit * 10 + *last_digit) as u32;
    }
    else {
        panic!("Cannot find digits");
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}