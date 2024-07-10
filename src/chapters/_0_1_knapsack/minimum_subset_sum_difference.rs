use std::{cell::RefCell, rc::Rc};
struct SolutionBruteForce;

impl SolutionBruteForce {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        let nums = Rc::new(nums);
        Self::recur(nums.clone(), 0, 0, 0)
    }

    fn recur(nums: Rc<Vec<u32>>, index: usize, sum1: u32, sum2: u32) -> u32 {
        if index == nums.len() {
            return sum1.abs_diff(sum2);
        }

        let diff1 = Self::recur(nums.clone(), index + 1, sum1 + nums[index], sum2);
        let diff2 = Self::recur(nums.clone(), index + 1, sum1, sum2 + nums[index]);

        diff1.min(diff2)
    }
}

struct SolutionMemoization;
impl SolutionMemoization {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        let nums = Rc::new(nums);
        let sum = nums.iter().sum::<u32>() as usize;
        // memo[index][sum] -> diff
        let memo = vec![vec![-1; sum + 1]; nums.len()];
        let memo = Rc::new(RefCell::new(memo));
        Self::recursive(memo, nums, 0, 0, 0)
    }

    fn recursive(
        memo: Rc<RefCell<Vec<Vec<i32>>>>,
        nums: Rc<Vec<u32>>,
        index: usize,
        sum1: u32,
        sum2: u32,
    ) -> u32 {
        if index >= nums.len() {
            return sum1.abs_diff(sum2);
        }
        if memo.borrow()[index][sum1 as usize] > 0 {
            return memo.borrow()[index][sum1 as usize] as u32;
        }
        let diff1 = Self::recursive(
            memo.clone(),
            nums.clone(),
            index + 1,
            sum1 + nums[index],
            sum2,
        );
        let diff2 = Self::recursive(
            memo.clone(),
            nums.clone(),
            index + 1,
            sum1,
            sum2 + nums[index],
        );

        let res = diff1.min(diff2);
        memo.borrow_mut()[index][sum1 as usize] = res as i32;
        res
    }
}

struct SolutionDp;

/// use true/false for the dp is always easy and we favor that case;
/// yet we can use information besides the last value
impl SolutionDp {
    fn minimum_subset_sum_difference(nums: Vec<u32>) -> u32 {
        let target = (nums.iter().sum::<u32>() / 2) as usize;
        // dp[index][sum]: up to index, can we find a combination that equals to sum
        let mut dp = vec![vec![false; target + 1]; nums.len()];

        for i in 0..nums.len() {
            dp[i][0] = true;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

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
// }
