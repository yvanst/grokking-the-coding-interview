use super::TreeNode;
use std::collections::VecDeque;

struct Solution;
impl Solution {
    fn reverse_level_order_traversal(node: Box<TreeNode>) -> Vec<Vec<i32>> {
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
                }
                if let Some(next_node) = node.right {
                    deque.push_back(next_node);
                }
            }
            res.push(cur_ans);
        }

        res.into_iter().rev().collect::<Vec<Vec<_>>>()
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
        Solution::reverse_level_order_traversal(root),
        vec![vec![9, 10, 5], vec![7, 1], vec![12]]
    );
}
