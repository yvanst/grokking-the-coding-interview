use super::TreeNode;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn level_averages_in_a_binary_tree(node: Box<TreeNode>) -> Vec<f64> {
        let mut res = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_back(node);

        while !deque.is_empty() {
            let len = deque.len();
            let mut sum = 0;
            for _ in 0..len {
                let node = deque.pop_front().unwrap();
                sum += node.val;
                if let Some(next_node) = node.left {
                    deque.push_back(next_node);
                }
                if let Some(next_node) = node.right {
                    deque.push_back(next_node);
                }
            }
            res.push(sum as f64 / len as f64);
        }
        res
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
            right: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
    });
    assert_eq!(
        Solution::level_averages_in_a_binary_tree(root),
        vec![12f64, 4f64, 6.5f64]
    );
}
