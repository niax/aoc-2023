use aoc_2023::commons::io::Input;
use std::error::Error;
use integer_sqrt::IntegerSquareRoot;

peg::parser! {
    grammar race_parser() for str {
        rule fileend() -> () = ("\n" / "");

        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_list() -> Vec<u64>
            = number() ** (" "+)

        pub rule races() -> Vec<Race>
            = "Time:" " "+ times:number_list() "\n"
              "Distance:" " "+ distances:number_list() fileend() {
                  let mut races = Vec::with_capacity(times.len());
                  for (time, best_distance) in std::iter::zip(times.iter(), distances.iter()) {
                      races.push(Race{
                          time:*time, 
                          best_distance: *best_distance,
                      });
                  }
                  races
          }
    }
}

#[derive(Debug)]
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

fn fold_num(acc: u64, i: u64) -> u64 {
    acc * (10_u64.pow(i.ilog10() + 1)) + i
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = race_parser::races(input.as_str())?;

    let part1 = input.iter().map(Race::record_beating_tries).product::<u64>();

    let part2_race = Race {
        time: input.iter().map(|race| race.time).fold(0, fold_num),
        best_distance: input.iter().map(|race| race.best_distance).fold(0, fold_num),
    };
    let part2 = part2_race.record_beating_tries();

    println!("{}\n{}", part1, part2);
    Ok(())
}
