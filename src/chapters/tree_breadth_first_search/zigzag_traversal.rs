use super::TreeNode;
use std::collections::VecDeque;
struct Solution;

impl Solution {
    fn zigzag_traversal(node: Box<TreeNode>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_back(node);

        while !deque.is_empty() {
            let mut cur_ans = Vec::new();
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();
                cur_ans.push(node.val);
                if let Some(next_node) = node.left {
                    deque.push_back(next_node);
                };
                if let Some(next_node) = node.right {
                    deque.push_back(next_node);
                };
            }
            res.push(cur_ans)
        }

        res.into_iter()
            .enumerate()
            .map(|(idx, vec)| {
                if idx % 2 == 0 {
                    vec
                } else {
                    vec.into_iter().rev().collect::<Vec<_>>()
                }
            })
            .collect::<Vec<Vec<_>>>()
    }
}

#[test]
fn test() {
    let root = Box::new(TreeNode {
        val: 12,
        left: Some(Box::new(TreeNode {
            val: 7,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 10,
                left: Some(Box::new(TreeNode {
                    val: 20,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 17,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
    });
    assert_eq!(
        Solution::zigzag_traversal(root),
        vec![vec![12], vec![1, 7], vec![9, 10, 5], vec![17, 20]]
    );
}
