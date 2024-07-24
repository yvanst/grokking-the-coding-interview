//! Given a set of distinct numbers, find all of its permutations.

use std::collections::VecDeque;

/// this problem follows the subsets pattern and we can follow a similar Breadth
/// First Search approach, however, unlike subsets, every permutation must contain
/// all the numbers
///
/// how can we generate permutations in the 4th step from the permutations of the
/// 3rd step?
/// if we look closely, we will realize that when we add a new number, we take each
/// permutation of the previous step and insert the new number in every position
/// to generate teh new permutation
struct Solution;

impl Solution {
    fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut deque = VecDeque::new();
        let mut swap_deque = VecDeque::new();
        deque.push_back(vec![]);

        for &n in nums.iter() {
            while let Some(subset) = deque.pop_front() {
                // notice the + 1 here
                for idx in 0..(subset.len() + 1) {
                    let mut cur_subset = subset.clone();
                    cur_subset.insert(idx, n);
                    // notice only push back the modified value, thus n! rather than 2^N
                    swap_deque.push_back(cur_subset);
                }
            }
            std::mem::swap(&mut deque, &mut swap_deque);
        }

        deque.into()
    }
}

/// we know that there are a total of N! permutations of a set with N numbers. in the algorithm
/// above, we are iterating through all of these permutations with the help of the two for loops.
/// in each iteration, we go through all the current permutations to insert a new number in them.
/// to insert a number into a permutation of size N will take O(N), which makes the overall time
/// complexity of our algorithm O(N*N!)
///
/// all the additional space used by our algorithm is for the result list and the queue to store
/// the intermediate permutations. at any time, we don't have more than N! permutations between the
/// result list and the queue. therefore the overall space complexity to store N! permutations each
/// containing N elements will be O(N*N!).
#[test]
fn test() {
    // dbg!(Solution::permutations(vec![1, 3, 5]));
    assert_eq!(Solution::permutations(vec![1, 3, 5]).len(), 6);
}
