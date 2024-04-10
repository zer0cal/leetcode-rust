use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        println!("{:?}", height);
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;
        while left < right {
            println!(
                "<{} {}>, [{} {}], {}",
                left, right, height[left], height[right], result
            );
            let temp = min(height[left], height[right]) * ((right - left) as i32);
            result = max(result, temp);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn first() {
        let result = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
    }

    #[test]
    fn second() {
        let result = Solution::max_area(vec![1, 0, 0, 0, 0, 0, 0, 2, 2]);
        assert_eq!(result, 8);
    }

    #[test]
    fn one_one() {
        let result = Solution::max_area(vec![1, 1]);
        assert_eq!(result, 1);
    }
}
