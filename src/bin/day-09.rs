use std::error::Error;

use aoc_2023::commons::io::Input;

fn next_num(seq: &mut [isize]) -> (isize, isize) {
    if seq.iter().all(|i| *i == 0) {
        (0, 0)
    } else {
        let seq_len = seq.len();
        let first = seq[0];
        let mut last = seq[0];
        for i in 1..seq_len {
            let v = seq[i];
            seq[i] = v - last;
            last = v;
        }
        let next = next_num(&mut seq[1..seq_len]);
        (first - next.0, last + next.1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = input.as_str();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut nums = Vec::with_capacity(100);
    for line in input.lines() {
        nums.clear();
        for num_str in line.split_whitespace() {
            nums.push(num_str.parse().unwrap());
        }
        let (p2, p1) = next_num(&mut nums);
        part1 += p1;
        part2 += p2;
    }

    println!("{}\n{}", part1, part2);

    Ok(())
}
