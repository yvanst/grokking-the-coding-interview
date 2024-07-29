//! Given a set of positive numbers, partition the set into two subsets with minimum difference
//! between their subset sums.

use std::collections::HashMap;
/// this problem follows the 0/1 Knapsack pattern and can be converted into a Subset Sum problem
struct SolutionBruteForce;

impl SolutionBruteForce {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        Self::dfs(&nums, 0, 0, 0)
    }

    fn dfs(nums: &[u32], index: usize, sum1: u32, sum2: u32) -> u32 {
        if index == nums.len() {
            return sum1.abs_diff(sum2);
        }

        let diff1 = Self::dfs(nums, index + 1, sum1 + nums[index], sum2);
        let diff2 = Self::dfs(nums, index + 1, sum1, sum2 + nums[index]);

        diff1.min(diff2)
    }
}

struct SolutionMemoization;
impl SolutionMemoization {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        // memo[index][sum] -> diff
        let mut memo: HashMap<(usize, u32), u32> = HashMap::new();
        Self::dfs(&mut memo, &nums, 0, 0, 0)
    }

    fn dfs(
        memo: &mut HashMap<(usize, u32), u32>,
        nums: &[u32],
        index: usize,
        sum1: u32,
        sum2: u32,
    ) -> u32 {
        if index >= nums.len() {
            return sum1.abs_diff(sum2);
        }
        if let Some(diff) = memo.get(&(index, sum1)) {
            return *diff;
        }
        let diff1 = Self::dfs(memo, nums, index + 1, sum1 + nums[index], sum2);
        let diff2 = Self::dfs(memo, nums, index + 1, sum1, sum2 + nums[index]);

        let res = diff1.min(diff2);
        memo.insert((index, sum1), res);
        res
    }
}

/// let's assume S represents the total sum of all the numbers. so, in this problem, we are trying
/// to find a subset whose sum is as close to S/2 as possible. if we can partition the given set
/// into two subsets of an equal sum, we get the minimum difference 0. this transforms our problem
/// to Subset Sum, where we try to find a subset whose sum is equal to a given number.
/// if we can't find such a subset, then we will take the subset which has the sum closet to S/2.
/// this is easily possible, as we will be calculating all possible sums with every subset.
///
/// what is the closet subset we can find? we can find the subset if we start moving backwards in
/// the last row from the bottom right corner to find the first true.
struct SolutionDp;

/// use true/false for the dp is always easy and we favor that case;
/// yet we have other informations besides the last value
impl SolutionDp {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        let target = (nums.iter().sum::<u32>() / 2) as usize;
        // dp[index][sum]: up to index, can we find a combination that equals to sum
        let mut dp = vec![vec![false; target + 1]; nums.len()];

        for outer in dp.iter_mut() {
            outer[0] = true;
        }

        for s in 0..=target {
            dp[0][s] = nums[0] as usize == s;
        }

        // dp[i][s] = dp[i-1][s] || dp[i-1][s-nums[i]]
        for i in 1..nums.len() {
            for s in 1..=target {
                if s >= nums[i] as usize {
                    dp[i][s] = dp[i - 1][s] || dp[i - 1][s - nums[i] as usize];
                } else {
                    dp[i][s] = dp[i - 1][s];
                }
            }
        }
        let sum1 = (0..=target).rev().find(|s| dp[nums.len() - 1][*s]).unwrap() as u32;
        let sum2 = nums.iter().sum::<u32>() - sum1;
        sum1.abs_diff(sum2)
    }
}

/// because of the two recursive calls, the time complexity of the above algorithm is exponential
/// O(2^N), where N represents the total number.
///
/// the space complexity is O(N), which is used to store the recursion stack.
#[test]
fn test1() {
    assert_eq!(
        SolutionBruteForce::minimum_subset_sum_difference(vec![1, 2, 3, 9]),
        3
    );
    assert_eq!(
        SolutionBruteForce::minimum_subset_sum_difference(vec![1, 2, 7, 1, 5]),
        0
    );
    assert_eq!(
        SolutionBruteForce::minimum_subset_sum_difference(vec![1, 3, 100, 4]),
        92
    );
}

#[test]
fn test2() {
    assert_eq!(
        SolutionMemoization::minimum_subset_sum_difference(vec![1, 2, 3, 9]),
        3
    );
    assert_eq!(
        SolutionMemoization::minimum_subset_sum_difference(vec![1, 2, 7, 1, 5]),
        0
    );
    assert_eq!(
        SolutionMemoization::minimum_subset_sum_difference(vec![1, 3, 100, 4]),
        92
    );
}
/// the above solution has the time and space complexity of O(NS), where N represents total numbers
/// and S is the total sum of all the numbers
#[test]
fn test3() {
    assert_eq!(
        SolutionDp::minimum_subset_sum_difference(vec![1, 2, 3, 9]),
        3
    );
    assert_eq!(
        SolutionDp::minimum_subset_sum_difference(vec![1, 2, 7, 1, 5]),
        0
    );
    assert_eq!(
        SolutionDp::minimum_subset_sum_difference(vec![1, 3, 100, 4]),
        92
    );
}
