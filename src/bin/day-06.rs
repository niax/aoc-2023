use aoc_2023::commons::io::Input;
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = race_parser::races(input.as_str())?;

    let part1 = input.iter().map(|race| {
        let mut record_beating = 0;
        for button_hold in 0..race.time {
            let time_remaining = race.time - button_hold;
            let distance = time_remaining * button_hold;
            if distance > race.best_distance {
                record_beating += 1;
            }
        }
        record_beating
    }).product::<u32>();

    println!("{}", part1);

    let race = Race {
        time: input.iter().map(|race| race.time).fold(String::new(), |acc, i| {
            format!("{}{}", acc, i)
        }).parse().unwrap(),
        best_distance: input.iter().map(|race| race.best_distance).fold(String::new(), |acc, i| {
            format!("{}{}", acc, i)
        }).parse().unwrap(),
    };

    let mut record_beating = 0;
    for button_hold in 0..race.time {
        let time_remaining = race.time - button_hold;
        let distance = time_remaining * button_hold;
        if distance > race.best_distance {
            record_beating += 1;
        }
    }
    println!("{}", record_beating);
    Ok(())
}
