//! Given ‘M’ sorted arrays, find the K’th smallest number among all the arrays.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// this problem follows the k-way merge pattern and we can follow a similar approach as discussed
/// in Merge K Sorted Lists.
///
/// we can start merging all the arrays, but instead of inserting numbers into a merged list, we
/// will keep count to see how many elements have been inserted in the merged list. once that count
/// is equal to K, we have found our required number.
struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    val: Reverse<i32>,
    outer_idx: usize,
    inner_idx: usize,
}

impl Solution {
    fn kth_smallest_number_in_m_sorted_lists(
        sorted_arrays: Vec<Vec<i32>>,
        k: usize,
    ) -> Option<i32> {
        let mut count_down = k;
        let mut heap = BinaryHeap::new();
        for (idx, arr) in sorted_arrays.iter().enumerate() {
            heap.push(HeapElement {
                val: Reverse(arr[0]),
                outer_idx: idx,
                inner_idx: 0,
            })
        }
        while let Some(element) = heap.pop() {
            count_down -= 1;
            if count_down == 0 {
                return Some(element.val.0);
            }
            if element.inner_idx < sorted_arrays[element.outer_idx].len() - 1 {
                heap.push(HeapElement {
                    val: Reverse(sorted_arrays[element.outer_idx][element.inner_idx + 1]),
                    outer_idx: element.outer_idx,
                    inner_idx: element.inner_idx + 1,
                })
            }
        }
        None
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::kth_smallest_number_in_m_sorted_lists(
            vec![vec![2, 6, 8], vec![3, 6, 7], vec![1, 3, 4]],
            5,
        ),
        Some(4)
    );
}
