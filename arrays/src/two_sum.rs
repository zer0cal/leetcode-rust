use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if hm.contains_key(n) {
                return vec![hm[n], i as i32];
            }
            let key = target - n;
            hm.insert(key, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum::Solution;

    #[test]
    fn test_case_1() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
