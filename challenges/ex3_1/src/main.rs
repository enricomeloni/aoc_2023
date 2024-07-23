use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use utils::{get_input_path, read_lines};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Symbol {
    position: usize,
    value: char,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Number {
    line: usize,
    position: (usize, usize),
    value: u32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Line {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.position, self.value)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "([{}:{}]@{}: {})",
            self.position.0, self.position.1, self.line, self.value
        )
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numbers_str = self.numbers.iter().join(", ");
        let symbols_str = self.symbols.iter().join(", ");

        write!(f, "Numbers: {}\nSymbols: {}", numbers_str, symbols_str)
    }
}

fn parse_line(line_chars: &str, line_idx: usize) -> Line {
    enum ScanElem {
        Number(Number),
        Symbol(Symbol),
        None,
    }

    let padded_line = format!("{}.", line_chars);

    let scanned_elems: Vec<ScanElem> = padded_line
        .chars()
        .into_iter()
        .enumerate()
        .scan(String::new(), |curr_str, (idx, curr_char)| {
            let mut scan_elems: Vec<ScanElem> = Vec::new();

            if curr_char.is_digit(10) {
                curr_str.push(curr_char);
            } else {
                if let Ok(parsed) = curr_str.parse::<u32>() {
                    let scan_elem = ScanElem::Number(Number {
                        line: line_idx,
                        value: parsed,
                        position: (idx - curr_str.len(), idx - 1),
                    });
                    scan_elems.push(scan_elem);
                    curr_str.clear();
                }

                if curr_char != '.' {
                    scan_elems.push(ScanElem::Symbol(Symbol {
                        position: idx,
                        value: curr_char,
                    }));
                }
            }

            return Some(scan_elems);
        })
        .flatten()
        .collect_vec();

    let numbers: Vec<Number> = scanned_elems
        .iter()
        .filter_map(|scan_elem| {
            if let ScanElem::Number(number) = scan_elem {
                return Some(*number);
            } else {
                return None;
            }
        })
        .collect();

    let symbols: Vec<Symbol> = scanned_elems
        .iter()
        .filter_map(|scan_elem| {
            if let ScanElem::Symbol(symbol) = scan_elem {
                return Some(*symbol);
            } else {
                return None;
            }
        })
        .collect();

    Line {
        numbers: numbers,
        symbols: symbols,
    }
}

fn extract_part_numbers(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> Vec<Number> {
    let part_numbers = numbers
        .iter()
        .filter(|&number| {
            let number_pos = number.position;
            let range = (number_pos.0.saturating_sub(1))..(number_pos.1.saturating_add(2));

            for symbol in symbols {
                if range.contains(&symbol.position) {
                    return true;
                }
            }

            return false;
        })
        .map(|number| *number)
        .collect_vec();

    part_numbers
}

fn find_part_numbers(lines: Vec<Line>) -> Vec<Number> {
    let mut part_numbers: HashSet<Number> = HashSet::new();

    for idx in 0..lines.len() {
        let curr_line = &lines[idx];
        let numbers = &curr_line.numbers;

        // check previous line
        if idx > 0 {
            let prev_line_symbols = &lines[idx - 1].symbols;
            part_numbers.extend(extract_part_numbers(numbers, prev_line_symbols));
        }

        // check current line
        part_numbers.extend(extract_part_numbers(numbers, &curr_line.symbols));

        // check next line
        if idx < lines.len() - 1 {
            let next_line_symbols = &lines[idx + 1].symbols;
            part_numbers.extend(extract_part_numbers(numbers, next_line_symbols));
        }
    }

    let part_numbers = part_numbers.into_iter().collect_vec();

    part_numbers
}

fn main() {
    let input_path = get_input_path(3, Some("input.txt"));

    if 1 == 2 {}

    if let Ok(lines) = read_lines(&input_path) {
        let parsed_lines: Vec<Line> = lines
            .into_iter()
            .enumerate()
            .map(|(line_idx, line)| parse_line(&line.unwrap(), line_idx))
            .collect();

        let mut part_numbers = find_part_numbers(parsed_lines);

        part_numbers.sort_by_key(|number| (number.line, number.position.0));

        println!("{}", part_numbers.iter().join("\n"));

        println!(
            "The sum is {}",
            part_numbers
                .into_iter()
                .map(|number| number.value)
                .sum::<u32>()
        )
    }
}
