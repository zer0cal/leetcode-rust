pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l = nums1.len() + nums2.len();
        if nums1.is_empty() {
            if l == 1 {
                return nums2[0] as f64;
            }
            return Self::next(&nums2[..l / 2], &nums2[l / 2..], 0, &l, &nums2[0]);
        };
        if nums2.is_empty() {
            if l == 1 {
                return nums1[0] as f64;
            }
            return Self::next(&nums1[..l / 2], &nums1[l / 2..], 0, &l, &nums1[0]);
        };
        Self::next(&nums1, &nums2, 0, &l, &nums1[0])
    }

    fn next(a: &[i32], b: &[i32], i: i32, len: &usize, last: &i32) -> f64 {
        if i != len.div_ceil(2) as i32 {
            return if a.len() == 1 {
                println!("a.l=1,\t{},\t{},\t{:?}\t{:?}", i, &a[0].min(b[0]), a, b);
                Self::next(b, a, i + 1, len, &a[0].min(b[0]))
            } else if b.len() == 1 {
                println!("b.l=1,\t{},\t{},\t{:?}\t{:?}", i, &a[1].min(b[0]), a, b);
                Self::next(&a[1..], b, i + 1, len, &a[1].min(b[0]))
            } else if a[0] < b[0] {
                println!("a < b,\t{},\t{},\t{:?}\t{:?}", i, a[0], a, b);
                Self::next(&a[1..], b, i + 1, len, &a[0])
            } else {
                println!("a > b,\t{},\t{},\t{:?}\t{:?}", i, b[0], a, b);
                Self::next(a, &b[1..], i + 1, len, &b[0])
            };
        }
        if len % 2 == 0 {
            println!("even,\t{},\t{},\t{:?}\t{:?}", i, last, a, b);
            return if a[0] < b[0] {
                (a[0] + *last) as f64 / 2f64
            } else {
                (b[0] + *last) as f64 / 2f64
            };
        }
        println!("odd,\t{},\t{},\t{:?}\t{:?}", i, last, a, b);
        // if a[0] < b[0] {
        //     return a[0] as f64;
        // }
        // b[0] as f64
        *last as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0f64);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5f64);
    }
}
