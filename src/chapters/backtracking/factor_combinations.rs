//! Numbers can be regarded as the product of their factors.
//!
//! For example, 8 = 2 x 2 x 2 = 2 x 4.
//!
//! Given an integer n, return all possible combinations of its factors. You may return the answer
//! in any order.

/// we will start by iterating through all integers from 2 to the square root of n + 1. if the
/// current integer i divides n, we will add i to the list of current factor and appends this list
/// along with the corresponding factor of n/i to the result.
///
/// then we can recursively call the function with n/i as the new input, i as the start value, and
/// curr and result as the current factors and results.
///
/// after each recursive call, we have to pop the last element from curr to backtrack and find other
/// factors
///
/// backtrack is used for the "all possible combinations" cases
struct Solution;

impl Solution {
    fn factor_combinations(num: u32) -> Vec<Vec<u32>> {
        let mut cur_res = Vec::new();
        let mut res = Vec::new();

        Self::backtrack(num, 2, &mut cur_res, &mut res);

        res
    }

    fn backtrack(num: u32, start: u32, cur_res: &mut Vec<u32>, res: &mut Vec<Vec<u32>>) {
        for n in start..=((num as f32).sqrt() as u32) {
            if num % n == 0 {
                cur_res.push(n);

                // edge case could not be in the beginning of the function
                // sometimes it's easier to deal with it in the middle
                cur_res.push(num / n);
                res.push(cur_res.clone());
                // pop num/n, and continue the dfs
                cur_res.pop();

                Self::backtrack(num / n, n, cur_res, res);
                // pop n for the next loop n+1
                cur_res.pop();
            }
        }
    }
}

/// we can't have more than O(N) factors of a number N. this means that the function can't be called
/// a maximum of O(N) times recursively. the for loop iterates a maximum of O(sqrt(N)) times. this
/// means that the overall time complexity is O(N*sqrt(N)) or O(N^1.5)
///
/// ignoring the space needed for the output array, the space complexity will be O(log(N)) as we
/// need to save only one factor while in the recursion, and our recursion tree won't get bigger
/// than O(log(N)) steps
#[test]
fn test() {
    assert_eq!(
        Solution::factor_combinations(8),
        vec![vec![2, 4], vec![2, 2, 2]]
    );
    assert_eq!(
        Solution::factor_combinations(20),
        vec![vec![2, 10], vec![2, 2, 5], vec![4, 5],]
    );
}
