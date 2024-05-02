struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s_bytes = s.into_bytes();
        let mut res = 1;
        let mut i = 0;
        while i < s_bytes.len() - res {
            for (j, e) in s_bytes[i..].iter().enumerate() {
                if !s_bytes[i..i + j].contains(e) {
                    res = res.max(j + 1);
                } else {
                    break;
                }
            }

            i += 1;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_abcabcbb() {
        let result = Solution::length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(result, 3);
    }

    #[test]
    fn case_bbbbb() {
        let result = Solution::length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(result, 1);
    }

    #[test]
    fn case_pwwkew() {
        let result = Solution::length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(result, 3);
    }

    #[test]
    fn case_0() {
        let result = Solution::length_of_longest_substring(String::from(""));
        assert_eq!(result, 0);
    }

    #[test]
    fn case_1() {
        let result = Solution::length_of_longest_substring(String::from("a"));
        assert_eq!(result, 1);
    }

    #[test]
    fn case_aa() {
        let result = Solution::length_of_longest_substring(String::from("aa"));
        assert_eq!(result, 1);
    }

    #[test]
    fn case_aab() {
        let result = Solution::length_of_longest_substring(String::from("aab"));
        assert_eq!(result, 2);
    }

    #[test]
    fn case_abb() {
        let result = Solution::length_of_longest_substring(String::from("abb"));
        assert_eq!(result, 2);
    }
}
