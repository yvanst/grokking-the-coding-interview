//! Given an unsorted array of numbers, find the top ‘K’ frequently occurring numbers in it.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
/// this problem follows top K numbers. the only difference is that in this problem, we need to find
/// the most frequently occurring number compared to finding the largest numbers.
///
/// we can follow the same approach as discussed in the Top K Elements problem. however, in this
/// problem, we need to know the frequency of each number, for which we can use a HashMap.
///
/// once we have the frequency map, we can use a MinHeap to find the K most frequently occurring
/// number. in this Min Heap, instead of comparing numbers we will compare their frequencies in
/// order to get frequently occurring numbers
struct Solution;

impl Solution {
    fn top_k_frequent_numbers(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let mut frequency_map = HashMap::new();
        for num in nums {
            frequency_map
                .entry(num)
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
        let mut heap = BinaryHeap::new();
        // go through all numbers of the frequency_map and push them in the minHeap, which will have
        // top k frequent numbers.
        // if the heap size is more than k, we remove the smallest number
        for (num, freq) in frequency_map.into_iter() {
            heap.push((Reverse(freq), num));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, num)| num).collect()
    }
}
/// the time complexity of the above algorithm is O(N + N*logK)
///
/// the space complexity will be O(N). even though we are storing only K numbers in the heap.
/// for the frequency map we need to store all the N numbers.
#[test]
fn test() {
    assert_eq!(
        Solution::top_k_frequent_numbers(vec![1, 3, 5, 12, 11, 12, 11], 2),
        vec![12, 11]
    );
    assert_eq!(
        Solution::top_k_frequent_numbers(vec![5, 12, 11, 3, 11], 2),
        vec![3, 11]
    );
}
