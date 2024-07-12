use super::TreeNode;
use std::collections::VecDeque;
struct Solution;

impl Solution {
    fn minimum_depth_of_a_binary_tree(node: Box<TreeNode>) -> u32 {
        let mut deque = VecDeque::new();
        let mut res = 0;
        deque.push_back(node);
        while !deque.is_empty() {
            let len = deque.len();
            res += 1;
            for _ in 0..len {
                let node = deque.pop_front().unwrap();
                if node.left.is_none() && node.right.is_none() {
                    return res;
                }
                if let Some(next_node) = node.left {
                    deque.push_back(next_node);
                }
                if let Some(next_node) = node.right {
                    deque.push_back(next_node);
                }
            }
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
            left: None,
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
    assert_eq!(Solution::minimum_depth_of_a_binary_tree(root), 2);
}
