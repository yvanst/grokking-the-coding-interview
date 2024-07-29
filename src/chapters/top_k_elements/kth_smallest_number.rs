//! Given an unsorted array of numbers, find Kth smallest number in it.
//!
//! Please note that it is the Kth smallest number in the sorted order, not the Kth distinct
//! element.

use std::collections::BinaryHeap;
/// this problem follows the top K numbers pattern but has two differences:
/// 1. here we need to find the Kth smallest number, whereas in top K numbers we were dealing with K
/// largest numbers.
/// 2. in this problem, we need to find only one number(Kth smallest) compared to finding all K
/// largest numbers
struct Solution;

impl Solution {
    fn kth_smallest_number(nums: Vec<i32>, k: usize) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        for num in nums {
            if heap.len() < k {
                heap.push(num)
            } else if heap.peek().filter(|top| **top > num).is_some() {
                heap.pop();
                heap.push(num);
            }
        }
        heap.pop()
    }
}

/// the time complexity of this algorithm is O(K*logK + (N-K)*logK), which is asymptotically equal
/// to O(N*logK)
/// 
/// the space complexity is O(K)
#[test]
fn test() {
    assert_eq!(
        Solution::kth_smallest_number(vec![1, 5, 12, 2, 11, 5], 3),
        Some(5)
    );
    assert_eq!(
        Solution::kth_smallest_number(vec![1, 5, 12, 2, 11, 5], 4),
        Some(5)
    );
    assert_eq!(
        Solution::kth_smallest_number(vec![5, 12, 11, -1, 12], 3),
        Some(11)
    );
}
