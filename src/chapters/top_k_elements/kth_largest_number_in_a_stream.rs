//! Design a class to efficiently find the Kth largest element in a stream of numbers.
//!
//! The class should have the following two things:
//! 1. The constructor of the class should accept an integer array containing initial numbers from
//! the stream and an integer ‘K’.
//! 2. The class should expose a function add(int num) which will store the given number and return
//! the Kth largest number.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl Solution {
    fn kth_largest_number_in_a_stream(nums: Vec<i32>, k: usize) -> Self {
        let mut heap = BinaryHeap::new();
        for num in nums {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if heap.peek().filter(|Reverse(top)| *top < num).is_some() {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        Self { heap, k }
    }

    fn add(&mut self, num: i32) -> Option<i32> {
        self.heap.push(Reverse(num));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().map(|r| r.0)
    }
}
/// the time complexity of the add() function will be O(logK) since we are inserting the new number
/// in the heap.
///
/// the space complexity will be O(k) for sorting numbers in the heap.
#[test]
fn test() {
    let mut solution = Solution::kth_largest_number_in_a_stream(vec![3, 1, 5, 12, 2, 11], 4);
    assert_eq!(solution.add(6), Some(5));
    assert_eq!(solution.add(13), Some(6));
    assert_eq!(solution.add(4), Some(6));
}
