use super::TreeNode;
use std::collections::VecDeque;
/// we can use a Queue to efficiently traverse in BFS fashion. here are the steps of our algorithm:
/// 1. start by pushing the root node to the queue.
/// 2. keep iterating until the queue is empty.
/// 3. in each iteration, first count the elements in the queue(let's call it level_size), we will
///    have these many nodes in the current level.
/// 4. next, remove level_size nodes from the queue and push their value in an array to represent
///    the current level.
/// 5. after removing each node from the queue, insert both of its children into the queue.
/// 6. if the queue is not empty, repeat from step 3 for the next level.
///

struct Solution;

impl Solution {
    fn binary_tree_level_order_traversal(node: Box<TreeNode>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);
        // this while-len-for pattern is very brilliant
        while !queue.is_empty() {
            let len = queue.len();
            let mut cur = Vec::new();
            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                cur.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.unwrap());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.unwrap())
                }
            }
            res.push(cur);
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
        Solution::binary_tree_level_order_traversal(root),
        vec![vec![12], vec![7, 1], vec![9, 10, 5]]
    );
}
