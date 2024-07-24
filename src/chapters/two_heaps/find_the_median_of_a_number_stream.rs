//! Problem Statement
//!
//! Design a class to calculate the median of a number stream. The class should have the following
//! two methods:
//!
//! 1. insertNum(int num): stores the number in the class
//! 2. findMedian(): returns the median of all numbers inserted in the class
//!
//! If the count of numbers inserted in the class is even, the median will be the average of the
//! middle two numbers.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// as we know, the median is the middle value in an ordered integer list. so a brute force solution
/// could be to maintain a sorted list of all numbers inserted in the class so that we can
/// efficiently return the median whenever required. inserting a number in a sorted list will take
/// O(N) time if there are N numbers in the list
///
/// can we utilize the fact that we don't need the fully sorted list? we are only interested in
/// finding the middle element.
///
/// assume x is the median of a list. this means that half of numbers in the list will be smaller
/// than or equal to x and half will be greater than or equal to x. this leads us to an approach
/// where we can divide the list into two halves: one half to store all the smaller numbers and one
/// half to store the larger numbers.
///
/// the median of all the numbers will either be the largest number in the smallNumList or the
/// smallest number in the largeNumList. if the total number of elements is even, the median will be
/// average of these two numbers.
///
/// the best data structure that comes to mind to find the smallest or largest number among a list
/// of numbers is a Heap
///
/// 1. we can store the first half of numbers(smallNumList) in a Max Heap. we should use a Max Heap
///    as we are interested in knowing the largest number in the first half
///
/// 2. we can store the second half of numbers(largeNumList) in a Min Heap, as we interested in
///    knowing the smallest number in the second half.
///
/// inserting a number in a heap will take O(logN), which is better than the brute force approach.
/// at any time, the median of the current list of numbers can be calculated from the top element of
/// the two heaps
///
/// we can insert a number in the Max Heap if the number is smaller than the top number of the heap.
/// after every insertion, we will balance the numbers of elements in both heaps, so that they have
/// an equal number of elements. if the count of numbers is odd, let's decide to have more numbers
/// in Max Heap than the Min Heap.

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
    fn insert_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || *self.max_heap.peek().unwrap() >= num {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        if self.max_heap.len() > self.min_heap.len() + 1 {
            if let Some(num) = self.max_heap.pop() {
                self.min_heap.push(Reverse(num));
            }
        }
        if self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(num)) = self.min_heap.pop() {
                self.max_heap.push(num)
            }
        }
    }

    fn find_median(&self) -> Option<f32> {
        if self.max_heap.len() == self.min_heap.len() {
            if let (Some(num1), Some(Reverse(num2))) = (self.max_heap.peek(), self.min_heap.peek())
            {
                Some((*num1 as f32 + *num2 as f32) / 2.0)
            } else {
                None
            }
        } else {
            self.max_heap.peek().map(|n| *n as f32)
        }
    }
}


/// the time complexity of the insertNum() will be O(logN) due to the insertion in the heap. 
/// the time complexity of the findMedian() will be O(1) as we can find the median from the top
/// elements of the heaps
///
/// the space complexity will be O(N) because, at any time, we will be storing all the numbers.
#[test]
fn test() {
    let mut solution = Solution::new();
    solution.insert_num(3);
    solution.insert_num(1);
    assert_eq!(solution.find_median(), Some(2.0));

    solution.insert_num(5);
    assert_eq!(solution.find_median(), Some(3.0));

    solution.insert_num(4);
    assert_eq!(solution.find_median(), Some(3.5));
}
