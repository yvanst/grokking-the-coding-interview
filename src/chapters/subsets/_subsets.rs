//! Given a set with distinct elements, find all of its distinct subsets.

use std::collections::VecDeque;

/// to generate all subsets of the given set, we can use the Breadth First Search(BFS) approach. we
/// can start with an empty set, iterate through all numbers one-by-one, and add them to existing
/// sets to create new subsets.
struct Solution;

impl Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut deque = VecDeque::new();
        let mut swap_deque = VecDeque::new();
        deque.push_back(vec![]);

        // from a smaller problem, we add one new element on top of that
        for &num in nums.iter() {
            // double the number of elements in the deque; thus two push_back
            while let Some(subset) = deque.pop_front() {
                let mut add_subset = subset.clone();
                add_subset.push(num);
                swap_deque.push_back(subset);
                swap_deque.push_back(add_subset);
            }

            std::mem::swap(&mut deque, &mut swap_deque);
        }
        deque.into()
    }
}

/// since, in each step, the number of subsets doubles as we add each element to all the existing
/// subsets, therefore, we will have a total of O(2^N) subsets, where N is the total number of
/// elements in the input set. and since we counstruct a new subset from an existing set,
/// therefore, the time complexity of the above algorithm will be O(N*2^N)
///
/// all the additional space used by our algorithm is for the output list. since we will have a
/// total of O(2^N) subsets, and each subset can take up to O(N) space, therefore, the space
/// complexity of our algorithm will be O(N*2^N).
#[test]
fn test() {
    assert_eq!(Solution::subsets(vec![1, 3]).len(), 4);
    assert_eq!(Solution::subsets(vec![1, 5, 3]).len(), 8);
}
