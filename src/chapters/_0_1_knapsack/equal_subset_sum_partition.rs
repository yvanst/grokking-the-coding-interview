use std::rc::Rc;

struct SolutionBruteForce;

/// use two dimensional array because it's necessary for the bottom-up approach
/// we set the initial state as target and gradually reduce it to 0 because we are narrowing down/simplying the question
/// likewise, we increment the index because we are trying to solve a simpler question with smaller options
impl SolutionBruteForce {
    fn equal_subset_sum_partition(nums: Vec<i32>) -> bool {
        let nums = Rc::new(nums);
        let target = nums.iter().sum::<i32>();
        if target % 2 == 1 {
            return false;
        }
        Self::recursive(nums.clone(), target / 2, 0)
    }

    fn recursive(nums: Rc<Vec<i32>>, target: i32, index: usize) -> bool {
        if target == 0 {
            return true;
        }
        if target < 0 || index >= nums.len() {
            return false;
        }

        let res1 = if nums[index] <= target {
            Self::recursive(nums.clone(), target - nums[index], index + 1)
        } else {
            false
        };

        let res2 = Self::recursive(nums.clone(), target, index + 1);
        res1 || res2
    }
}

struct SolutionDp;

impl SolutionDp {
    /// bottom up solution is opposite to top down solution lie in their deal with index
    /// for bottom up solution, dp[index][capacity] means for a subproblem  0..=index, can we satisfy the requirement?
    /// for top down solution, fn(index, capacity) means for a subproblem index..len(), can we satisfy the requirement?
    /// for both cases, capacity is an extra requriement that needs to be introduced by if statement
    fn equal_subset_sum_partition(nums: Vec<usize>) -> bool {
        let target = nums.iter().sum::<usize>();
        if target % 2 != 0 {
            return false;
        }
        let target = target / 2;
        // dp[index][sum] == true if we can make them sum from the first index numbers
        // note the array notation requries every num > 0 and target > 0
        let mut dp = vec![vec![false; target + 1]; nums.len()];
        for i in 0..nums.len() {
            dp[i][0] = true;
        }
        for s in 0..=target {
            dp[0][s] = nums[0] == s;
        }
        // dp[index][sum] = dp[index - 1][sum] || dp[index - 1][sum - nums[index]]
        for i in 1..nums.len() {
            for s in 1..=target {
                if s >= nums[i] {
                    dp[i][s] = dp[i - 1][s - nums[i]] || dp[i - 1][s];
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
    assert!(SolutionBruteForce::equal_subset_sum_partition(vec![
        1, 2, 3, 4
    ]));
    assert!(SolutionBruteForce::equal_subset_sum_partition(vec![
        1, 1, 3, 4, 7
    ]));
    assert!(!SolutionBruteForce::equal_subset_sum_partition(vec![
        2, 3, 4, 6
    ]));

    assert!(SolutionDp::equal_subset_sum_partition(vec![1, 2, 3, 4]));
    assert!(SolutionDp::equal_subset_sum_partition(vec![1, 1, 3, 4, 7]));
    assert!(!SolutionDp::equal_subset_sum_partition(vec![2, 3, 4, 6]));
}
// }
