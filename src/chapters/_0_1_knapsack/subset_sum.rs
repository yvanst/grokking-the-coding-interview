/// the state beween 0..nums.len() and 0..=capacity is not symmtric
/// we don't have the corresponding state of no item available, we start from one item, though we can choose not to take it
/// on the other hand, we have the state of don't have any capacity, in such cases, we can fill the dp with all true

struct SolutionDp;
impl SolutionDp {
    fn subset_sum(nums: Vec<u32>, sum: u32) -> bool {
        let sum = sum as usize;
        // dp[index][sum]
        let mut dp = vec![vec![false; sum + 1]; nums.len()];

        for i in 0..nums.len() {
            dp[i][0] = true;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

    #[test]
    fn test1() {
        assert!(SolutionDp::subset_sum(vec![1, 2, 3, 7], 6));
        assert!(SolutionDp::subset_sum(vec![1, 2, 7, 1, 5], 10));
        assert!(!SolutionDp::subset_sum(vec![1, 3, 4, 8], 6));
    }
// }