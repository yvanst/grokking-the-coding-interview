use super::TreeNode;
use std::collections::VecDeque;
struct Solution;

impl Solution {
    // we will keep inserting child nodes to the queue. as soon as we find
    // the given node, we will return the next node from the queue as the
    // level order successor
    fn level_order_successor(node: Box<TreeNode>, key: i32) -> Option<i32> {
        let mut deque = VecDeque::new();
        deque.push_back(node);

        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();

                if let Some(next) = node.left {
                    deque.push_back(next);
                }
                if let Some(next) = node.right {
                    deque.push_back(next);
                }

                if node.val == key {
                    return deque.pop_front().map(|node| node.val);
                }
            }
        }

        None
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
    assert_eq!(Solution::level_order_successor(root, 9), Some(10));
}
