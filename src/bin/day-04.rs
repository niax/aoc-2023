use peg::str::LineCol;
use std::error::Error;
use std::str::FromStr;

peg::parser! {
    // Card 166: 33 18 55 57 68 79 35 40 17 53 | 93 26 55 97 80 84 44 21 15 75 11 79 83  9 50 35 78 43 39 18 17 53 42 68 86
    grammar card_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_set() -> u128
            = nums:number() ** (" "+) {
                let mut x = 0;
                for num in nums {
                    x |= (1 << num);
                }
                x
            }

        pub rule card() -> Card
            = "Card" (" "+) id:number() ":" (" "+) winning:number_set() " | " (" "*) picked:number_set() {
                Card {
                    id,
                    matching_count: (winning & picked).count_ones(),
                }
            }

        rule card_list() -> Vec<Card> = c:card() ** ("\n")

        pub rule cards() -> Vec<Card> = c:card_list() "\n" {
            c
        }
    }
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    matching_count: u32,
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
            if card.matching_count > 0 {
                1 << (card.matching_count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &[Card]) -> u32 {
    let mut card_counts = Vec::with_capacity(input.len() + 1);
    card_counts.push(0_u32);
    for _ in input {
        card_counts.push(1_u32);
    }

    for card in input {
        // This card's count
        let card_count = card_counts[card.id as usize];
        for i in 0..card.matching_count {
            let next_card_id = card.id + i + 1;
            card_counts[next_card_id as usize] += card_count;
        }
    }

    card_counts.iter().sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("inputs/04")?;
    let mmap = unsafe { memmap2::Mmap::map(&file)? };
    let s = std::str::from_utf8(&mmap)?;
    let input = card_parser::cards(s)?;

    println!("{}\n{}", part1(&input), part2(&input));

    Ok(())
}
