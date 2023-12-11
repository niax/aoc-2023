use std::error::Error;

use aoc_2023::commons::io::Input;
use bitvec::prelude::*;

const GRID_SIZE: usize = 140;
const P2_GROWTH: usize = 1_000_000;

#[inline]
fn map(point: &(usize, usize), col_mappings: &[usize], row_mappings: &[usize]) -> (usize, usize) {
    (col_mappings[point.0], row_mappings[point.1])
}

#[inline]
fn solve(galaxies: &[(usize, usize)], col_mappings: &[usize], row_mappings: &[usize]) -> usize {
    let mut result = 0;
    for i in 0..galaxies.len() {
        let g1 = map(&galaxies[i], col_mappings, row_mappings);
        for j in (i + 1)..galaxies.len() {
            let g2 = map(&galaxies[j], col_mappings, row_mappings);
            let dist = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            result += dist;
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = input.as_str();

    let mut galaxies = Vec::with_capacity(GRID_SIZE * 10);
    let mut p1_row_mappings = [0usize; GRID_SIZE];
    let mut p2_row_mappings = [0usize; GRID_SIZE];

    let mut column_has_galaxy = bitarr![usize, Lsb0; 0; GRID_SIZE];

    let mut p1_mapped_row = 0;
    let mut p2_mapped_row = 0;
    for (y, line) in input.lines().enumerate() {
        let mut line_has_galaxy = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                column_has_galaxy.set(x, true);
                line_has_galaxy = true;
            }
        }
        p1_row_mappings[y] = p1_mapped_row;
        p2_row_mappings[y] = p2_mapped_row;
        if !line_has_galaxy {
            p1_mapped_row += 2;
            p2_mapped_row += P2_GROWTH;
        } else {
            p1_mapped_row += 1;
            p2_mapped_row += 1;
        }
    }

    let mut p1_col_mappings = [0usize; GRID_SIZE];
    let mut p2_col_mappings = [0usize; GRID_SIZE];
    let mut p1_mapped_col = 0;
    let mut p2_mapped_col = 0;
    for (x, has_galaxy) in column_has_galaxy.iter().enumerate() {
        if x >= GRID_SIZE {
            break;
        }
        p1_col_mappings[x] = p1_mapped_col;
        p2_col_mappings[x] = p2_mapped_col;
        if !has_galaxy {
            p1_mapped_col += 2;
            p2_mapped_col += P2_GROWTH;
        } else {
            p1_mapped_col += 1;
            p2_mapped_col += 1;
        }
    }

    let part1 = solve(&galaxies, &p1_col_mappings, &p1_row_mappings);
    let part2 = solve(&galaxies, &p2_col_mappings, &p2_row_mappings);
    println!("{}\n{}", part1, part2);

    Ok(())
}
