pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l = nums1.len() + nums2.len();
        if nums1.is_empty() {
            return match nums2.len() {
                1 => nums2[0] as f64,
                _ => Self::next(&nums2[..l / 2], &nums2[l / 2..], 0, &l, &nums2[0]),
            };
        };
        if nums2.is_empty() {
            return match nums1.len() {
                1 => nums1[0] as f64,
                _ => Self::next(&nums1[..l / 2], &nums1[l / 2..], 0, &l, &nums1[0]),
            };
        };
        Self::next(&nums1, &nums2, 0, &l, &nums1[0].min(nums2[0]))
    }

    fn next(left: &[i32], right: &[i32], deepness: i32, length: &usize, previous: &i32) -> f64 {
        println!(
            "{:?} {:?} {} {} {}",
            left, right, deepness, length, previous
        );
        let limit = length.div_ceil(2) as i32;
        if deepness != limit {
            return if left.is_empty() && right.is_empty() {
                *previous as f64
            } else if left.is_empty() {
                let r = match right.len() {
                    1 => &[],
                    _ => &right[1..],
                };
                Self::next(r, left, deepness + 1, length, &right[0])
            } else if right.is_empty() {
                Self::next(&left[1..], right, deepness + 1, length, &left[0])
            } else if left[0] < right[0] {
                let l = match left.len() {
                    1 => &[],
                    _ => &left[1..],
                };
                Self::next(l, right, deepness + 1, length, &left[0])
            } else {
                let r = match right.len() {
                    1 => &[],
                    _ => &right[1..],
                };
                Self::next(left, r, deepness + 1, length, &right[0])
            };
        }
        if length % 2 == 0 {
            return if left.is_empty() {
                (right[0] + *previous) as f64 / 2f64
            } else if right.is_empty() {
                (left[0] + *previous) as f64 / 2f64
            } else if left[0] > right[0] {
                (right[0] + *previous) as f64 / 2f64
            } else {
                (left[0] + *previous) as f64 / 2f64
            };
        }
        *previous as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_and_one() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0f64);
    }

    #[test]
    fn two_and_two_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5f64);
    }

    #[test]
    fn two_and_two_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]);
        assert_eq!(result, 2.5f64);
    }

    #[test]
    fn two_and_two_3() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]);
        assert_eq!(result, 2.5f64);
    }

    #[test]
    fn empty_and_one() {
        let result = Solution::find_median_sorted_arrays(vec![], vec![3]);
        assert_eq!(result, 3.0f64);
    }

    #[test]
    fn one_and_empty() {
        let result = Solution::find_median_sorted_arrays(vec![1], vec![]);
        assert_eq!(result, 1.0f64);
    }

    #[test]
    fn three_and_two() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4]);
        assert_eq!(result, 3.0f64);
    }

    #[test]
    fn eight_and_two() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3, 5, 6, 7, 8, 9, 10], vec![2, 4]);
        assert_eq!(result, 5.5f64);
    }

    #[test]
    fn empty_and_nine() {
        let result = Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 5.0f64);
    }

    #[test]
    fn five_and_four() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8]);
        assert_eq!(result, 5.0f64);
    }

    #[test]
    fn one_and_three() {
        let result = Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]);
        assert_eq!(result, 2.5f64);
    }
}
