#![allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl ListNode {
    #[inline]
    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut iter = v.iter().rev();
        let mut res = Some(Box::new(ListNode::new(*(iter.next().unwrap()))));
        for val in iter {
            res = Some(Box::new(ListNode {
                val: *val,
                next: res,
            }));
        }
        res
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut ml1, mut ml2) = (&l1, &l2);
        let mut carred = 0;
        let mut v = Vec::new();
        while let (Some(b1), Some(b2)) = (ml1, ml2) {
            let sum = b1.val + b2.val + carred;
            carred = sum / 10;
            v.push(sum % 10);
            ml1 = &b1.next;
            ml2 = &b2.next;
        }
        while let Some(b) = ml1 {
            let sum = b.val + carred;
            carred = sum / 10;
            v.push(sum % 10);
            ml1 = &b.next;
        }
        while let Some(b) = ml2 {
            let sum = b.val + carred;
            carred = sum / 10;
            v.push(sum % 10);
            ml2 = &b.next;
        }
        if carred > 0 {
            v.push(carred)
        }
        ListNode::from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn first() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let expected = ListNode::from_vec(vec![7, 0, 8]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected);
    }

    #[test]
    fn zero() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let expected = ListNode::from_vec(vec![0]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected);
    }

    #[test]
    fn second() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let expected = ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected);
    }
}
