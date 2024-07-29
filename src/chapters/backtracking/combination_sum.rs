//! Given an array of distinct positive integers candidates and a target integer target, return a
//! list of all unique combinations of candidates where the chosen numbers sum to target. You may
//! return the combinations in any order.
//!
//! The same number may be chosen from candidates an unlimited number of times. Two combinations are
//! unique if the frequency of at least one of the chosen numbers is different.

/// what backtrack means is that we set a list as our answer, and iterate the candidate array,
/// adding each candidate to the answer and recursively call the function to add more candidate
/// and update the remaining target, if the target becomes 0, we add current solution to the
/// result list, if the target becomes negative, we backtrack and remove the last added candidate
/// from the list.
///
/// we don't use backtrack to return anything, we only use side effect to push current_result to
/// final_results, that's the information the recursive function should give us
struct Solution;

impl Solution {
    fn combination_sum(candidates: Vec<u32>, target: u32) -> Vec<Vec<u32>> {
        let mut res = Vec::new();
        let mut cur_res = Vec::new();

        Self::backtrack(&candidates, target, 0, &mut cur_res, &mut res);
        res
    }
    fn backtrack(
        candidates: &[u32],
        target: u32,
        index: usize,
        cur_res: &mut Vec<u32>,
        res: &mut Vec<Vec<u32>>,
    ) {
        if target == 0 {
            res.push(cur_res.to_vec());
            return;
        }

        // combination v.s. permutation -> both for loop traverse the nums from front to end
        // but the permutation should use insert to simulate another for loop for all positions
        //
        // since we allow for count multiple times for one item, we include the current index in the
        // next recursion
        for i in index..candidates.len() {
            let candidate = candidates[i];
            if candidate <= target {
                cur_res.push(candidate);
                Self::backtrack(candidates, target - candidate, i, cur_res, res);
                cur_res.pop();
            }
        }
    }
}

/// this algorithm has a time complexity of O(N^(T/M+1)), where N is the total number of elements in
/// the candidates array, T is the target value, and M is the smallest value among the candidates.
/// this is because the execution of the backtracking is similar to a DFS traversal of an n-ary
/// tree. so the time complexity would be the same as the number of nodes in the n-ary tree.
/// each node can call the backtrack function a maximum of N times, i.e., the total number of
/// candidates. the maximal depth of the n-ary tree would be T/M, where we keep on adding the
/// smallest element to the combination.
/// as we know, the maximal number of nodes in N-ary tree of T/M height would be N^(T/M+1), hence
/// the time complexity is O(N^(T/M+1))
///
/// ignoring the space needed for the output array, the space complexity will be O(T/M) because at
/// any time, we can pile up to T/M recursive calls of the backtrack function.
#[test]
fn test() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );

    assert_eq!(
        Solution::combination_sum(vec![2, 4, 6, 8], 10),
        vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 4],
            vec![2, 2, 6],
            vec![2, 4, 4],
            vec![2, 8],
            vec![4, 6]
        ]
    );
}
