#[inline]
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[inline]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let new_a = b;
        b = a % b;
        a = new_a;
    }

    a
}
