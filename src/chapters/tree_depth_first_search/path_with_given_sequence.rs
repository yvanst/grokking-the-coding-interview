use super::TreeNode;

struct Solution;

impl Solution {
    fn path_with_given_sequence(root: Box<TreeNode>, sequence: Vec<i32>) -> bool {
        Self::dfs(&Some(root), &sequence, 0)
    }

    // no need to explicitly backtrack, the function stack save the state
    fn dfs(node: &Option<Box<TreeNode>>, sequence: &Vec<i32>, index: usize) -> bool {
        match node.as_ref() {
            Some(node) => {
                if node.left.is_none()
                    && node.right.is_none()
                    && index == sequence.len() - 1
                    && node.val == sequence[index]
                {
                    return true;
                }
                if node.val != sequence[index] {
                    return false;
                }
                if index < sequence.len() {
                    Self::dfs(&node.left, sequence, index + 1)
                        || Self::dfs(&node.right, sequence, index + 1)
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

#[test]
fn test() {
    let root = Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 0,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 6,
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
    assert!(Solution::path_with_given_sequence(root, vec![1, 1, 6]));
}
