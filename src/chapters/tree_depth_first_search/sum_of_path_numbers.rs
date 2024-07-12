use super::TreeNode;

struct Solution;
impl Solution {
    fn sum_of_path_numbers(root: Box<TreeNode>) -> i32 {
        Self::dfs(&Some(root), 0)
    }

    fn dfs(node: &Option<Box<TreeNode>>, path_sum: i32) -> i32 {
        match node.as_ref() {
            Some(node) => {
                // there are only two cases, leaf node and non-leaf node
                // non-leaf node should return zero, thus we should invalidate
                // the wrong branch
                if node.left.is_none() && node.right.is_none() {
                    path_sum * 10 + node.val
                } else {
                    // tail recursive, no need to backtrack
                    Self::dfs(&node.left, path_sum * 10 + node.val)
                        + Self::dfs(&node.right, path_sum * 10 + node.val)
                }
            }
            None => 0,
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
    assert_eq!(Solution::sum_of_path_numbers(root), 332);
}
