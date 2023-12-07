use aoc_2023::commons::io::Input;
use integer_sqrt::IntegerSquareRoot;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    pub fn record_beating_tries(&self) -> u64 {
        let common_part = (self.time.pow(2) - 4 * self.best_distance).integer_sqrt();
        let mut neg = (self.time - (common_part)).div_ceil(2);
        let mut pos = (self.time + (common_part)) / 2;

        // Because we've done things with integers, check just in case the rounding is unfortunate
        if neg * (self.time - neg) > self.best_distance {
            neg -= 1
        }
        if pos * (self.time - pos) > self.best_distance {
            pos += 1
        }

        pos - neg - 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = input.as_str();

    let mut lines = input.lines();
    let (_, time_line) = lines.next().unwrap().split_once(":").unwrap();
    let (_, distance_line) = lines.next().unwrap().split_once(":").unwrap();

    let mut races = [Race {
        time: 0,
        best_distance: 0,
    }; 16];
    for (i, (time, distance)) in std::iter::zip(
        time_line.split_whitespace(),
        distance_line.split_whitespace(),
    )
    .enumerate()
    {
        races[i].best_distance = distance.parse()?;
        races[i].time = time.parse()?;
    }

    let part1 = races
        .iter()
        .map(Race::record_beating_tries)
        .product::<u64>();

    let part2_race = Race {
        time: time_line.split_whitespace().collect::<String>().parse()?,
        best_distance: distance_line
            .split_whitespace()
            .collect::<String>()
            .parse()?,
    };
    let part2 = part2_race.record_beating_tries();

    println!("{}\n{}", part1, part2);
    Ok(())
}
