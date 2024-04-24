#![allow(dead_code)]
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sums = HashSet::new();
        nums.sort();

        for i in 1..nums.len() - 1 {
            nums[..i]
                .iter()
                .skip_while(|x| **x + nums[i] < -nums.last().unwrap())
                .take_while(|x| **x + nums[i] <= -nums[i + 1])
                .for_each(|l| {
                    if let Ok(j) = nums[i + 1..].binary_search(&(-(l + nums[i]))) {
                        sums.insert(vec![l, &nums[i], &nums[i + j + 1]]);
                    }
                });
        }
        sums.iter()
            .map(|x| x.iter().map(|y| **y).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::three_sum::Solution;

    fn check(first: Vec<Vec<i32>>, second: Vec<Vec<i32>>) -> bool {
        return first.iter().all(|x| second.contains(x))
            && second.iter().all(|x| first.contains(x));
    }

    #[test]
    fn first() {
        let res = Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]);
        let exp = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert!(check(res, exp));
    }

    #[test]
    fn second() {
        let res = Solution::three_sum(vec![-2, 0, 1, 1, 2]);
        let exp = vec![vec![-2, 0, 2], vec![-2, 1, 1]];
        assert!(check(res, exp));
    }

    #[test]
    fn zeros() {
        let res = Solution::three_sum(vec![0, 0, 0]);
        let exp = vec![vec![0, 0, 0]];
        assert!(check(res, exp));
    }
}
