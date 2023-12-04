use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Eq, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn tuple_ref(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    pub fn tuple_copy(&self) -> (T, T)
    where
        T: Copy,
    {
        (self.x, self.y)
    }
}

impl<T> Point<T>
where
    T: Default,
{
    pub fn origin() -> Point<T> {
        Point::new(T::default(), T::default())
    }
}

impl<T> Clone for Point<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Copy for Point<T> where T: Copy {}

impl<T> fmt::Debug for Point<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> Add<(T, T)> for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: (T, T)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl<T> AddAssign<(T, T)> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: (T, T)) {
        *self = Self {
            x: self.x + other.0,
            y: self.y + other.1,
        };
    }
}

impl<T> Sub<(T, T)> for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: (T, T)) -> Self {
        Self {
            x: self.x - other.0,
            y: self.y - other.1,
        }
    }
}

impl<T> SubAssign<(T, T)> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: (T, T)) {
        *self = Self {
            x: self.x - other.0,
            y: self.y - other.1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let p = Point::new(1, 1000);
        assert_eq!(&1, p.x());
        assert_eq!(&1000, p.y());
    }

    #[test]
    fn test_add_points() {
        let p1 = Point::new(1, 100);
        let p2 = Point::new(-10, 50);
        let added = p1 + p2;
        assert_eq!(&-9, added.x());
        assert_eq!(&150, added.y());
    }

    #[test]
    fn test_add_tuple() {
        let p1 = Point::new(1, 100);
        let p2 = (9, -200);
        let added = p1 + p2;
        assert_eq!(&10, added.x());
        assert_eq!(&-100, added.y());
    }
}
