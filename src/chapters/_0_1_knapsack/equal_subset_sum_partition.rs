//! Given a set of positive numbers, find if we can partition it into two subsets such that the sum
//! of elements in both subsets is equal.

/// this problem follows the 0/1 Knapsack pattern, a basic brute-force solution could be to try all
/// combinations of partitioning the given numbers into two sets to see if any pair of sets has an
/// equal sum.
///
/// assume that S represents the total sum of all the given numbers. then the two equal subsets must
/// have a sum equal to S/2. this essentially transforms our problem to: find a subset of the given
/// numbers that has a total sum of S/2
struct SolutionBruteForce;

/// use two dimensional array because it's necessary for the bottom-up approach.
///
/// we set the initial state as target and gradually reduce it to 0 because we are narrowing
/// down/simplying the question. likewise, we increment the index because we are trying to solve a
/// simpler question with smaller choices
impl SolutionBruteForce {
    fn equal_subset_sum_partition(nums: Vec<i32>) -> bool {
        let target = nums.iter().sum::<i32>();
        if target % 2 == 1 {
            return false;
        }
        Self::dfs(&nums, target / 2, 0)
    }

    fn dfs(nums: &[i32], target: i32, index: usize) -> bool {
        if target == 0 {
            return true;
        }
        if index >= nums.len() {
            return false;
        }

        let res1 = if nums[index] <= target {
            Self::dfs(nums, target - nums[index], index + 1)
        } else {
            false
        };

        let res2 = Self::dfs(nums, target, index + 1);
        res1 || res2
    }
}

struct SolutionDp;

impl SolutionDp {
    /// bottom up solution is opposite to top down solution can be tell from how they deal with
    /// index.
    ///
    /// for bottom up solution, dp[index][capacity] means for a subproblem  0..=index, can we
    /// meet the requirement?
    ///
    /// for top down solution, fn(index, capacity) means for a subproblem index..len(), can we
    /// meet the requirement?
    ///
    /// for both cases, capacity is an extra requriement that needs to be introduced in if statement
    fn equal_subset_sum_partition(nums: Vec<usize>) -> bool {
        let target = nums.iter().sum::<usize>();
        if target % 2 != 0 {
            return false;
        }
        let target = target / 2;
        // dp[index][sum] == true if we can meet the sum requirement from 0..index numbers
        // notice the + 1 in capacity
        let mut dp = vec![vec![false; target + 1]; nums.len()];
        for outer in dp.iter_mut() {
            outer[0] = true;
        }

        for s in 0..=target {
            dp[0][s] = nums[0] == s;
        }
        // notice we skip the base case and start from 1.
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

/// the time complexity of the above algorithm is exponential O(2^N), where N represents the total
/// number.
///
/// the space complexity is O(N), which will be used to store the recursion stack.
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
}
/// the above solution has the time and space complexity of O(N*S), where N represents total numbers
/// and S is the total sum of all the numbers.
#[test]
fn test2() {
    assert!(SolutionDp::equal_subset_sum_partition(vec![1, 2, 3, 4]));
    assert!(SolutionDp::equal_subset_sum_partition(vec![1, 1, 3, 4, 7]));
    assert!(!SolutionDp::equal_subset_sum_partition(vec![2, 3, 4, 6]));
}
