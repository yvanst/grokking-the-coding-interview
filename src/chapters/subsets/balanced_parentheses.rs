//!  For a given number N, write a function to generate all combination of N pairs of balanced
//!  parentheses.

use std::collections::VecDeque;
/// this problem follows the subsets pattern and can be mapped to permutations. we can follow a
/// similar BFS approach
///
/// following a BFS approach, we will keep adding open parentheses or close parenthesis. at each
/// step we need to keep two things in mind:
/// 1. we can't add more than N open parenthesis
/// 2. to keep the parenthesis balanced, we can add a close parenthesis only when we have already
///    added enough open parenthesis. for this, we can keep a count of open and close parenthesis
///    with every combination.
struct Solution;

#[derive(Clone)]
struct ValidParenthesis {
    parentheses: Vec<char>,
    open_count: usize,
    close_count: usize,
}

impl Solution {
    fn balanced_parentheses(k: usize) -> Vec<String> {
        let mut queue = VecDeque::new();
        let mut swap_queue = VecDeque::new();
        queue.push_back(ValidParenthesis {
            parentheses: vec![],
            open_count: 0,
            close_count: 0,
        });

        for _ in 0..k * 2 {
            while let Some(mut vp) = queue.pop_front() {
                if vp.close_count < vp.open_count {
                    let mut vp_close = vp.clone();
                    vp_close.parentheses.push(')');
                    vp_close.close_count += 1;
                    swap_queue.push_back(vp_close);
                }
                if vp.open_count < k {
                    vp.parentheses.push('(');
                    vp.open_count += 1;
                    swap_queue.push_back(vp);
                }
            }
            std::mem::swap(&mut queue, &mut swap_queue);
        }

        queue
            .into_iter()
            .map(|vp| vp.parentheses.into_iter().collect::<String>())
            .collect::<Vec<_>>()
    }
}

/// if we don't care for the ordering that ) can only come after (, then we have two options for
/// every position, i.e., either put open parentheses or close parentheses. this means we can have
/// a maximum of 2^N combinations. because of the ordering, the actual number will be less than
/// 2^N.
///
/// if you see the visual representation of example-2 closely you will realize that, in the worse
/// case, it is equivalent to a binary tree, where each node will have two children. this means
/// that we will have 2^N leaf nodes and 2^N - 1 intermediate nodes. so the total number of
/// elements pushed in to the queue will be 2^N + 2^N - 1, which is asynmptotically equivalent to
/// O(2^N). while processing each element, we do need to concatenate the current string with ( or
/// ). this operation will take O(N), so the overall time complexity of our algorithm will be
/// O(N*2^N). this is not completely accurate but reasonable enough to be presented in the
/// interview.
///
/// the actual time complexity O(4^n/\sqrt(n)) is bounded by the Catalan number and is beyond the
/// scope of a coding interview.
#[test]
fn test() {
    assert_eq!(Solution::balanced_parentheses(2).len(), 2);
    assert_eq!(Solution::balanced_parentheses(3).len(), 5);
}
