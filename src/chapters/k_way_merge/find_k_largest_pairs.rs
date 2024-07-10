use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn find_k_largest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: u32) -> Vec<[i32; 2]> {
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        for i in 0..k.min(nums1.len()) {
            for j in 0..k.min(nums2.len()) {
                if heap.len() < k {
                    heap.push(Reverse((nums1[i] + nums2[j], nums1[i], nums2[j])));
                } else if nums1[i] + nums2[j] > heap.peek().unwrap().0 .0 {
                    heap.pop();
                    heap.push(Reverse((nums1[i] + nums2[j], nums1[i], nums2[j])));
                }
            }
        }
        let mut res = Vec::with_capacity(k);
        while let Some(item) = heap.pop() {
            let (_, num1, num2) = item.0;
            res.push([num1, num2]);
        }
        res
    }
}
// #[cfg(test)]
// mod tests {

use std::collections::HashSet;

// use super::*;

#[test]
fn test_case() {
    assert_eq!(
        Solution::find_k_largest_pairs(vec![9, 8, 2], vec![6, 3, 1], 3)
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([[9, 3], [9, 6], [8, 6]].iter())
    );

    assert_eq!(
        Solution::find_k_largest_pairs(vec![5, 2, 1], vec![2, -1], 3)
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([[5, 2], [5, -1], [2, 2]].iter())
    );
}
// }
