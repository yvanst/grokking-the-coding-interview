//! Given an N * N matrix where each row and column is sorted in ascending order, find the Kth
//! smallest element in the matrix.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// this problem follows the K-way merge pattern and can be easily converted to Kth Smallest Number
/// in M Sorted Lists. as each row of the given matrix can be seen as a sorted list, we esstentially
/// need to find the Kth smallest number in N sorted lists
struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    val: Reverse<i32>,
    outer_idx: usize,
    inner_idx: usize,
}

impl Solution {
    fn kth_smallest_number_in_a_sorted_matrix(matrix: Vec<Vec<i32>>, k: usize) -> Option<i32> {
        let mut count_down = k;
        let mut heap = BinaryHeap::new();
        for (idx, arr) in matrix.iter().enumerate() {
            heap.push(HeapElement {
                val: Reverse(arr[0]),
                outer_idx: idx,
                inner_idx: 0,
            });
        }
        while let Some(element) = heap.pop() {
            count_down -= 1;
            if count_down == 0 {
                return Some(element.val.0);
            }
            if element.inner_idx < matrix[element.outer_idx].len() - 1 {
                heap.push(HeapElement {
                    val: Reverse(matrix[element.outer_idx][element.inner_idx + 1]),
                    outer_idx: element.outer_idx,
                    inner_idx: element.inner_idx + 1,
                })
            }
        }
        None
    }
}

/// the overall time complexity of the above algorithm will be O(N + K * logN)
///
/// the space complexity will be O(N) because our min-heap will store one number from each of the N
/// rows
#[test]
fn test() {
    assert_eq!(
        Solution::kth_smallest_number_in_a_sorted_matrix(
            vec![vec![2, 6, 8], vec![3, 7, 10], vec![5, 8, 11]],
            5,
        ),
        Some(7)
    );
}
