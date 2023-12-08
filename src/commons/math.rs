use std::ops::{Div, Mul, Rem, Sub};

#[inline]
pub fn lcm<I>(a: I, b: I) -> I
where
    I: Mul<Output = I> + Div<Output = I> + Rem<Output = I> + Sub<Output = I> + PartialEq<I> + Copy,
{
    a * b / gcd(a, b)
}

#[inline]
pub fn gcd<I>(mut a: I, mut b: I) -> I
where
    I: Div<Output = I> + Rem<Output = I> + Sub<Output = I> + PartialEq<I> + Copy,
{
    let zero = b - b;
    while b != zero {
        let new_a = b;
        b = a % b;
        a = new_a;
    }

    a
}

pub trait LcmExt: Iterator {
    fn lcm(self) -> Option<Self::Item>
    where
        Self::Item: Mul<Output = Self::Item>
            + Div<Output = Self::Item>
            + Rem<Output = Self::Item>
            + Sub<Output = Self::Item>
            + PartialEq<Self::Item>
            + Copy,
        Self: Sized,
    {
        let mut result = None;
        for i in self {
            result = match result {
                Some(x) => Some(lcm(x, i)),
                None => Some(i),
            }
        }
        result
    }
}

impl<I: Iterator> LcmExt for I {}
