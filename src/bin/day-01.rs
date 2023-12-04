use aoc_2023::commons::io::load_argv_lines;
use std::error::Error;
use std::collections::HashMap;

fn part1(input: &Vec<String>) -> u32 {
    let map = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    solution(input, map)
}

fn part2(input: &Vec<String>) -> u32 {
    let map = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    solution(input, map)
}

fn solution(input: &Vec<String>, map: HashMap<&str, u32>) -> u32 {
    input.iter().map(|line| {
        let mut leftmost_idx = usize::MAX;
        let mut leftmost_value = None;
        let mut rightmost_idx = usize::MIN;
        let mut rightmost_value = None;
        for opt in map.keys() {
            if let Some(x) = line.find(opt) {
                if x < leftmost_idx {
                    leftmost_idx = x;
                    leftmost_value = Some(map[opt]);
                }
            }
            if let Some(x) = line.rfind(opt) {
                if x >= rightmost_idx {
                    rightmost_idx = x;
                    rightmost_value = Some(map[opt]);
                }
            }
        }

        leftmost_value.unwrap() * 10 + rightmost_value.unwrap()
    }).sum()
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<String>().collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
