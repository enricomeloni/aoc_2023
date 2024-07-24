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

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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

impl Number {
    fn adjacent_to_symbol(self, position: usize) -> bool {
        let range = (self.position.0.saturating_sub(1))..(self.position.1.saturating_add(2));

        range.contains(&position)
    }
}

fn parse_line(line_chars: &str, line_idx: usize) -> Line {
    enum ScanElem {
        Number(Number),
        Symbol(Symbol),
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

fn extract_gear_numbers<'a>(
    prev_numbers: Option<&'a Vec<Number>>,
    numbers: &'a Vec<Number>,
    next_numbers: Option<&'a Vec<Number>>,
    gear_position: usize,
) -> Option<(Number, Number)> {
    fn get_adjacents(numbers: &Vec<Number>, position: usize) -> Vec<&Number> {
        numbers
            .iter()
            .filter(|number: &&Number| number.adjacent_to_symbol(position))
            .collect_vec()
    }
    println!("NEW LINE");

    let mut adjacents = get_adjacents(numbers, gear_position);
    println!("adjacents: {:?}", adjacents);
    if let Some(prev_numbers) = prev_numbers {
        let prev_adjacents = get_adjacents(prev_numbers, gear_position);
        println!("prev_adjacents: {:?}", prev_adjacents);
        adjacents.extend(prev_adjacents);
    }

    if let Some(next_numbers) = next_numbers {
        let next_adjacents = get_adjacents(next_numbers, gear_position);
        println!("next_adjacents: {:?}", next_adjacents);
        adjacents.extend(next_adjacents);
    }

    if adjacents.len() == 2 {
        let gear_numbers: Option<(Number, Number)> = adjacents
            .into_iter()
            .cloned()
            .collect_tuple::<(Number, Number)>();
        return gear_numbers;
    } else {
        return None;
    }
}

fn find_gear_numbers(lines: &Vec<Line>) -> Vec<(Number, Number)> {
    let gear_symbols: Vec<(usize, Symbol)> = lines
        .iter()
        .cloned()
        .enumerate()
        .flat_map(|(idx, line)| {
            line.symbols
                .into_iter()
                .filter_map(|symbol| {
                    if symbol.value == '*' {
                        return Some((idx, symbol));
                    } else {
                        return None;
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let gear_numbers = gear_symbols
        .into_iter()
        .filter_map(|(line_idx, symbol)| {
            let prev_numbers = if line_idx > 0 {
                Some(&lines.get(line_idx - 1)?.numbers)
            } else {
                None
            };
            let next_numbers = if line_idx < lines.len() - 1 {
                Some(&lines.get(line_idx + 1)?.numbers)
            } else {
                None
            };
            let numbers = &lines[line_idx].numbers;
            extract_gear_numbers(prev_numbers, numbers, next_numbers, symbol.position)
        })
        .collect_vec();

    return gear_numbers;
}

fn main() {
    println!("I am here");
    let input_path = get_input_path(3, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let parsed_lines: Vec<Line> = lines
            .into_iter()
            .enumerate()
            .map(|(line_idx, line)| parse_line(&line.unwrap(), line_idx))
            .collect();

        let mut gear_numbers = find_gear_numbers(&parsed_lines);

        gear_numbers.sort_by_key(|(a,b)| (a.line, b.line));

        println!("Gear numbers are {:?}", gear_numbers);

        let sum: u32 = gear_numbers
            .into_iter()
            .map(|(a, b)| a.value * b.value)
            .sum();

        println!("The sum is {}", sum)
    }
}
