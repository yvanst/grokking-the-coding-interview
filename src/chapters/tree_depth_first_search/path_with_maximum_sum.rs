use super::TreeNode;
/// this problem follows the Binary Tree Path Sum pattern and shares the algorithmic
/// logic with Tree Diameter. we can follow the same DFS approach. the only difference
/// will be to ignore the paths with negative sums. since we need to find the overall
/// maximum sum, we should ignore any path which has an overall negative sum
struct Solution;

impl Solution {
    fn path_with_maximum_sum(root: Box<TreeNode>) -> i32 {
        let mut sum = i32::MIN;
        Self::dfs(&Some(root), &mut sum);
        sum
    }

    fn dfs(node: &Option<Box<TreeNode>>, sum: &mut i32) -> i32 {
        match node.as_ref() {
            Some(node) => {
                let left_sum = Self::dfs(&node.left, sum);
                let right_sum = Self::dfs(&node.right, sum);
                *sum = (*sum).max(left_sum + right_sum + node.val);
                node.val + left_sum.max(right_sum)
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
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Box::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })),
            })),
        })),
    });
    assert_eq!(Solution::path_with_maximum_sum(root), 31);
}
