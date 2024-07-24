//! Given an array of intervals, find the next interval of each interval. In a list of intervals,
//! for an interval ‘i’ its next interval ‘j’ will have the smallest ‘start’ greater than or equal
//! to the ‘end’ of ‘i’.
//!
//!Write a function to return an array containing indices of the next interval of each input
//!interval. If there is no next interval of a given interval, return -1. It is given that none of
//!the intervals have the same start point.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// a brute force solution could be to take one interval at a time and go through all the other
/// intervals to find the next interval. this algorithm will take O(N^2) where N is the total
/// number of intervals. can we do better than that?
///
/// we can utilize the Two Heaps approach. we can push all intervals into two heaps: one heap to
/// sort the intervals on maximum start time, and the other on maximum end time. we can then
/// iterate through all intervals of the maxEndHeap to find their next interval.
///
/// our algorithm will have the following steps:
/// 1. take out the top interval from the maxEndHeap to find its next interval. let's call this
///    interval topEnd
/// 2. find an interval in the maxStartHeap with the closest start greater than or equal to the
///    start of topEnd. since maxStartHeap is sorted by start of intervals, it is easy to find the
///    interval with the highest start. let's call this interval topStart.
/// 3. add the index of topStart in the result array as the next interval of topEnd. if we can't
///    find the next interval, add -1 in the result array
/// 4. put the topStart back in the maxStartHeap, as it could be the next interval of other
///    intervals.
/// 5. repeat the steps 1-4 until we have no intervals left in the maxEndHeap.
struct Solution;

impl Solution {
    fn next_interval(intervals: Vec<(i32, i32)>) -> Vec<Option<usize>> {
        // we use a pre-occupied array to store the result, thus we can change
        // the iteratative order
        let mut res = vec![None; intervals.len()];
        let mut start_heap = BinaryHeap::new();
        let mut end_heap = BinaryHeap::new();

        for (idx, interval) in intervals.iter().enumerate() {
            end_heap.push((Reverse(interval.1), idx));
            start_heap.push((Reverse(interval.0), idx));
        }

        // iterate from low to high, if a value is smaller than current value,
        // it is smaller than further value, we can safely pop from the heap
        while let Some((Reverse(end), end_idx)) = end_heap.pop() {
            while let Some((Reverse(start), start_idx)) = start_heap.pop() {
                if start >= end {
                    res[end_idx] = Some(start_idx);
                    // the current value can be used multiple times
                    start_heap.push((Reverse(start), start_idx));
                    break;
                }
            }
            // leave those couldn't be matched to default None
            if start_heap.is_empty() {
                break;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::next_interval(vec![(2, 3), (3, 4), (5, 6)]),
        vec![Some(1), Some(2), None]
    );
    assert_eq!(
        Solution::next_interval(vec![(3, 4), (1, 5), (4, 6)]),
        vec![Some(2), None, None]
    );
}
