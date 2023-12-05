use std::error::Error;

#[derive(Debug)]
pub struct Card {
    id: u32,
    matching_count: u32,
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
    let mut card_counts = Vec::new();
    card_counts.resize(input.len(), 1);

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

fn numlist_to_bitset(s: &str) -> u128 {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .map(|p| 1 << p.parse::<u32>().unwrap())
        .fold(0_u128, |acc, x| acc | x)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("inputs/04")?;
    let mmap = unsafe { memmap2::Mmap::map(&file)? };
    let s = std::str::from_utf8(&mmap)?;
    let input = s
        .lines()
        .map(|l| {
            let (card_header, card) = l.split_once(':').unwrap();
            let (_, mut card_id) = card_header.split_once(' ').unwrap();
            while card_id.chars().next().unwrap() == ' ' {
                card_id = &card_id[1..];
            }
            let (winners, picks) = card.split_once(" | ").unwrap();
            let matching_count =
                (numlist_to_bitset(winners) & numlist_to_bitset(picks)).count_ones();
            Card {
                id: card_id.parse::<u32>().unwrap() - 1,
                matching_count,
            }
        })
        .collect::<Vec<Card>>();

    println!("{}\n{}", part1(&input), part2(&input));

    Ok(())
}
