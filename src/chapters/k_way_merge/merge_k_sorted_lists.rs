//! Given an array of ‘K’ sorted arrays, merge them into one sorted array.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// a brute force could be to add all elements of the given K lists to one list and sort it. if
/// there are a total of N elements in all the input lists, then the brute force solution will have
/// a time complexity of O(N*logN) as we will need to sort the merged list. can we do better than
/// this? how can we utilize the fact that the input lists are individually sorted?
///
/// if we have to find the smallest element of all the input lists, we have to compare only the
/// smallest(i.e. the first) element of all the lists. once we have the smallest element, we can
/// put it in the merged list. following a simialr pattern, we can then find the next smallest
/// element of all the lists to add it to the merged list.
///
/// let's see how we can use a heap to find a better algorithm
/// 1. we can insert the first element of each array in a Min Heap.
/// 2. after this, we can take out the smallest element from the heap and add it to the merged
///    list.
/// 3. after removing the smallest element from the heap, we can insert the next element of the
///    same list into the heap.
/// 4. we can repeat steps 2 and 3 to populate the merged list in sorted order.
struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    val: Reverse<i32>,
    outer_idx: usize,
    inner_idx: usize,
}

impl Solution {
    fn merge_k_sorted_lists(sorted_arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut heap = BinaryHeap::new();

        for (idx, arr) in sorted_arrays.iter().enumerate() {
            heap.push(HeapElement {
                val: Reverse(arr[0]),
                outer_idx: idx,
                inner_idx: 0,
            });
        }
        while let Some(element) = heap.pop() {
            res.push(element.val.0);
            if element.inner_idx < sorted_arrays[element.outer_idx].len() - 1 {
                heap.push(HeapElement {
                    val: Reverse(sorted_arrays[element.outer_idx][element.inner_idx + 1]),
                    outer_idx: element.outer_idx,
                    inner_idx: element.inner_idx + 1,
                })
            }
        }

        res
    }
}

/// since we'll be going through all the elements of all lists and will be removing/adding one
/// element to the heap in each step, the time complexity of the above algorithm will be O(N*logK),
/// where N is the total number of elements in all the K input lists.
///
/// the space complexity will be O(K), because, at any time, our min-heap will be storing one
/// number from all the K input lists.
#[test]
fn test() {
    assert_eq!(
        Solution::merge_k_sorted_lists(vec![vec![2, 6, 8], vec![3, 6, 7], vec![1, 3, 4]]),
        vec![1, 2, 3, 3, 4, 6, 6, 7, 8,]
    );
}
