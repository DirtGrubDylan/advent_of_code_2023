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

#[must_use]
pub fn extended_euclidean(left_coeff: i64, right_coeff: i64) -> (i64, i64, i64) {
    let (mut old_rem, mut rem) = (left_coeff, right_coeff);
    let (mut old_s, mut s_coeff) = (1, 0);
    let (mut old_t, mut t_coeff) = (0, 1);

    while rem != 0 {
        let quotient = old_rem / rem;

        (old_rem, rem) = (rem, old_rem - quotient * rem);
        (old_s, s_coeff) = (s_coeff, old_s - quotient * s_coeff);
        (old_t, t_coeff) = (t_coeff, old_t - quotient * t_coeff);
    }

    (old_rem, old_s, old_t)
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn min_positive_linear_diophantine(
    left_coeff: i64,
    right_coeff: i64,
    diff_i: i64,
) -> Option<(i64, i64)> {
    let (gcd_i, xg_i, yg_i) = extended_euclidean(left_coeff.abs(), right_coeff.abs());

    let (gcd, xg, yg) = ((gcd_i as f64), (xg_i as f64), (yg_i as f64));
    let diff = diff_i as f64;

    let (x0, y0) = (
        xg * (diff / gcd) * (left_coeff.signum() as f64),
        yg * (diff / gcd) * (right_coeff.signum() as f64),
    );

    let max_k_for_positive_x: i64 = if right_coeff < 0 {
        ((x0 * -gcd) / (right_coeff as f64)).floor() as i64
    } else {
        i64::MAX
    };

    let max_k_for_positive_y: i64 = ((y0 * gcd) / (left_coeff as f64)).floor() as i64;

    let max_k = max_k_for_positive_y.min(max_k_for_positive_x);

    let min_x = (x0 as i64) + (right_coeff / gcd_i) * max_k;
    let max_y = (y0 as i64) - (left_coeff / gcd_i) * max_k;

    match (min_x, max_y) {
        (0, 0) => None,
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
