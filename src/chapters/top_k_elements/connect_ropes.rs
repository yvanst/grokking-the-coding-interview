//! Given ‘N’ ropes with different lengths, we need to connect these ropes into one big rope with
//! minimum cost. The cost of connecting two ropes is equal to the sum of their lengths.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// in this problem, following a greedy approach to connect the smallest ropes first will ensure the
/// lowest cost. we can use a Min Heap to find the smallest ropes following a similar approach as
/// discussed in Kth smallest number. once we connect two ropes, we need to insert the resultant
/// rope back in the Min Heap so that we can connect it with the remaining ropes.
struct Solution;

impl Solution {
    fn connect_ropes(rope_lengths: Vec<u32>) -> u32 {
        let mut heap = BinaryHeap::new();

        for length in rope_lengths {
            heap.push(Reverse(length));
        }
        let mut res = 0;
        while let Some(Reverse(len1)) = heap.pop() {
            if let Some(Reverse(len2)) = heap.pop() {
                res += len1 + len2;
                heap.push(Reverse(len1 + len2));
            } else {
                break;
            }
        }
        res
    }
}

/// given N ropes, we need O(N*logN) insert all the ropes in the heap. in each step, while
/// processing the heap, we take out two elements from the heap and insert one. this means that we
/// will have a total of N steps, having a total time complexity of O(N*logN)
///
/// the space complexity will be O(N) because we need to store all the ropes in the heap
#[test]
fn test() {
    assert_eq!(Solution::connect_ropes(vec![1, 3, 11, 5]), 33);
    assert_eq!(Solution::connect_ropes(vec![3, 4, 5, 6]), 36);
    assert_eq!(Solution::connect_ropes(vec![1, 3, 11, 5, 2]), 42);
}
