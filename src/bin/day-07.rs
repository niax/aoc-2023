use aoc_2023::commons::io::Input;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, error::Error};

const FIVE_OF_A_KIND: u64 = 6;
const FOUR_OF_A_KIND: u64 = 5;
const FULL_HOUSE: u64 = 4;
const THREE_OF_A_KIND: u64 = 3;
const TWO_PAIR: u64 = 2;
const ONE_PAIR: u64 = 1;
const HIGH_CARD: u64 = 0;

#[derive(Debug)]
struct Hand {
    int_repr: u64,
    bid: u32,
}

impl Hand {
    pub fn new(cards: [u64; 5], hand_type: u64, bid: u32) -> Self {
        let x = cards[4]
            | cards[3] << 8
            | cards[2] << 16
            | cards[1] << 24
            | cards[0] << 32
            | hand_type << 48;

        Self { int_repr: x, bid }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.int_repr.eq(&other.int_repr)
    }
}
impl Eq for Hand {}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        self.int_repr.partial_cmp(&other.int_repr)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;

    let mut hands = Vec::with_capacity(1000);
    let mut part2_hands = Vec::with_capacity(1000);
    for line in input.as_str().lines() {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        let mut cards = [0_u64; 5];
        let mut part2_cards = [0_u64; 5];
        let mut card_counts = HashMap::with_capacity(5);
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = if c.is_digit(10) {
                c.to_digit(10).unwrap() as u64
            } else {
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Dunno card type"),
                }
            };

            part2_cards[i] = match cards[i] {
                11 => 1,
                _ => cards[i],
            };

            let count = card_counts.entry(cards[i]).or_insert(0);
            *count += 1;
        }

        let joker_count = *card_counts.get(&11).unwrap_or(&0);

        let sorted_card_counts = card_counts
            .iter()
            .sorted_by_key(|(_, v)| *v)
            .rev()
            .collect::<Vec<_>>();
        let hand_type = match (card_counts.len(), *sorted_card_counts[0].1) {
            (1, _) => FIVE_OF_A_KIND,
            (2, 4) => FOUR_OF_A_KIND,
            (2, 3) => FULL_HOUSE,
            (3, 3) => THREE_OF_A_KIND,
            (3, 2) => TWO_PAIR,
            (4, 2) => ONE_PAIR,
            (5, _) => HIGH_CARD,
            _ => panic!("Dunno hand type"),
        };

        let mut sorted_card_counts_without_jokers = sorted_card_counts
            .iter()
            .filter(|(k, _)| **k != 11)
            .map(|(_, v)| **v)
            .collect::<Vec<_>>();

        if sorted_card_counts_without_jokers.len() > 0 {
            sorted_card_counts_without_jokers[0] += joker_count;
        } else {
            sorted_card_counts_without_jokers.push(joker_count);
        }
        let part2_hand_type = match (
            sorted_card_counts_without_jokers.len(),
            sorted_card_counts_without_jokers[0],
        ) {
            (1, _) => FIVE_OF_A_KIND,
            (2, 4) => FOUR_OF_A_KIND,
            (2, 3) => FULL_HOUSE,
            (3, 3) => THREE_OF_A_KIND,
            (3, 2) => TWO_PAIR,
            (4, 2) => ONE_PAIR,
            (5, _) => HIGH_CARD,
            _ => panic!("Dunno hand type"),
        };

        let bid = bid_str.parse()?;
        hands.push(Hand::new(cards, hand_type, bid));
        part2_hands.push(Hand::new(part2_cards, part2_hand_type, bid));
    }

    hands.sort();
    part2_hands.sort();

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>();

    let part2 = part2_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>();
    println!("{}\n{}", part1, part2);

    Ok(())
}
