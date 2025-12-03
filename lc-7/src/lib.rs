// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1],
// then return 0.
//
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

pub struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let negative = x < 0;
        let x_norm = if negative {x * -1} else {x};
        let str_res = format!("{x_norm}");
        let output_str: String = str_res.chars().rev().collect();
        println!("{output_str}");
        let mut parsed = output_str.parse::<i32>().unwrap_or(0);
        if negative {
            parsed *= -1
        }
        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let res = Solution::reverse(123);
        assert_eq!(res, 321);
    }
    #[test]
    fn example_2() {
        let res = Solution::reverse(-123);
        assert_eq!(res, -321);
    }
    #[test]
    fn example_3() {
        let res = Solution::reverse(120);
        assert_eq!(res, 21);
    }
}
