use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf}; 

pub fn get_input_path(day: u8, file: Option<&str>) -> PathBuf {
    PathBuf::from(format!("inputs/{}/{}", day, file.unwrap_or("input.txt")))
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
