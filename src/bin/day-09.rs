use std::error::Error;

use aoc_2023::commons::io::Input;

fn next_num(seq: &[isize]) -> (isize, isize) {
    if seq.iter().all(|i| *i == 0) {
        (0, 0)
    } else {
        let mut iter = seq.iter();
        let mut last = *iter.next().unwrap();
        let mut new_seq = Vec::with_capacity(seq.len());
        for i in iter {
            new_seq.push(i - last);
            last = *i;
        }
        let next = next_num(&new_seq);
        (seq[0] - next.0, last + next.1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = input.as_str();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let (p2, p1) = next_num(&nums);
        part1 += p1;
        part2 += p2;
    }

    println!("{}\n{}", part1, part2);

    Ok(())
}
