use itertools::Itertools;
use utils::{get_input_path, read_lines};

struct Card {
    id: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>, // sorted
}

impl Card {
    fn get_points(&self) -> u32 {
        let matches = self
            .winning
            .iter()
            .filter(|winning_n| self.scratched.contains(winning_n))
            .collect_vec();

        let n_matches: u32 = matches.len().try_into().unwrap();
        return if n_matches > 0 {
            2_u32.pow(n_matches - 1)
        } else {
            0
        };
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

        let points: u32 = cards.iter().map(|card| card.get_points()).sum();

        println!("You won {} points", points)
    }
}
