//! Given two integer arrays to represent weights and profits of N items, find a subset of these
//! items that will give us maximum profit such that their cumulative weight is not more than a
//! given number C, and return the maxium profit. Each item can only be selected once, which means
//! either we put an item in the knapsack or we skip it.

struct SolutionBruteForce;

impl SolutionBruteForce {
    fn __0_1_knapsack(profits: Vec<i32>, weights: Vec<i32>, capacity: i32) -> i32 {
        Self::dfs(&profits, &weights, capacity, 0)
    }
    fn dfs(profits: &[i32], weight: &[i32], capacity: i32, cur_idx: usize) -> i32 {
        if cur_idx == weight.len() {
            return 0;
        }
        let profit1 = if weight[cur_idx] <= capacity {
            profits[cur_idx] + Self::dfs(profits, weight, capacity - weight[cur_idx], cur_idx + 1)
        } else {
            0
        };
        let profit2 = Self::dfs(profits, weight, capacity, cur_idx + 1);
        profit1.max(profit2)
    }
}
struct SolutionMemoization;

impl SolutionMemoization {
    fn __0_1_knapsack(profits: Vec<i32>, weights: Vec<i32>, capacity: i32) -> i32 {
        // memo[cur_index][remain_weight]
        let mut memo = vec![vec![-1; capacity as usize + 1]; weights.len()];

        Self::dfs(&profits, &weights, capacity, 0, &mut memo)
    }
    fn dfs(
        profits: &[i32],
        weight: &[i32],
        remain_weight: i32,
        cur_idx: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if cur_idx == weight.len() {
            return 0;
        }
        if memo[cur_idx][remain_weight as usize] > 0 {
            return memo[cur_idx][remain_weight as usize];
        }
        let profit1 = if weight[cur_idx] <= remain_weight {
            profits[cur_idx]
                + Self::dfs(
                    profits,
                    weight,
                    remain_weight - weight[cur_idx],
                    cur_idx + 1,
                    memo,
                )
        } else {
            0
        };
        let profit2 = Self::dfs(profits, weight, remain_weight, cur_idx + 1, memo);
        let res = profit1.max(profit2);
        memo[cur_idx][remain_weight as usize] = res;
        res
    }
}

struct SolutionDp;

impl SolutionDp {
    fn __0_1_knapsack(profits: Vec<i32>, weights: Vec<i32>, capacity: i32) -> i32 {
        // memo[index][capacity]
        let mut dp = vec![vec![-1; capacity as usize + 1]; weights.len()];

        // no capacity
        for outer in dp.iter_mut() {
            outer[0] = 0;
        }
        // either select the first element or not
        for inner in 0..=capacity {
            dp[0][inner as usize] = if inner >= weights[0] { profits[0] } else { 0 }
        }
        for i in 1..weights.len() {
            for c in 1..=(capacity as usize) {
                if c >= weights[i] as usize {
                    dp[i][c] = dp[i - 1][c].max(dp[i - 1][c - weights[i] as usize] + profits[i])
                } else {
                    dp[i][c] = dp[i - 1][c]
                }
            }
        }
        dp[weights.len() - 1][capacity as usize]
    }
}

/// the above algorithm's time complexity is exponential O(2^N), where N represents the total number
/// of items.
///
/// the space complexity is O(N). the space will be used to store the recursion stack. since the
/// recursive algorithm works in a depth-first fashion, which means that we can't have more than N
/// recursive calls stack at any time
#[test]
fn test1() {
    assert_eq!(
        SolutionBruteForce::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7),
        22
    );
    assert_eq!(
        SolutionBruteForce::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6),
        17
    );
}

/// since our memoization array `dp[profits.len()][capacity+1]` stores the results for all
/// subproblems, we can conclude that we will not have more than NC subproblems where N is the
/// number of items and C is the knapsack capacity, this means that our time complexity will be
/// O(NC)
///
/// the above algorithm will use O(NC) space for the memoization array. other than that, we will use
/// O(N) space for the recursion call-stack. so the total space complexity will be O(NC + N), which
/// is asymptotically equivalent to O(NC)
#[test]
fn test2() {
    assert_eq!(
        SolutionMemoization::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7),
        22
    );
    assert_eq!(
        SolutionMemoization::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6),
        17
    );
}
/// the above solution has the time and space complexity of O(NC) where N represents total items,
/// and C is the maximum capacity
#[test]
fn test3() {
    assert_eq!(
        SolutionDp::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7),
        22
    );
    assert_eq!(
        SolutionDp::__0_1_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6),
        17
    );
}
