/// let's say sum(s1) denotes the total sum of set s1, and
/// sum(s2) denotes the total sum of set s2. so the requried
/// equation is that sum(s1) - sum(s2) = S
///
/// this equation can be reduced to the subset sum problem. let's
/// assume that sum(num) denote the total sum of all the numbers,
/// there fore sum(s1) + sum(s2) = sum(num)
///
/// let's add the two equations, we have 2 * sum(s1) = S + sum(num)
///
/// this essentially converts our problem to find the count of subsets
/// of the given numbers whose sum is equal to (S + sum(num)) / 2,
/// note that the sum should be even
struct SolutionDp;

impl SolutionDp {
    fn target_sum(nums: Vec<u32>, sum: u32) -> u32 {
        let target = nums.iter().sum::<u32>() + sum;
        if target % 2 != 0 {
            return 0;
        }
        let target = (target / 2) as usize;

        // dp[index][sum] -> count
        let mut dp = vec![vec![0; target + 1]; nums.len()];

        // for i in 0..nums.len() {
        //     dp[i][0] = 1;
        // }
        dp.iter_mut().for_each(|row| row[0] = 1);

        for s in 1..=target {
            dp[0][s] = if s == nums[0] as usize { 1 } else { 0 }
        }

        // dp[i][s] = dp[i-1][s] + dp[i-1][s - nums[i]]
        for i in 1..nums.len() {
            for s in 1..=target {
                if s >= nums[i] as usize {
                    dp[i][s] = dp[i - 1][s] + dp[i - 1][s - nums[i] as usize];
                } else {
                    dp[i][s] = dp[i - 1][s];
                }
            }
        }

        dp[nums.len() - 1][target]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

#[test]
fn test1() {
    assert_eq!(SolutionDp::target_sum(vec![1, 1, 2, 3], 1), 3);
    assert_eq!(SolutionDp::target_sum(vec![1, 2, 7, 1], 9), 2);
}
// }
