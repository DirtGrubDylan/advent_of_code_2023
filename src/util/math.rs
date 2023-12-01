use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(first: T, second: T) -> T
where
    T: Rem<Output = T> + PartialEq + Eq + Ord + Copy + From<u8>,
{
    let mut max = first;
    let mut min = second;

    if min > max {
        let val = max;
        max = min;
        min = val;
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
}
