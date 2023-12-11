use std::error::Error;

use aoc_2023::commons::io::Input;
use aoc_2023::commons::grid::{Grid, SingleVecGrid, BitGrid};

// Never eat shredded wheat (it's naaasty)
#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    #[inline]
    pub fn step(&self, pos: (isize, isize)) -> (isize, isize) {
        let delta = match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        };

        (pos.0 + delta.0, pos.1 + delta.1)
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
enum Cell {
    #[default]
    Ground,
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    StartingPosition,
}

impl Cell {
    #[inline]
    pub fn step(&self, inbound_dir: Direction) -> Option<Direction> {
        match (self, inbound_dir) {
            (Self::VerticalPipe, Direction::North) => Some(Direction::North),
            (Self::VerticalPipe, Direction::South) => Some(Direction::South),
            (Self::HorizontalPipe, Direction::East) => Some(Direction::East),
            (Self::HorizontalPipe, Direction::West) => Some(Direction::West),
            (Self::NorthEastBend, Direction::South) => Some(Direction::East),
            (Self::NorthEastBend, Direction::West) => Some(Direction::North),
            (Self::NorthWestBend, Direction::South) => Some(Direction::West),
            (Self::NorthWestBend, Direction::East) => Some(Direction::North),
            (Self::SouthWestBend, Direction::North) => Some(Direction::West),
            (Self::SouthWestBend, Direction::East) => Some(Direction::South),
            (Self::SouthEastBend, Direction::North) => Some(Direction::East),
            (Self::SouthEastBend, Direction::West) => Some(Direction::South),
            _ => None,
        }
    }

    #[inline]
    pub fn can_go_north(&self) -> bool {
        match self {
            Self::VerticalPipe | Self::NorthEastBend | Self::NorthWestBend => true,
            _ => false,
        }
    }
}

const GRID_SIZE: usize = 140;

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let input = input.as_str();

    let mut starting = (0, 0);
    let mut grid = SingleVecGrid::new(GRID_SIZE, GRID_SIZE);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '|' => Cell::VerticalPipe,
                '-' => Cell::HorizontalPipe,
                'L' => Cell::NorthEastBend,
                'J' => Cell::NorthWestBend,
                '7' => Cell::SouthWestBend,
                'F' => Cell::SouthEastBend,
                'S' => Cell::StartingPosition,
                '.' => Cell::Ground,
                _ => panic!("NOPE!"),
            };

            if cell == Cell::StartingPosition {
                starting = (x,y);
            }
            grid.set((x, y), cell);
        }
    }

    let mut dir = Direction::North;
    for d in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let next_pos = d.step((starting.0 as isize, starting.1 as isize));
        let cell = grid.at(&(next_pos.0 as usize, next_pos.1 as usize));
        if cell.is_some() && cell.unwrap().step(d).is_some() {
            dir = d;
            break;
        }
    }

    let mut pipe_grid = BitGrid::new(GRID_SIZE, GRID_SIZE);
    let mut pos = (starting.0 as isize, starting.1 as isize);
    pos = dir.step(pos);
    let mut pipe_len = 0u32;
    while let Some(cell) = grid.at(&(pos.0 as usize, pos.1 as usize)) {
        pipe_grid.set((pos.0 as usize, pos.1 as usize), true);
        if cell == &Cell::StartingPosition {
            break;
        }
        dir = cell.step(dir).unwrap();
        pos = dir.step(pos);
        pipe_len += 1;
    }
    let part1 = pipe_len.div_ceil(2);


    let mut inside_count = 0;
    let mut inside_grid = BitGrid::new(GRID_SIZE, GRID_SIZE);
    for y in 0..pipe_grid.height() {
        let mut inside = false;
        for x in 0..pipe_grid.width() {
            if *pipe_grid.at(&(x, y)).unwrap() {
                if grid.at(&(x, y)).unwrap().can_go_north()  {
                    inside = !inside;
                }
            } else if inside {
                inside_grid.set((x, y), true);
                inside_count += 1;
            }
        }
    }

    println!("{}\n{}", part1, inside_count); 

    Ok(())
}
