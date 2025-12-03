// Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.
//
// The algorithm for myAtoi(string s) is as follows:
//
//     Whitespace: Ignore any leading whitespace (" ").
//
//     Signedness: Determine the sign by checking if the next character is '-' or '+',
//                 assuming positivity if neither present.
//
//     Conversion: Read the integer by skipping leading zeros until a non-digit character is
//                 encountered or the end of the string is reached. If no digits were read,
//                 then the result is 0.
//
//     Rounding:   If the integer is out of the 32-bit signed integer range [-231, 231 - 1],
//                 then round the integer to remain in the range. Specifically, integers less than
//                 -231 should be rounded to -231, and integers greater than 231 - 1 should be
//                 rounded to 231 - 1.
//
// Return the integer as the final result.

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let stripped = s.trim().char_indices();

        let mut negative = false;
        let mut output: u64 = 0;
        let base: u64 = 10;

        for c in stripped {
            if output >= i32::MAX as u64 {
                break
            }
            match c.1 {
                '-' => {
                    match c.0 {
                        0 => { negative = true }
                        _ => break
                    }
                }
                '+' => {
                    match c.0 {
                        0 => { negative = false }
                        _ => break
                    }
                }
                '0' => {
                    match output == 0 {
                        true => continue,
                        _ => output = (output * base) + (c.1 as u8 - '0' as u8) as u64
                    }
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    output = (output * base) + (c.1 as u8 - '0' as u8) as u64
                },
                _ => break
            }
        }
        println!("BIP {output}");
        if output > i32::MAX as u64 {
            return match negative {
                true => i32::MIN,
                _ => i32::MAX
            }
        }

        match negative {
            true => output as i32 * -1,
            _ => output as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let res = Solution::my_atoi("42".to_string());
        assert_eq!(res, 42);
    }
    #[test]
    fn example_2() {
        let res = Solution::my_atoi(" -042".to_string());
        assert_eq!(res, -42);
    }
    #[test]
    fn example_3() {
        let res = Solution::my_atoi("1337c0d3".to_string());
        assert_eq!(res, 1337);
    }
    #[test]
    fn example_4() {
        let res = Solution::my_atoi("-91283472332".to_string());
        assert_eq!(res, -2147483648);
    }
    #[test]
    fn example_5() {
        let res = Solution::my_atoi("9223372036854775808".to_string());
        assert_eq!(res, 2147483647);
    }
    #[test]
    fn example_6() {
        let res = Solution::my_atoi("18446744073709551617".to_string());
        assert_eq!(res, 2147483647);
    }
}
