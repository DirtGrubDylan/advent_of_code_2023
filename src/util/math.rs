use std::mem;
use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(first: T, second: T) -> T
where
    T: Rem<Output = T> + PartialEq + Eq + Ord + Copy + From<u8>,
{
    let mut max = first;
    let mut min = second;

    if min > max {
        mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;

        if res == T::from(0) {
            break;
        }

        max = min;
        min = res;
    }

    min
}

pub fn lcm<T>(first: T, second: T) -> T
where
    T: Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialEq + Eq + Ord + Copy + From<u8>,
{
    first * (second / gcd(first, second))
}

pub fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;

        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_r, old_s, old_t)
}

pub fn min_positive_linear_diophantine(a: i32, b: i32, c: i32) -> Option<(i32, i32)> {
    let (gcd, xg, yg) = extended_euclidean(a.abs(), b.abs());

    let (x0, y0) = (xg * c * a.signum() / gcd, yg * c * b.signum() / gcd);

    let max_k_for_positive_y: i32 = (f64::from(y0 * gcd) / f64::from(a)).floor() as i32;

    let min_x = x0 + (b / gcd) * max_k_for_positive_y;
    let max_y = y0 - (a / gcd) * max_k_for_positive_y;

    match (min_x, max_y) {
        (x, y) if (x >= 0) && (y >= 0) => Some((x, y)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_not_1() {
        let expected = 5;

        let result = gcd(15, 20);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_gcd_1() {
        let expected = 1;

        let result = gcd(19, 20);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lcm_reduced() {
        let expected = 60;

        let result = lcm(15, 20);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lcm_not_reduced() {
        let expected = 19 * 20;

        let result = lcm(19, 20);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extended_euclidean() {
        assert_eq!(extended_euclidean(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean(25, 36), (1, 13, -9));
        assert_eq!(extended_euclidean(69, 54), (3, -7, 9));
        assert_eq!(extended_euclidean(55, 79), (1, 23, -16));
        assert_eq!(extended_euclidean(33, 44), (11, -1, 1));
        assert_eq!(extended_euclidean(50, 70), (10, 3, -2));
        assert_eq!(extended_euclidean(3, 50), (1, 17, -1));
    }

    #[test]
    fn test_min_positive_linear_diophantine() {
        assert_eq!(min_positive_linear_diophantine(3, -50, 44), Some((48, 2)));
        assert_eq!(min_positive_linear_diophantine(3, 50, 44), None);
        assert_eq!(min_positive_linear_diophantine(50, 3, 44), None);
    }
}
