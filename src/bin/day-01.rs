use aoc_2023::commons::io::Input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;

    let map = [
        ("1", 1, true),
        ("2", 2, true),
        ("3", 3, true),
        ("4", 4, true),
        ("5", 5, true),
        ("6", 6, true),
        ("7", 7, true),
        ("8", 8, true),
        ("9", 9, true),
        ("one", 1, false),
        ("two", 2, false),
        ("three", 3, false),
        ("four", 4, false),
        ("five", 5, false),
        ("six", 6, false),
        ("seven", 7, false),
        ("eight", 8, false),
        ("nine", 9, false),
    ];

    let (p1, p2) = input
        .as_str()
        .lines()
        .map(|line| {
            let mut p1_leftmost = None;
            let mut p1_rightmost = None;
            let mut p2_leftmost = None;
            let mut p2_rightmost = None;

            'outer: for i in 0..line.len() {
                let substr = &line[i..];
                for (stringy, value, p1) in map {
                    if substr.starts_with(stringy) {
                        if p1 {
                            p1_leftmost = Some(value)
                        }
                        if p2_leftmost.is_none() {
                            p2_leftmost = Some(value)
                        }
                        if p1_leftmost.is_some() && p2_leftmost.is_some() {
                            break 'outer;
                        }
                    }
                }
            }

            'outer: for i in 0..line.len() {
                let substr = &line[..(line.len() - i)];
                for (stringy, value, p1) in map {
                    if substr.ends_with(stringy) {
                        if p1 {
                            p1_rightmost = Some(value)
                        }
                        if p2_rightmost.is_none() {
                            p2_rightmost = Some(value)
                        }
                        if p1_rightmost.is_some() && p2_rightmost.is_some() {
                            break 'outer;
                        }
                    }
                }
            }

            (
                p1_leftmost.unwrap() * 10 + p1_rightmost.unwrap(),
                p2_leftmost.unwrap() * 10 + p2_rightmost.unwrap(),
            )
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("{}\n{}", p1, p2);

    Ok(())
}
