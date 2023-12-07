use aoc_2023::commons::io::Input;
use peg::str::LineCol;
use std::error::Error;
use std::str::FromStr;

peg::parser! {
    grammar game_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule red() -> Pull
            = count:number() " red" {
                Pull::Red(count)
            }

        rule green() -> Pull
            = count:number() " green" {
                Pull::Green(count)
            }

        rule blue() -> Pull
            = count:number() " blue" {
                Pull::Blue(count)
            }

        rule pull() -> Pull = red() / green() / blue()

        rule pulls() -> Vec<Pull> = pull() ** ", "

        rule round() -> Round = p:pulls() {
            let mut round = Round { red: 0, green: 0, blue: 0 };

            for pull in p {
                match pull {
                    Pull::Red(n) => { round.red += n },
                    Pull::Green(n) => { round.green += n },
                    Pull::Blue(n) => { round.blue += n },
                }
            }

            round
        }

        rule rounds() -> Vec<Round> = round() ** "; "

        pub rule game() -> Game
            = "Game " id:number() ": " r:rounds() {
                Game {
                    id,
                    rounds: r,
                }
            }

    }
}

pub enum Pull {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = peg::error::ParseError<LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        game_parser::game(s)
    }
}

fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for round in &game.rounds {
                red = red.max(round.red);
                green = green.max(round.green);
                blue = blue.max(round.blue);
            }

            if red <= 12 && green <= 13 && blue <= 14 {
                game.id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for round in &game.rounds {
                red = red.max(round.red);
                green = green.max(round.green);
                blue = blue.max(round.blue);
            }

            red * green * blue
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?
        .as_lines_parsed::<Game>()
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
