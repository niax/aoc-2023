use aoc_2023::commons::io::Input;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, error::Error};

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn rank(&self) -> usize {
        match *self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

impl PartialOrd<HandType> for HandType {
    fn partial_cmp(&self, other: &HandType) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5], // 2 = 2,.... T = 10, J = 11,
    hand_type: HandType,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            None => None,
        }
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
    for line in input.as_str().lines() {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        let mut cards = [0_u8; 5];
        let mut card_counts = HashMap::with_capacity(5);
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = if c.is_digit(10) {
                c.to_digit(10).unwrap() as u8
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

            let count = card_counts.entry(cards[i]).or_insert(0);
            *count += 1;
        }

        let hand_type = match (
            card_counts.len(),
            card_counts.values().sorted().rev().next().unwrap(),
        ) {
            (1, _) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            (5, _) => HandType::HighCard,
            _ => panic!("Dunno hand type"),
        };

        hands.push(Hand {
            bid: bid_str.parse()?,
            cards,
            hand_type,
        });
    }

    hands.sort();

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>();
    println!("{}", part1);

    Ok(())
}
