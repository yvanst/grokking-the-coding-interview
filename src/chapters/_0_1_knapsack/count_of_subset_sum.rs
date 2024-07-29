//! Given a set of positive numbers, find the total number of subsets whose sum is equal to a given
//! number S.

/// this problem follows the 0/1 knapsack pattern and is quite simialr to Subset Sum. the only
/// difference in this problem is that we need to count the number of subsets, whereas in Subset Sum
/// we only wanted to know if a subset with the given sum existed.
///
/// a basic brute-force solution could be to try all subsets of the given numbers to count the
/// subsets that have sum equal to S
struct SolutionBruteForce;

impl SolutionBruteForce {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        Self::dfs(&nums, 0, sum)
    }

    /// think of tree structure when simulating the process,
    /// all the leaf node should be either 0 or 1
    /// but we can have mid node that is greater than 1
    fn dfs(nums: &[u32], index: usize, sum: u32) -> u32 {
        if sum == 0 {
            return 1;
        }
        if index >= nums.len() {
            return 0;
        }

        // left branch case, take the value
        let count1 = if sum >= nums[index] {
            Self::dfs(nums, index + 1, sum - nums[index])
        } else {
            0
        };
        // right branch case, leave the value
        let count2 = Self::dfs(nums, index + 1, sum);

        // calculate current node result
        count1 + count2
    }
}

use std::collections::HashMap;

struct SolutionMemoization;

/// first implement the non-memo case, then plug the memo in
impl SolutionMemoization {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        // memo[index][sum] -> count
        let mut memo = HashMap::new();
        Self::dfs(&mut memo, &nums, sum, 0)
    }
    /// think in two cases
    /// 1. the leaf node case: it's either a valid result or not
    /// 2. the mid node case: we should take the current value or not
    fn dfs(memo: &mut HashMap<(usize, u32), u32>, nums: &[u32], sum: u32, index: usize) -> u32 {
        if sum == 0 {
            return 1;
        }
        if index >= nums.len() {
            return 0;
        }
        // plug the memo in
        if let Some(res) = memo.get(&(index, sum)) {
            return *res;
        }
        let count1 = if nums[index] <= sum {
            Self::dfs(memo, nums, sum - nums[index], index + 1)
        } else {
            0
        };
        let count2 = Self::dfs(memo, nums, sum, index + 1);
        let res = count1 + count2;
        // plug the memo in
        memo.insert((index, sum), res);
        res
    }
}

/// we will try to find if we can make up all possible sums with every subset to
/// populate the array dp[total_numbers][sum + 1].
///
/// so at every step we have two options:  -> TAKE IT OR LEAVE IT
///   1. exclude the number. count all the subsets without the given number up to
///      the given sum => dp[index-1][sum]
///   2. include the number if its value is not more than the sum. in that case, we
///      will count all the subset to get the remaining sum => dp[index - 1][sum - nums[index]]
/// to find the total sets, we will add both of the above values
///
/// when the value of dp array is bool type, it's clear for whether we can do something and which
/// state can do.
/// when the value of dp array is number type, it's clear for how many ways can do or the how many
/// profits can we accumulated.
struct SolutionDp;
impl SolutionDp {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        // dp[index][sum] -> count
        let mut dp = vec![vec![0; sum as usize + 1]; nums.len()];
        // same as the recursive method, there are two cases, the leaf node/edge case should be take
        // care of separately
        for outer in dp.iter_mut() {
            outer[0] = 1; // think carefully about the edge case
        }

        for s in 1..=sum {
            dp[0][s as usize] = if s == nums[0] { 1 } else { 0 };
        }

        // take it or leave it
        // dp[i][s] = dp[i-1][s] + dp[i-1][s-nums[i]]
        for i in 1..nums.len() {
            for s in 1..=sum {
                let s = s as usize;
                if s >= nums[i] as usize {
                    dp[i][s] = dp[i - 1][s] + dp[i - 1][s - nums[i] as usize];
                } else {
                    dp[i][s] = dp[i - 1][s]
                }
            }
        }

        dp[nums.len() - 1][sum as usize]
    }
}

/// the time complexity of the above algorithm is exponential O(2^N), where N represents the total
/// number.
///
/// the space complexity is O(N), this memory is used to store the recursion stack.
#[test]
fn test1() {
    assert_eq!(
        SolutionBruteForce::count_of_subset_sum(vec![1, 1, 2, 3], 4),
        3
    );
    assert_eq!(
        SolutionBruteForce::count_of_subset_sum(vec![1, 2, 7, 1, 5], 9),
        3
    );
}

#[test]
fn test2() {
    assert_eq!(
        SolutionMemoization::count_of_subset_sum(vec![1, 1, 2, 3], 4),
        3
    );
    assert_eq!(
        SolutionMemoization::count_of_subset_sum(vec![1, 2, 7, 1, 5], 9),
        3
    );
}

/// the above solution has the time and space complexity of O(NS), where N represents total numbers
/// and S is the desired sum
#[test]
fn test3() {
    assert_eq!(SolutionDp::count_of_subset_sum(vec![1, 1, 2, 3], 4), 3);
    assert_eq!(SolutionDp::count_of_subset_sum(vec![1, 2, 7, 1, 5], 9), 3);
}
