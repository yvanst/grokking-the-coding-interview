use std::{cell::RefCell, rc::Rc};

struct SolutionBruteForce;

impl SolutionBruteForce {
    fn solve_knapsack(profits: Vec<u32>, weights: Vec<u32>, capacity: u32) -> u32 {
        let profits = Rc::new(profits);
        let weights = Rc::new(weights);
        Self::recursive(profits.clone(), weights.clone(), capacity as i32, 0)
    }

    fn recursive(profits: Rc<Vec<u32>>, weights: Rc<Vec<u32>>, capacity: i32, index: usize) -> u32 {
        if capacity <= 0 || index >= profits.len() {
            return 0;
        }
        let profit1 = if weights[index] as i32 <= capacity {
            profits[index]
                + Self::recursive(
                    profits.clone(),
                    weights.clone(),
                    capacity - weights[index] as i32,
                    index + 1,
                )
        } else {
            0
        };

        let profit2 = Self::recursive(profits.clone(), weights.clone(), capacity, index + 1);

        profit1.max(profit2)
    }
}

struct SolutionMemoization;
impl SolutionMemoization {
    fn solve_knapsack(profits: Vec<u32>, weights: Vec<u32>, capacity: u32) -> u32 {
        let profits = Rc::new(profits);
        let weights = Rc::new(weights);

        let memo = vec![vec![-1; capacity as usize + 1]; weights.len()];
        let memo = Rc::new(RefCell::new(memo));

        Self::recursive(memo, profits, weights, capacity as i32, 0)
    }
    fn recursive(
        memo: Rc<RefCell<Vec<Vec<i32>>>>,
        profits: Rc<Vec<u32>>,
        weights: Rc<Vec<u32>>,
        capacity: i32,
        index: usize,
    ) -> u32 {
        if capacity <= 0 || index >= weights.len() {
            return 0;
        }
        if memo.borrow()[index][capacity as usize] > 0 {
            return memo.borrow()[index][capacity as usize] as u32;
        }
        let profit1 = if weights[index] as i32 <= capacity {
            profits[index]
                + Self::recursive(
                    memo.clone(),
                    profits.clone(),
                    weights.clone(),
                    capacity - weights[index] as i32,
                    index + 1,
                )
        } else {
            0
        };

        let profit2 = Self::recursive(
            memo.clone(),
            profits.clone(),
            weights.clone(),
            capacity,
            index + 1,
        );

        let res = profit1.max(profit2);
        memo.borrow_mut()[index][capacity as usize] = res as i32;
        res
    }
}

struct SolutionDp;
impl SolutionDp {
    fn solve_knapsack(profits: Vec<u32>, weights: Vec<u32>, capacity: u32) -> u32 {
        // dp[index][capacity]
        let mut dp = vec![vec![0; capacity as usize + 1]; weights.len()];

        // boundary capacity = 0 is not valid(but the state is in the dp array), capacity = total_capacity is valid
        for i in 0..weights.len() {
            // it's not valid, we just assign it to zero
            dp[i][0] = 0;
        }

        // boundary index = 0 is valid, index = weights.len() is not valid(and the state is not in the dp array)
        for c in 0..=capacity {
            // it's valid, we need to judge the case
            if c >= weights[0] {
                dp[0][c as usize] = profits[0]
            }
        }

        // dp[index][capacity] = max(dp[index-1][capacity], profits[index] + dp[index-1][capacity - weights[index]])
        for i in 1..weights.len() {
            for c in 1..=capacity {
                let c = c as usize;
                if c >= weights[i] as usize {
                    dp[i][c] = dp[i - 1][c].max(profits[i] + dp[i - 1][c - weights[i] as usize]);
                } else {
                    dp[i][c] = dp[i - 1][c];
                }
            }
        }

        dp[weights.len() - 1][capacity as usize]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

#[test]
fn test1() {
    assert_eq!(
        SolutionBruteForce::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7),
        SolutionMemoization::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7)
    );

    assert_eq!(
        SolutionBruteForce::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6),
        SolutionMemoization::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6)
    );

    assert_eq!(
        SolutionBruteForce::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7),
        SolutionDp::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 7)
    );

    assert_eq!(
        SolutionBruteForce::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6),
        SolutionDp::solve_knapsack(vec![1, 6, 10, 16], vec![1, 2, 3, 5], 6)
    );
}
// }
