// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
//
// P   A   H   N
// A P L S I I G
// Y   I   R
//
// And then read line by line: "PAHNAPLSIIGYIR"
//
// Write the code that will take a string and make this conversion given a number of rows:
//
// string convert(string s, int numRows);
//
// Constraints:
//
//     1 <= s.length <= 1000
//     s consists of English letters (lower-case and upper-case), ',' and '.'.
//     1 <= numRows <= 1000

pub struct Solution {}

impl Solution {
    fn stamp(data: &mut Vec<Vec<u8>>, dest: (usize, usize), to_stamp: u8) {
        while data.len() < dest.0 {
            data.push(Vec::new());
        }
        let row_mut: &mut Vec<u8> = data.get_mut(dest.1).unwrap();
        row_mut.push(to_stamp)
    }

    fn stringify(data: Vec<Vec<u8>>) -> String {
        let mut result = String::new();

        for row in data {
            let row_string: String = String::from_utf8(row).unwrap();
            result += &row_string;
        }

        result
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s
        }
        let mut rows: Vec<Vec<u8>> = Vec::new();
        for _ in 0..num_rows {
            rows.push(Vec::new());
        }

        let mut diagonal = false;

        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut mag: Vec<u8> = s.clone().chars().map(|c| {c as u8}).collect();

        while ! mag.is_empty() {
            Solution::stamp(&mut rows, (x, y), mag.remove(0));
            match diagonal {
                false => y += 1,
                _ => {
                    x += 1;
                    y -= 1;
                }
            }
            if y == 0 || y == (num_rows - 1) as usize {
                diagonal = !diagonal;
            }
        }

        Solution::stringify(rows)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "PAYPALISHIRING".into();
        let num_rows = 3;
        let output = Solution::convert(s, num_rows);

        assert_eq!(output, "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn example_2() {
        let s: String = "PAYPALISHIRING".into();
        let num_rows = 4;
        let output = Solution::convert(s, num_rows);

        assert_eq!(output, "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn example_3() {
        let s: String = "A".into();
        let num_rows = 1;
        let output = Solution::convert(s, num_rows);

        assert_eq!(output, "A".to_string());
    }

    #[test]
    fn example_4() {
        let s: String = "AB".into();
        let num_rows = 1;
        let output = Solution::convert(s, num_rows);

        assert_eq!(output, "AB".to_string());
    }

}
