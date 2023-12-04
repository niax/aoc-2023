use aoc_2023::commons::io::load_argv_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<String>().collect::<Result<Vec<_>, _>>()?;

    let map = [
        ("1", "1", 1, true),
        ("2", "2", 2, true),
        ("3", "3", 3, true),
        ("4", "4", 4, true),
        ("5", "5", 5, true),
        ("6", "6", 6, true),
        ("7", "7", 7, true),
        ("8", "8", 8, true),
        ("9", "9", 9, true),
        ("one", "eno", 1, false),
        ("two", "owt", 2, false),
        ("three", "eerht", 3, false),
        ("four", "ruof", 4, false),
        ("five", "evif", 5, false),
        ("six", "xis", 6, false),
        ("seven", "neves", 7, false),
        ("eight", "thgie", 8, false),
        ("nine", "enin", 9, false),
    ];

    let (p1, p2) = input
        .iter()
        .map(|line| {
            let mut p1_leftmost = None;
            let mut p1_rightmost = None;
            let mut p2_leftmost = None;
            let mut p2_rightmost = None;

            'outer: for i in 0..line.len() {
                let substr = &line[i..];
                for (stringy, _, value, p1) in map {
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
                let revsubstr = line.chars().rev().skip(i).collect::<String>();
                for (_, stringy, value, p1) in map {
                    if revsubstr.starts_with(stringy) {
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

    println!("{}", p1);
    println!("{}", p2);

    Ok(())
}
