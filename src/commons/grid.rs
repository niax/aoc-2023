use bitvec::prelude::*;
use lazy_static::lazy_static;
use std::cmp;
use std::collections::HashMap;

lazy_static! {
    static ref LETTERS: HashMap<u32, char> = {
        let mut h = HashMap::new();
        h.insert(529680320, 'A');
        h.insert(1067881856, 'B');
        h.insert(512103552, 'C');
        h.insert(1065752448, 'D');
        h.insert(1067882560, 'E');
        h.insert(1067616256, 'F');
        h.insert(512120256, 'G');
        h.insert(1059098560, 'H');
        h.insert(8910912, 'I');
        h.insert(33955712, 'J');
        h.insert(1059153984, 'K');
        h.insert(1057230912, 'L');
        h.insert(0, 'M');
        h.insert(1059082176, 'N');
        h.insert(512104320, 'O');
        h.insert(1066550784, 'P');
        h.insert(0, 'Q');
        h.insert(1066559040, 'R');
        h.insert(429283456, 'S');
        h.insert(0, 'T');
        h.insert(1040457600, 'U');
        h.insert(941101496, 'V');
        h.insert(0, 'W');
        h.insert(0, 'X');
        h.insert(807432752, 'Y');
        h.insert(597072960, 'Z');
        h.insert(0, ' ');
        h
    };
    static ref ADJACENT: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
}

pub trait Grid {
    type Value;
    type Coordinate;

    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn at(&self, coord: &Self::Coordinate) -> Option<&Self::Value>;
    fn set(&mut self, coord: Self::Coordinate, value: Self::Value);

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)>;
    fn from_rows(source: impl IntoIterator<Item = impl IntoIterator<Item = Self::Value>>) -> Self;
}

trait FullGrid: Grid {
    fn row_for_point(p: &Self::Coordinate) -> usize;
    fn column_for_point(p: &Self::Coordinate) -> usize;
}

pub struct ResizingBitGrid {
    values: BitVec,
    width: usize,
    height: usize,
}

impl ResizingBitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: bitvec![0; width * height],
            width,
            height,
        }
    }

    fn index_in(&self, x: usize, y: usize, width: usize, height: usize) -> Option<usize> {
        if x >= width || y >= height {
            None
        } else {
            Some(width * y + x)
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        self.index_in(x, y, self.width, self.height)
    }

    fn resize(&mut self, new_width: usize, new_height: usize) {
        let mut new_values = bitvec![0; new_width * new_height];
        for i in self.values.iter_ones() {
            let x = i % self.width;
            let y = i / self.width;
            let new_i = self
                .index_in(x, y, new_width, new_height)
                .expect("We really should have an index here");
            new_values.set(new_i, true);
        }
        self.values = new_values;
        self.height = new_height;
        self.width = new_width;
    }

    pub fn set_cell_count(&self) -> usize {
        self.values.count_ones()
    }
}

impl Grid for ResizingBitGrid {
    type Value = bool;
    type Coordinate = (usize, usize);

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn at(&self, coord: &Self::Coordinate) -> Option<&Self::Value> {
        let (x, y) = coord;
        self.index(*x, *y).map(|i| &self.values[i])
    }

    fn set(&mut self, coord: Self::Coordinate, value: Self::Value) {
        let (x, y) = coord;

        let mut idx = self.index(x, y);
        if idx.is_none() {
            let delta_x = if x >= self.width {
                x + 1 - self.width
            } else {
                0
            };
            let delta_y = if y >= self.height {
                y + 1 - self.height
            } else {
                0
            };
            let new_width = self.width.max(self.width + delta_x * 2);
            let new_height = self.height.max(self.height + delta_y * 2);
            self.resize(new_width, new_height);
            idx = self.index(x, y);
        }
        let i = idx.expect("Should now have index in range!");
        *self.values.get_mut(i).unwrap() = value
    }

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)> {
        todo!()
    }

    fn from_rows(_: impl IntoIterator<Item = impl IntoIterator<Item = Self::Value>>) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct BitGrid {
    values: BitVec,
    width: usize,
    height: usize,
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: bitvec![0; width * height],
            width,
            height,
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.width * y + x)
        }
    }

    pub fn set_cell_count(&self) -> usize {
        self.values.count_ones()
    }

    pub fn print(&self, true_val: char, false_val: char) {
        for y in 0..self.height() {
            let row = (0..self.width())
                .map(|x| {
                    if *self.at(&(x, y)).unwrap() {
                        true_val
                    } else {
                        false_val
                    }
                })
                .collect::<String>();
            println!("{}", row);
        }
    }

    pub fn decode_string(&self) -> String {
        let font_width = 5;
        let font_height = 6;
        let mut x = 0;
        let mut s = String::new();
        while (x + font_width) <= self.width() + 1 {
            let mut letter_bits = BitVec::<u32, Msb0>::with_capacity(32);
            for xd in 0..font_width {
                let x = xd + x;
                for y in 0..font_height {
                    letter_bits.push(*self.at(&(x, y)).unwrap_or(&false));
                }
            }
            let c = *LETTERS.get(&letter_bits.load::<u32>()).unwrap_or(&'?');
            s.push(c);
            x += font_width;
        }
        s
    }
}

impl Grid for BitGrid {
    type Value = bool;
    type Coordinate = (usize, usize);

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn at(&self, coord: &Self::Coordinate) -> Option<&bool> {
        let (x, y) = coord;
        self.index(*x, *y).map(|i| &self.values[i])
    }

    fn set(&mut self, coord: Self::Coordinate, value: bool) {
        let (x, y) = coord;

        match self.index(x, y) {
            Some(i) => *self.values.get_mut(i).unwrap() = value,
            None => panic!("Setting value outside of grid"),
        }
    }

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)> {
        panic!("Not implemented");
    }

    fn from_rows(_: impl IntoIterator<Item = impl IntoIterator<Item = bool>>) -> Self {
        panic!("Not implemented");
    }
}

pub struct RaycastIterator<'a, G> {
    grid: &'a G,
    step: (isize, isize),
    pos: (isize, isize),
}

impl<'a, G, T: 'a> Iterator for RaycastIterator<'a, G>
where
    G: Grid<Value = T, Coordinate = (usize, usize)>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let was = self.pos;
        self.pos = (self.pos.0 + self.step.0, self.pos.1 + self.step.1);

        let x = if was.0 >= 0 {
            was.0 as usize
        } else {
            return None;
        };
        let y = if was.1 >= 0 {
            was.1 as usize
        } else {
            return None;
        };

        self.grid.at(&(x, y))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SingleVecGrid<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> SingleVecGrid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_vecgrid(grid: VecGrid<T>) -> Self {
        let mut new_grid = SingleVecGrid::new(grid.width(), grid.height());
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let coord = (x, y);
                let v = grid.at(&coord).unwrap();
                new_grid.set(coord, v.clone());
            }
        }
        new_grid
    }

    pub fn adjacent(&self, coord: (usize, usize)) -> impl Iterator<Item = ((usize, usize), &T)> {
        ADJACENT
            .iter()
            .map(move |off| (coord.0 as isize + off.0, coord.1 as isize + off.1))
            .filter(|(x, y)| {
                *x >= 0 && *x < self.width() as isize && *y >= 0 && *y < self.height() as isize
            })
            .map(|(x, y)| {
                let coord = (x as usize, y as usize);
                (coord, self.at(&coord).unwrap())
            })
    }

    pub fn raycast(&self, from: (usize, usize), step: (isize, isize)) -> RaycastIterator<Self> {
        RaycastIterator {
            grid: self,
            step,
            pos: (from.0 as isize, from.1 as isize),
        }
    }

    pub fn north_from(&self, coord: (usize, usize)) -> impl Iterator<Item = Option<&T>> {
        let (x, y) = coord;
        (0..y).rev().map(move |y| self.at(&(x, y)))
    }

    pub fn south_from(&self, coord: (usize, usize)) -> impl Iterator<Item = Option<&T>> {
        let (x, y) = coord;
        (y + 1..self.height()).rev().map(move |y| self.at(&(x, y)))
    }

    pub fn west_from(&self, coord: (usize, usize)) -> impl Iterator<Item = Option<&T>> {
        let (x, y) = coord;
        (0..x).rev().map(move |x| self.at(&(x, y)))
    }

    pub fn east_from(&self, coord: (usize, usize)) -> impl Iterator<Item = Option<&T>> {
        let (x, y) = coord;
        (x + 1..self.width()).rev().map(move |x| self.at(&(x, y)))
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.width * y + x)
        }
    }
}

impl<T> Grid for SingleVecGrid<T>
where
    T: Default + Clone,
{
    type Value = T;
    type Coordinate = (usize, usize);

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn at(&self, coord: &Self::Coordinate) -> Option<&T> {
        let (x, y) = coord;
        self.index(*x, *y).map(|i| &self.values[i])
    }

    fn set(&mut self, coord: Self::Coordinate, value: T) {
        let (x, y) = coord;

        match self.index(x, y) {
            Some(i) => self.values[i] = value,
            None => panic!("Setting value outside of grid"),
        }
    }

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, value)| {
                let x = i % self.width;
                let y = i / self.width;
                ((x, y), value)
            })
            .collect()
    }

    fn from_rows(_: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        panic!("Not implemented");
    }
}

impl<T> Clone for SingleVecGrid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VecGrid<T> {
    rows: Vec<Vec<T>>,
    width: Option<usize>,
}

impl<T> VecGrid<T> {
    pub fn new() -> Self {
        VecGrid {
            rows: Vec::new(),
            width: None,
        }
    }

    pub fn add_row(&mut self, source: impl IntoIterator<Item = T>) {
        let row: Vec<T> = source.into_iter().collect();
        if let Some(w) = self.width {
            assert_eq!(w, row.len());
        } else {
            self.width = Some(row.len());
        }
        self.rows.push(row);
    }

    pub fn edges(&self) -> Vec<Vec<&T>> {
        if self.width.is_none() || self.rows.is_empty() {
            // There are all empty edges when we don't have any content
            vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()]
        } else {
            let top = self.rows.get(0).unwrap().iter().collect();
            let bottom = self.rows.get(self.height() - 1).unwrap().iter().collect();
            let mut right = Vec::with_capacity(self.height());
            let mut left = Vec::with_capacity(self.height());
            for r in &self.rows {
                right.push(&r[r.len() - 1]);
                left.push(&r[0]);
            }

            vec![top, right, bottom, left]
        }
    }
}

impl<T> Grid for VecGrid<T> {
    type Value = T;
    type Coordinate = (usize, usize);

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.width.unwrap_or(0)
    }

    fn at(&self, coord: &Self::Coordinate) -> Option<&T> {
        let (x, y) = coord;
        self.rows.get(*y).and_then(|row| row.get(*x))
    }

    fn set(&mut self, coord: Self::Coordinate, value: T) {
        let (x, y) = coord;
        if x >= self.width.unwrap_or(0) {
            panic!(
                "Setting value outside of grid: {} > width {:?}",
                x, self.width
            );
        }
        if y >= self.rows.len() {
            panic!(
                "Setting value outside of grid: {} > height {}",
                y,
                self.rows.len()
            );
        }

        self.rows[y][x] = value;
    }

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| ((x, y), v)))
            .collect()
    }

    fn from_rows(source: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let mut grid = VecGrid::new();
        for row in source {
            let row_vec: Vec<T> = row.into_iter().collect();
            grid.add_row(row_vec);
        }
        grid
    }
}

impl<T> Default for VecGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SparseGrid<T> {
    cells: HashMap<(isize, isize), T>,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self {
        SparseGrid {
            cells: HashMap::new(),
        }
    }

    fn key_range<F>(&self, key_fn: F) -> usize
    where
        F: Fn(isize, isize) -> isize,
    {
        if self.cells.is_empty() {
            0
        } else {
            let mut min = isize::MAX;
            let mut max = isize::MIN;
            for k in self.cells.keys() {
                let v = key_fn(k.0, k.1);
                min = cmp::min(min, v);
                max = cmp::max(max, v);
            }
            (max - min + 1) as usize
        }
    }
}

impl<T> Grid for SparseGrid<T> {
    type Value = T;
    type Coordinate = (isize, isize);

    fn height(&self) -> usize {
        self.key_range(|_, y| y)
    }

    fn width(&self) -> usize {
        self.key_range(|x, _| x)
    }

    fn at(&self, coord: &Self::Coordinate) -> Option<&T> {
        self.cells.get(coord)
    }

    fn set(&mut self, coord: Self::Coordinate, val: T) {
        self.cells.insert(coord, val);
    }

    fn points(&self) -> Vec<(Self::Coordinate, &Self::Value)> {
        self.cells.iter().map(|(coord, v)| (*coord, v)).collect()
    }

    fn from_rows(source: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let mut grid = SparseGrid::new();
        for (y, row) in source.into_iter().enumerate() {
            for (x, val) in row.into_iter().enumerate() {
                grid.set((x as isize, y as isize), val);
            }
        }
        grid
    }
}

impl<T> Default for SparseGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for SparseGrid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        SparseGrid {
            cells: self.cells.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vecgrid() {
        let grid = VecGrid::<usize>::new();

        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
    }

    #[test]
    fn simple_vecgrid() {
        let mut grid = VecGrid::new();

        grid.add_row(vec![0, 1, 2, 3]);
        grid.add_row(vec![4, 5, 6, 7]);
        grid.add_row(vec![8, 9, 10, 11]);

        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 3);

        assert_eq!(grid.at(&(0, 0)), Some(&0));
        assert_eq!(grid.at(&(1, 1)), Some(&5));
        assert_eq!(grid.at(&(2, 2)), Some(&10));
        assert_eq!(grid.at(&(4, 2)), None);

        assert_eq!(
            grid.edges(),
            vec![
                vec![&0, &1, &2, &3],
                vec![&3, &7, &11],
                vec![&8, &9, &10, &11],
                vec![&0, &4, &8],
            ]
        );

        let other_grid =
            VecGrid::from_rows(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]);
        assert_eq!(other_grid, grid);
    }

    #[test]
    fn empty_sparsegrid() {
        let grid = SparseGrid::<usize>::new();

        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
    }

    #[test]
    fn simple_sparsegrid() {
        let mut grid = SparseGrid::new();

        /*
         * 0 x x
         * x 1 x
         * x x 2
         * x x 4
         */
        grid.set((-1, -1), 0);
        grid.set((0, 0), 1);
        grid.set((1, 1), 2);
        grid.set((1, 2), 4);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 4);

        assert_eq!(grid.at(&(-1, -1)), Some(&0));
        assert_eq!(grid.at(&(1, 1)), Some(&2));
        assert_eq!(grid.at(&(1, 2)), Some(&4));
        assert_eq!(grid.at(&(4, 2)), None);
    }
}
