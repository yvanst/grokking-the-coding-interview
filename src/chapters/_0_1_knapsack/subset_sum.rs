//! Given a set of positive numbers, determine if a subset exists whose sum is equal to a given
//! number ‘S’.

/// this problem follows the 0/1 Knapsack pattern and is quite similar to Equal Subset Sum partition
///
/// the state beween 0..nums.len() and 0..=capacity is not symmtric.
/// we don't have the state of no item available, we start from one item, though we can choose not
/// to take it. on the other hand, we have the state of don't have any capacity, in such cases, we
/// can fill the dp with all true
struct SolutionDp;
impl SolutionDp {
    fn subset_sum(nums: Vec<u32>, sum: u32) -> bool {
        let sum = sum as usize;
        // dp[index][sum]
        let mut dp = vec![vec![false; sum + 1]; nums.len()];

        for outer in dp.iter_mut() {
            outer[0] = true;
        }

        for s in 0..=sum {
            dp[0][s] = s == nums[0] as usize;
        }

        // dp[i][s] = dp[i - 1][s] || dp[i - 1][s - nums[i]]
        for i in 1..nums.len() {
            for s in 1..=sum {
                if s >= nums[i] as usize {
                    dp[i][s] = dp[i - 1][s - nums[i] as usize] || dp[i - 1][s];
                } else {
                    dp[i][s] = dp[i - 1][s]
                }
            }
        }

        dp[nums.len() - 1][sum]
    }
}

/// the above solution has the time and space complexity of O(N*S), where N represents total numbers
/// and S is the required sum
#[test]
fn test() {
    assert!(SolutionDp::subset_sum(vec![1, 2, 3, 7], 6));
    assert!(SolutionDp::subset_sum(vec![1, 2, 7, 1, 5], 10));
    assert!(!SolutionDp::subset_sum(vec![1, 3, 4, 8], 6));
}
