//! Given an unsorted array of numbers, find the ‘K’ largest numbers in it.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// a brute force solution could be to sort the array and return the largest k numbers. the time
/// complexity of such an algorithm will be O(N*logN) as we need to use a sorting algorithm.
///
/// the best data structure that comes to mind to keep track of top K elements is Heap.
///
/// if we iterate through the array one element at a time and keep K largest numbers in a heap such
/// that each time we find a larger number than the smallest number in the heap, we do two things:
/// 1. take out the smallest number from the heap
/// 2. insert the larger number into the heap
struct Solution;

struct HeapElement;

impl Solution {
    fn top_k_numbers(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        for num in nums {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if heap.peek().filter(|top| top.0 < num).is_some() {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.into_iter().map(|num| num.0).collect()
    }
}

/// the time complexity of this algorithm is O(K*logK + (N-K)*logK), which is asynmptotically equal
/// to O(N*logK)
///
/// the space complexity will be O(K) since we need to store the top K numbers in the heap
#[test]
fn test() {
    assert_eq!(
        Solution::top_k_numbers(vec![3, 1, 5, 12, 2, 11], 3),
        vec![5, 12, 11]
    );
    assert_eq!(
        Solution::top_k_numbers(vec![5, 12, 11, -1, 12], 3),
        vec![11, 12, 12]
    );
}
