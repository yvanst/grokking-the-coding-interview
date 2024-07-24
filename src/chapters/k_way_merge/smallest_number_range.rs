//! Given ‘M’ sorted arrays, find the smallest range that includes at least one number from each of
//! the ‘M’ lists.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// this problem follows the K-way merge pattern and we can follow a similar approach as discussed
/// in Merge K Sorted Lists
///
/// we can start by inserting the first number from all the arrays in a min-heap. we will keep track
/// of the largest number that we have inserted in the heap(currentMaxNumber)
///
/// in a loop, we'll take the smallest(top) element from the min-heap and currentMaxNumber has the
/// largest element that we inserted in the heap. if these two numbers give us a smaller range,
/// we'll update our range. finally, if the array of the top element has more elements, we'll insert
/// the next element to the heap
///
/// we can finish searching the minimum range as soon as an array is completed or, in other terms,
/// the heap has less than M elements
struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    val: Reverse<i32>,
    outer_idx: usize,
    inner_idx: usize,
}

impl Solution {
    /// because the sorted array is in increasing order, we can only use min heap
    fn smallest_number_range(sorted_arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res_start = 0;
        let mut res_end = i32::MAX;

        let mut heap_max = i32::MIN;

        let mut heap = BinaryHeap::new();

        for (idx, arr) in sorted_arrays.iter().enumerate() {
            heap_max = heap_max.max(arr[0]);
            heap.push(HeapElement {
                val: Reverse(arr[0]),
                outer_idx: idx,
                inner_idx: 0,
            })
        }
        while let Some(element) = heap.pop() {
            if heap_max - element.val.0 < res_end - res_start {
                res_start = element.val.0;
                res_end = heap_max;
            }
            if element.inner_idx < sorted_arrays[element.outer_idx].len() - 1 {
                let val = sorted_arrays[element.outer_idx][element.inner_idx + 1];
                heap.push(HeapElement {
                    val: Reverse(val),
                    outer_idx: element.outer_idx,
                    inner_idx: element.inner_idx + 1,
                });
                heap_max = heap_max.max(val);
            } else {
                // notice the break is necessary here
                break;
            }
        }

        vec![res_start, res_end]
    }
}
/// since, at most, we'll be going through all the elements of all the arrays and will remove/add
/// one element in the heap in each step, the time complexity of the above algorithm will be
/// O(N*logM) where N is the total number of elements in all the M input arrays.
///
/// the space complexity will be O(M) because, at any time, our min-heap will be store one number
/// from all the M input array.
#[test]
fn test() {
    dbg!(Solution::smallest_number_range(vec![
        vec![1, 5, 8],
        vec![4, 12],
        vec![7, 8, 10]
    ]));
}
