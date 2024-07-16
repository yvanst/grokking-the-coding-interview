/// this problem follows the subsets pattern and can be mapped to permutations. we
/// can follow a similar BFS approach
///
/// following a BFS approach, we will keep adding open parentheses or close parenthesis.
/// at each step we need to keep two things in mind:
/// 1. we can't add more than N open parenthesis
/// 2. to keep the parenthesis balanced, we can add a close parenthesis only when we
/// have already added enough open parenthesis. for this, we can keep a count of open
/// and close parenthesis with every combination.
struct Solution;

/// parenthesis string, open parenthesis count, close parenthesis count
#[derive(Clone)]
struct ValidParenthesis(Vec<char>, usize, usize);

impl Solution {
    fn balanced_parentheses(k: usize) -> Vec<String> {
        let mut queue = Vec::new();
        let mut swap_queue = Vec::new();
        queue.push(ValidParenthesis(vec![], 0, 0));

        loop {
            while let Some(mut vp_open) = queue.pop() {
                if vp_open.2 < vp_open.1 {
                    let mut vp_close = vp_open.clone();
                    vp_close.0.push(')');
                    vp_close.2 += 1;
                    swap_queue.push(vp_close);
                }
                if vp_open.1 < k {
                    vp_open.0.push('(');
                    vp_open.1 += 1;
                    swap_queue.push(vp_open);
                }
            }
            std::mem::swap(&mut queue, &mut swap_queue);
            if queue[0].0.len() == k * 2 {
                break;
            }
        }

        queue
            .into_iter()
            .map(|vp| vp.0)
            .map(String::from_iter)
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::balanced_parentheses(2).len(), 2);
    assert_eq!(Solution::balanced_parentheses(3).len(), 5);
}
