//! Given a set of numbers that might contain duplicates, find all of its distinct subsets.

/// this problem follows the subsets pattern and we can follow a similar breadth first search(BFS)
/// approach. the only additional thing we need to do is handle duplicate.
///
/// to handle duplicates, we will do two extra things:
/// 1. sort all numbers of the given set. this will ensure that all duplicate numbers are next to
///    each other.
/// 2. follow the same BFS approach but whenever we are about to process a duplicate, instead of
///    adding the current number to all the existing subsets, only add it to the subsets which were
///    created in the previous step.
struct Solution;

impl Solution {
    fn subsets_with_duplicates(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut queue: Vec<Vec<i32>> = Vec::new();

        let mut prev_round_start = 0;
        for (idx, &n) in nums.iter().enumerate() {
            let mut extend_queue = if idx > 0 && nums[idx] == nums[idx - 1] {
                queue[prev_round_start..].to_vec()
            } else {
                queue.clone()
            };

            extend_queue.iter_mut().for_each(|subset| subset.push(n));

            prev_round_start = queue.len();
            queue.extend(extend_queue);
        }
        queue
    }
}

/// since, in each step, the number of subsets doubles as we add each element to all the existing
/// subsets, we will have a total of O(2^N) subsets, where N is the total number of elements in the
/// input set. and since we construct a new subset from an existing set, therefore, the time
/// complexity of the above algorithm will be O(N*2^N)
///
/// all the additional space used by our algorithm is for the output list. since, at most, we will
/// have a total of O(2^N) subsets, and each subset can take up to O(N) space, therefore, the space
/// complexity of our algorithm will be O(N*2^N).
#[test]
fn test() {
    // dbg!(Solution::subsets_with_duplicates(vec![1, 5, 3, 3]));
    assert_eq!(Solution::subsets_with_duplicates(vec![1, 3, 3]).len(), 6);
    assert_eq!(
        Solution::subsets_with_duplicates(vec![1, 5, 3, 3]).len(),
        12
    );
}
