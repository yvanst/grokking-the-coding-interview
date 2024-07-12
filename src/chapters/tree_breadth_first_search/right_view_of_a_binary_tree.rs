use super::TreeNode;
use std::collections::VecDeque;
struct Solution;

impl Solution {
    fn right_view_of_a_binary_tree(node: Box<TreeNode>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut deque = VecDeque::new();
        deque.push_back(node);

        while !deque.is_empty() {
            let len = deque.len();
            for i in 0..len {
                let node = deque.pop_front().unwrap();

                if let Some(next) = node.left {
                    deque.push_back(next);
                }
                if let Some(next) = node.right {
                    deque.push_back(next);
                }

                if i == len - 1 {
                    res.push(node.val);
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
            left: Some(Box::new(TreeNode {
                val: 9,
                left: Some(Box::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                })),
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
        Solution::right_view_of_a_binary_tree(root),
        vec![12, 1, 5, 3]
    );
}
