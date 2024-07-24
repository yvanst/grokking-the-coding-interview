//! Given an array of numbers and a number k, find the median of all the k sized sub-arrays (or
//! windows) of the array.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// this problem follows the Two Heaps pattern and share similarities with Find the Median of a
/// Number Stream. we can follow a similar approach of maintaining a max-heap and a min-heap for
/// the list of numbers to find their median
///
/// the only difference is that we need to keep track of a sliding window of k numbers. this means,
/// in each iteration, when we insert a new number in the heaps, we need to remove one number from
/// the heaps which is going out of the sliding window. after the removal, we need to rebalance the
/// heaps in the same way that we did while inserting.
struct Solution {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    fn sliding_window_median(&mut self, nums: Vec<i32>, k: usize) -> Vec<f32> {
        let mut res = Vec::new();

        for (idx, &num) in nums.iter().enumerate() {
            // after push, we should have k or less than k elements
            if self.max_heap.is_empty() || self.max_heap.peek().filter(|n| **n > num).is_some() {
                self.max_heap.push(num);
            } else {
                self.min_heap.push(Reverse(num));
            }
            self.rebalance();

            // if less than k, we skip
            if self.max_heap.len() + self.min_heap.len() < k {
                continue;
            }

            if k % 2 == 0 {
                if let (Some(n1), Some(Reverse(n2))) = (self.max_heap.peek(), self.min_heap.peek())
                {
                    res.push((*n1 as f32 + *n2 as f32) / 2.0);
                }
            } else if let Some(num) = self.max_heap.peek() {
                res.push(*num as f32);
            }

            let window_head = nums[idx + 1 - k];
            // keep the two heaps to have k-1 elements for the next iteration
            self.remove_single(window_head);
        }

        res
    }

    fn rebalance(&mut self) {
        if self.max_heap.len() > self.min_heap.len() + 1 {
            if let Some(num) = self.max_heap.pop() {
                self.min_heap.push(Reverse(num));
            }
        }
        if self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(num)) = self.min_heap.pop() {
                self.max_heap.push(num);
            }
        }
    }

    fn remove_single(&mut self, element: i32) {
        let mut flag = true;
        if self.max_heap.peek().filter(|n| **n >= element).is_some() {
            self.max_heap.retain(|n| {
                if *n == element && flag {
                    flag = false;
                    false
                } else {
                    true
                }
            });
        } else {
            self.min_heap.retain(|Reverse(n)| {
                if *n == element && flag {
                    flag = false;
                    false
                } else {
                    true
                }
            });
        }
        self.rebalance();
    }
}

/// the time complexity of our algorithm is O(N*K) where N is the total number of elements in the
/// input array and K is the size of the sliding window.
/// this is due to the fact that we are going through all the N numbers and, while doing so, we are
/// doing two things:
///   1. inserting/removing numbers from heaps of size K. this will take O(logK)
///   2. removing the element going out of the sliding window, this will take O(K) as we will be
///      searching this element in an array of size K
///
/// ignoring the space needed for the ouput array, the space complexity will be O(k) because, at
/// any time, we will be storing the numbers within the sliding window.
#[test]
fn test() {
    let mut solution1 = Solution::new();
    assert_eq!(
        solution1.sliding_window_median(vec![1, 2, -1, 3, 5], 2),
        vec![1.5, 0.5, 1.0, 4.0]
    );

    let mut solution2 = Solution::new();
    assert_eq!(
        solution2.sliding_window_median(vec![1, 2, -1, 3, 5, 5, 5, 5], 3),
        vec![1.0, 2.0, 3.0, 5.0, 5.0, 5.0]
    );

    let mut solution3 = Solution::new();
    assert_eq!(
        solution3.sliding_window_median(vec![5, 5, 5, 5, 5, 5, 5], 4),
        vec![5.0, 5.0, 5.0, 5.0]
    );
}
