use aoc_2023::commons::io::load_argv_lines;
use peg::str::LineCol;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

peg::parser! {
    // Card 166: 33 18 55 57 68 79 35 40 17 53 | 93 26 55 97 80 84 44 21 15 75 11 79 83  9 50 35 78 43 39 18 17 53 42 68 86
    grammar card_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_list() -> Vec<u32>
            = number() ** (" "+)

        pub rule card() -> Card
            = "Card" (" "+) id:number() ":" (" "+) winning:number_list() " | " (" "*) picked:number_list() {
                Card {
                    id,
                    winning,
                    picked: picked.iter().copied().collect(),
                }
            }
    }
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    picked: HashSet<u32>,
}

impl Card {
    pub fn matching_count(&self) -> u32 {
        self.winning
            .iter()
            .filter(|x| self.picked.contains(x))
            .count() as u32
    }
}

impl FromStr for Card {
    type Err = peg::error::ParseError<LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        card_parser::card(s)
    }
}

fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| {
            let matching = card.matching_count();
            if matching == 0 {
                0
            } else {
                2_u32.pow(matching - 1)
            }
        })
        .sum()
}

fn part2(input: &[Card]) -> u32 {
    let mut card_counts = HashMap::new();
    for card in input {
        card_counts.insert(card.id, 1);
    }

    for card in input {
        // This card's count
        let card_count = *card_counts.get(&card.id).unwrap();
        for i in 0..card.matching_count() {
            let next_card_id = card.id + i + 1;
            let x = card_counts.get(&next_card_id).unwrap();
            card_counts.insert(next_card_id, x + card_count);
        }
    }

    card_counts.values().sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<Card>().collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
