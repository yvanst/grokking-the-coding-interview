use std::cell::RefCell;
use std::rc::Rc;

/// the only case we use a dp for int value as result is that we are counting something??
/// dp is bool case is clear for whether we can do something and which state can do
/// the so called "which one can do" can be lifted to "how many ones can do"

struct SolutionBruteForce;

impl SolutionBruteForce {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        let nums = Rc::new(nums);

        Self::recursive(nums.clone(), 0, sum)
    }

    /// think of tree structure when simulating the process,
    /// all the leaf node should be either 0 or 1
    /// but we can have mid node that is greater than 1
    fn recursive(nums: Rc<Vec<u32>>, index: usize, sum: u32) -> u32 {
        if sum == 0 {
            return 1;
        }
        if index >= nums.len() {
            return 0;
        }

        // left branch result
        let count1 = if sum >= nums[index] {
            Self::recursive(nums.clone(), index + 1, sum - nums[index])
        } else {
            0
        };
        // right branch result
        let count2 = Self::recursive(nums.clone(), index + 1, sum);

        // calculate current node result
        count1 + count2
    }
}

struct SolutionMemoization;

/// first implement the non-memo case, then plug the memo in
impl SolutionMemoization {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        let nums = Rc::new(nums);
        // memo[index][sum] -> count
        let memo = vec![vec![-1; sum as usize + 1]; nums.len()];
        let memo = Rc::new(RefCell::new(memo));
        Self::recursive(memo, nums, sum, 0)
    }
    /// think in two cases
    /// 1. the leaf node case: it's either a valid result or not
    /// 2. the mid node case: we should take the current value or not
    fn recursive(
        memo: Rc<RefCell<Vec<Vec<i32>>>>,
        nums: Rc<Vec<u32>>,
        sum: u32,
        index: usize,
    ) -> u32 {
        if sum == 0 {
            return 1;
        }
        if index >= nums.len() {
            return 0;
        }
        // plug the memo in
        if memo.borrow()[index][sum as usize] > 0 {
            return memo.borrow()[index][sum as usize] as u32;
        }
        let count1 = if nums[index] <= sum {
            Self::recursive(memo.clone(), nums.clone(), sum - nums[index], index + 1)
        } else {
            0
        };
        let count2 = Self::recursive(memo.clone(), nums.clone(), sum, index + 1);
        let res = count1 + count2;
        // plug the memo in
        memo.borrow_mut()[index][sum as usize] = res as i32;
        res
    }
}

struct SolutionDp;
/// we will try to find if we can make up all possible sums with every subset to
/// populate the array dp[total_numbers][sum + 1].
///
/// so at every step we have two options:  -> TAKE IT OR LEAVE IT
///   1. exclude the number. count all the subsets without the given number up to
///      the given sum => dp[index-1][sum]
///   2. include the number if its value is not more than the sum. in that case, we
///      will count all the subset to get the remaining sum => dp[index - 1][sum - nums[index]]
/// to find the total sets, we will add both of the above values
impl SolutionDp {
    fn count_of_subset_sum(nums: Vec<u32>, sum: u32) -> u32 {
        // dp[index][sum] -> count
        let mut dp = vec![vec![0; sum as usize + 1]; nums.len()];
        // same as the recursive method, there are two cases, the leaf node/edge case should be take
        // care of separately
        for i in 0..nums.len() {
            dp[i][0] = 1; // think carefully about the edge case
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

// #[cfg(test)]
// mod tests {
//     use super::*;

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
    #[test]
    fn test3() {
        assert_eq!(SolutionDp::count_of_subset_sum(vec![1, 1, 2, 3], 4), 3);
        assert_eq!(SolutionDp::count_of_subset_sum(vec![1, 2, 7, 1, 5], 9), 3);
    }
// }
