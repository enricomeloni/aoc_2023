use std::{borrow::BorrowMut, collections::HashMap};

use itertools::Itertools;
use utils::{get_input_path, read_lines};

fn map_char_to_value(ch: char) -> u32 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => ch.to_string().parse().unwrap(),
    }
}

enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    Pair = 1,
    High = 0,
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Hand {
    cards: String,
}

impl Hand {
    fn get_cards_count(&self) -> HashMap<char, u8> {
        let mut counts: HashMap<char, u8> = HashMap::new();

        for char in self.cards.chars() {
            if let Some(count) = counts.get_mut(&char) {
                *count = *count + 1;
            } else {
                counts.insert(char, 1);
            }
        }

        return counts;
    }

    fn get_hand_type(&self) -> HandType {
        let counts = self.get_cards_count();
        let unique_cards = counts.keys().len();
        let max_repetitions = *counts.values().max().unwrap();

        let hand_type = match unique_cards {
            5 => HandType::High,
            4 => HandType::Pair,
            3 => match max_repetitions {
                2 => HandType::TwoPair,
                3 => HandType::Three,
                _ => panic!("Should not happen"),
            },
            2 => match max_repetitions {
                4 => HandType::Four,
                3 => HandType::FullHouse,
                _ => panic!("Should not happen"),
            },
            1 => HandType::Five,
            _ => panic!("Should not happen"),
        };
        return hand_type;
    }

    fn get_hand_values(&self) -> Vec<u32> {
        self.cards
            .chars()
            .map(|ch| map_char_to_value(ch))
            .collect_vec()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_hand_type = self.get_hand_type() as u32;
        let other_hand_type = other.get_hand_type() as u32;

        if my_hand_type > other_hand_type {
            return std::cmp::Ordering::Greater;
        } else if my_hand_type < other_hand_type {
            return std::cmp::Ordering::Less;
        } else {
            let my_hand_values = self.get_hand_values();
            let other_hand_values = other.get_hand_values();

            for (my_value, other_value) in std::iter::zip(my_hand_values, other_hand_values) {
                if my_value > other_value {
                    return std::cmp::Ordering::Greater;
                } else if my_value < other_value {
                    return std::cmp::Ordering::Less;
                }
            }
            return std::cmp::Ordering::Equal;
        }
    }
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

fn main() {
    let input_path = get_input_path(7, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let mut bids = lines
            .iter()
            .map(|line| {
                let mut parts = line.split_whitespace();

                let hand = Hand {
                    cards: parts.next().unwrap().to_string(),
                };

                let bid = parts.next().unwrap().parse::<u32>().unwrap();

                Bid { hand, bid }
            })
            .collect_vec();


        bids.sort_by(|this, other| this.hand.cmp(&other.hand));

        println!("Bids: {:?}", bids);

        let mapped_bids = bids
            .iter()
            .enumerate()
            .map(|(rank, bid)| bid.bid as u64 * (rank as u64 + 1))
            .collect_vec();

        println!("Mapped bids: {:?}", mapped_bids);

        let total: u64 = mapped_bids.into_iter().sum();

        println!("Total is {}", total);
    }
}
