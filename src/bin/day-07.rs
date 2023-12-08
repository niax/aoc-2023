use aoc_2023::commons::io::Input;
use std::{cmp::Ordering, collections::BinaryHeap, error::Error};

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
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        other.int_repr.cmp(&self.int_repr)
    }
}

#[inline]
fn answer(hands: &mut BinaryHeap<Hand>) -> u32 {
    let mut x = 0;
    let mut i = 0;
    while let Some(hand) = hands.pop() {
        i += 1;
        x += i * hand.bid;
    }
    x
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;

    let mut hands = BinaryHeap::with_capacity(1000);
    let mut part2_hands = BinaryHeap::with_capacity(1000);
    for line in input.as_str().lines() {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let mut cards = [0_u64; 5];
        let mut part2_cards = [0_u64; 5];
        let mut card_counts = [0_u8; 15];
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = if c.is_ascii_digit() {
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

            card_counts[cards[i] as usize] += 1;
        }

        let different_cards = card_counts.iter().filter(|x| **x > 0).count();
        let most_common_card_count = card_counts.iter().max().unwrap();
        let hand_type = match (different_cards, most_common_card_count) {
            (1, _) => FIVE_OF_A_KIND,
            (2, 4) => FOUR_OF_A_KIND,
            (2, 3) => FULL_HOUSE,
            (3, 3) => THREE_OF_A_KIND,
            (3, 2) => TWO_PAIR,
            (4, 2) => ONE_PAIR,
            (5, _) => HIGH_CARD,
            _ => panic!(
                "Dunno hand type {} {}",
                different_cards, most_common_card_count
            ),
        };

        let joker_count = card_counts[11];
        card_counts[11] = 0;

        let different_cards = card_counts.iter().filter(|x| **x > 0).count();
        let most_common_card_count = card_counts.iter().max().unwrap() + joker_count;
        let part2_hand_type = match (different_cards, most_common_card_count) {
            (0, _) => FIVE_OF_A_KIND,
            (1, _) => FIVE_OF_A_KIND,
            (2, 4) => FOUR_OF_A_KIND,
            (2, 3) => FULL_HOUSE,
            (3, 3) => THREE_OF_A_KIND,
            (3, 2) => TWO_PAIR,
            (4, 2) => ONE_PAIR,
            (5, _) => HIGH_CARD,
            _ => panic!(
                "Dunno hand type {} {}",
                different_cards, most_common_card_count
            ),
        };

        let bid = bid_str.parse()?;
        hands.push(Hand::new(cards, hand_type, bid));
        part2_hands.push(Hand::new(part2_cards, part2_hand_type, bid));
    }

    let part1 = answer(&mut hands);
    let part2 = answer(&mut part2_hands);
    println!("{}\n{}", part1, part2);

    Ok(())
}
