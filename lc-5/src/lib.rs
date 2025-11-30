// Given a string s, return the longest
// in s.
//
//
// Constraints:
//
//     1 <= s.length <= 1000
//     s consist of only digits and English letters.
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    fn is_palindrome(s: &str) -> bool {
        if s.len() == 1 {
            return true
        }
        let mut i: usize = 0;
        let mut j: usize = s.len() - 1;
        let s_bytes = s.as_bytes();

        while i < j {
            if s_bytes[i] != s_bytes[j] {
                return false
            }
            i += 1;
            j -= 1;
        }
        true
    }


    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s
        }
        let mut i: usize = 0;

        let mut longest = String::new();
        let mut substrings: HashSet<String> = HashSet::new();
        while i < s.len() {
            let mut j = i;
            while j < s.len() {
                let this_slice = s[i..j + 1].to_string();
                if this_slice.as_bytes()[0] == this_slice.as_bytes()[this_slice.len() - 1] {
                    substrings.insert(this_slice);
                }
                j += 1;
            }
            i += 1;
        }

        for substring in substrings {
            if Self::is_palindrome(&substring) {
                if substring.len() >= longest.len() {
                    longest = substring
                }
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_1() {
        let s = "babad";

        let pal =  Solution::longest_palindrome(s.into());

        assert!(pal == "bab" || pal == "aba")
    }

    #[test]
    fn ex_2() {
        let s = "cbbd";

        let pal =  Solution::longest_palindrome(s.into());

        assert_eq!(pal, "bb");
    }

    #[test]
    fn ex_3() {
        let s = "a";

        let pal =  Solution::longest_palindrome(s.into());

        assert_eq!(pal, "a");
    }

    #[test]
    fn ex_4() {
        let s = "ac";

        let pal =  Solution::longest_palindrome(s.into());

        assert!(pal == "a" || pal == "c")
    }

    #[test]
    fn ex_5() {
        let s = "klvxwqyzugrdoaccdafdfrvxiowkcuedfhoixzipxrkzbvpusslsgfjocvidnpsnkqdfnnzzawzsslwnvvjyoignsfbxkgrokzyusxikxumrxlzzrnbtrixxfioormoyyejashrowjqqzifacecvoruwkuessttlexvdptuvodoavsjaepvrfvbdhumtuvxufzzyowiswokioyjtzzmevttheeyjqcldllxvjraeyflthntsmipaoyjixygbtbvbnnrmlwwkeikhnnmlfspjgmcxwbjyhomfjdcnogqjviggklplpznfwjydkxzjkoskvqvnxfzdrsmooyciwulvtlmvnjbbmffureoilszlonibbcwfsjzguxqrjwypwrskhrttvnqoqisdfuifqnabzbvyzgbxfvmcomneykfmycevnrcsyqclamfxskmsxreptpxqxqidvjbuduktnwwoztvkuebfdigmjqfuolqzvjincchlmbrxpqgguwuyhrdtwqkdlqidlxzqktgzktihvlwsbysjeykiwokyqaskjjngovbagspyspeghutyoeahhgynzsyaszlirmlekpboywqdliumihwnsnwjc";

        let pal =  Solution::longest_palindrome(s.into());

        assert_eq!(pal, "wnsnw");
    }
}
