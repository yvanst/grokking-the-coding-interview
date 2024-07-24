//! Given an expression containing digits and operations (+, -, *), find all possible ways in which
//! the expression can be evaluated by grouping the numbers and operators using parentheses.

/// the problem follows the subsets pattern and can be mapped to balanced parentheses. we can
/// follow a similar BFS approach.
/// 1. we can iterate through the expression character-by-character.
/// 2. we can break the expression into two halves whenever we get an operator
/// 3. the two parts can be calculated by recursively calling the function
/// 4. once we have evaluation results from the left and right halves, we can combine them to
///    produce all results
struct Solution;

impl Solution {
    fn evaluate_expression(expression: &str) -> Vec<i32> {
        // base case
        if expression.chars().all(|ch| ch.is_ascii_digit()) {
            return vec![expression.parse::<i32>().unwrap()];
        }

        let mut res = Vec::new();

        for (i, ch) in expression.char_indices() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    let left_res = Self::evaluate_expression(&expression[0..i]);
                    let right_res = Self::evaluate_expression(&expression[i + 1..]);
                    // all possible results of left to combine with all possible results of right
                    for &left in &left_res {
                        for &right in &right_res {
                            match ch {
                                '+' => res.push(left + right),
                                '-' => res.push(left - right),
                                '*' => res.push(left * right),
                                '/' => res.push(left / right),
                                _ => panic!(),
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        res
    }
}

/// the time complexity of this algorithm will be exponential and will be similar to Balanced
/// Parentheses. estimated tme complexity will be O(N*2^N) but the actual time complexity
/// O(4^N/\sqrt(N)) is bounded by the Catalan number is beyond the scope of a coding interview.
#[test]
fn test() {
    assert_eq!(Solution::evaluate_expression("1+2*3").len(), 2);
    assert_eq!(Solution::evaluate_expression("2*3-4-5").len(), 5);
}
