/// this problem follows the subsets pattern and we can follow a similar breadth first
/// search(BFS) approach. the only additional thing we need to do is handle duplicate.
///
/// to handle duplicates, we will do two extra things:
/// 1. sort all numbers of the given set. this will ensure that all duplicate numbers
/// are next to each other.
/// 2. follow the same BFS approach but whenever we are about to process a duplicate,
/// instead of adding the current number to all the existing subsets, only add it to
/// the subsets which were created in the previous step.
struct Solution;

impl Solution {
    fn subsets_with_duplicates(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![vec![]];

        let mut prev_start_index = 0;
        for (idx, &n) in nums.iter().enumerate() {
            let mut subsets;
            if idx > 0 && nums[idx] == nums[idx - 1] {
                subsets = res[prev_start_index..].to_vec();
            } else {
                subsets = res.clone();
            }
            subsets.iter_mut().for_each(|s| s.push(n));

            prev_start_index = res.len();
            res.extend(subsets);
        }
        res
    }
}

#[test]
fn test() {
    // dbg!(Solution::subsets_with_duplicates(vec![1, 5, 3, 3]));
    assert_eq!(Solution::subsets_with_duplicates(vec![1, 3, 3]).len(), 6);
    assert_eq!(
        Solution::subsets_with_duplicates(vec![1, 5, 3, 3]).len(),
        12
    );
}
