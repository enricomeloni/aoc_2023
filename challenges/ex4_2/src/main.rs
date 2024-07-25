use std::collections::HashMap;

use itertools::Itertools;
use utils::{get_input_path, read_lines};

struct Card {
    id: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>, // sorted
}

impl Card {
    fn get_n_matches(&self) -> u32 {
        let matches = self
            .winning
            .iter()
            .filter(|winning_n| self.scratched.contains(winning_n))
            .collect_vec();

        let n_matches: u32 = matches.len().try_into().unwrap();
        n_matches
    }
}

fn parse_numbers_str(numbers_str: &str) -> Vec<u32> {
    let split_numbers = numbers_str.split_whitespace().collect_vec();

    split_numbers
        .iter()
        .map(|number_str| number_str.parse::<u32>().unwrap())
        .collect_vec()
}

fn parse_line(line: &str) -> Card {
    let column_idx = line.find(':').unwrap();
    let pipe_idx = line.find('|').unwrap();

    let card_str = &line[..column_idx];
    let winning_str = &line[(column_idx + 1)..(pipe_idx - 1)].trim();
    let scratched_str = &line[(pipe_idx + 1)..].trim();

    let card_number = card_str.split(' ').last().unwrap().parse::<u32>().unwrap();
    let winning = parse_numbers_str(winning_str);
    let mut scratched = parse_numbers_str(scratched_str);

    scratched.sort();

    Card {
        id: card_number,
        winning,
        scratched,
    }
}

fn main() {
    let input_path = get_input_path(4, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let cards: Vec<Card> = lines
            .into_iter()
            .map(|line| parse_line(&line.unwrap()))
            .collect();

        let mut counts: HashMap<u32, u32> = HashMap::with_capacity(cards.len());

        for card in &cards {
            counts.insert(card.id, 1_u32);
        }

        for card in &cards { 
            let n_matches = card.get_n_matches();

            let curr_card_count = counts[&card.id];
            
            for idx in &card.id + 1..&card.id + n_matches + 1 { 
                if let Some(count) = counts.get_mut(&idx) {
                    *count += curr_card_count;
                }
            }
        }

        let total: u32 = counts.values().sum();

        println!("The grand total is {}", total)
    }
}
