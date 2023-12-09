use utils::{get_input_path, read_lines};

const DIGIT_TUPLES: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

struct Occurrence {
    index: usize,
    substr: String,
    digit_val: String,
}

fn replace_digit_str(line: String) -> String {
    let mut occurrences: Vec<_> = DIGIT_TUPLES.iter()
        .flat_map(|&(digit_str, digit_val)| {
            line.match_indices(digit_str)
                .map(|(index, substr)| Occurrence {
                    index,
                    substr: String::from(substr),
                    digit_val: String::from(digit_val),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    occurrences.sort_by_key(|occurrence| occurrence.index);

    struct OccurrenceScanState {
        curr_str: String,
        curr_index: usize
    }

    let scan_state = occurrences.into_iter().fold( OccurrenceScanState{curr_str: String::new(), curr_index: 0}, |state, occurrence| {
        

        let mut new_str = state.curr_str;
          
        if state.curr_index < occurrence.index {
            new_str.push_str(&line[state.curr_index..occurrence.index]);
        }

        new_str.push_str(&occurrence.digit_val);
        let new_index = occurrence.index + occurrence.substr.len();

        OccurrenceScanState {
            curr_str: new_str,
            curr_index: new_index
        }
    });

    let repl_str = scan_state.curr_str + &line[scan_state.curr_index..];
    return repl_str;



    // let mut repl_line = String::new();
    // let mut curr_index: usize = 0;
    // for &occurrence in occurrences.iter() {
    //     if curr_index != occurrence.index {
    //         repl_line.push_str(&line[curr_index..occurrence.index]);
    //     }
    //     repl_line.push_str(&occurrence.digit_val);
    //     curr_index = occurrence.index + occurrence.substr.len()
    // }
    // repl_line.push_str(&line[curr_index..]);

    //println!("{} => {}", line, &repl_line);
    //println!("{}", &repl_line);
}

fn main() {
    let input_path = get_input_path(1, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let calibrations: Vec<_> = lines
            .map(|line| line.unwrap())
            .map(|line| (String::from(&line), replace_digit_str(String::from(line))))
            .map(|(line, repl_line)| {
                (
                    String::from(&line),
                    String::from(&repl_line),
                    get_calibration(&repl_line),
                )
            })
            .collect();
        // .map(|line| line.unwrap())
        // .map(|line| replace_digit_str(line))
        // .map(|line| get_calibration(&line))
        // .collect();

        // for ele in calibrations.iter() {
        //     println!("{} => {} = {}", &ele.0, &ele.1, &ele.2)
        // }

        let sum: u32 = calibrations.iter().map(|vals| vals.2).sum();
        println!("The sum is {}", sum)
    } else {
        println!("File cannot be found")
    }
}

fn get_calibration(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|character| character.to_digit(10))
        .collect();

    if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {
        return (*first_digit * 10 + *last_digit) as u32;
    } else {
        panic!("Cannot find digits");
    }
}

// fn get_calibration(line: &str) -> u32 {
//     let mut digits = Vec::<u32>::new();

//     let mut digit_buf = String::new();
//     for character in line.chars() {
//         if character.is_ascii_alphabetic() {
//             digit_buf += &character.to_string();
//         } else if character.is_ascii_digit() {
//             digits.append(&mut find_digits_in_buf_and_clear(&mut digit_buf));
//             digits.push(character.to_digit(10).unwrap())
//         }
//     }
//     // final check in buffer
//     find_digits_in_buf_and_clear(&mut digit_buf);

//     if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {
//         return (*first_digit * 10 + *last_digit) as u32;
//     } else {
//         panic!("Cannot find digits");
//     }
// }

// fn find_digits_in_buf_and_clear(digit_buf: &mut String) -> Vec<u32> {
//     let mut digits = Vec::<u32>::new();
//     for (digit_str, digit_val) in DIGIT_TUPLES.iter() {
//         if digit_buf.contains(*digit_str) {
//             digits.push(*digit_val);
//         }
//     }
//     digit_buf.clear();
//     return digits;
// }
