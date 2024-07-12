use super::TreeNode;

/// this problem follows the Binary Tree Path Sum pattern. we can follow the same
/// DFS approach. there will be a few differences
/// 1. at every step, we need to find the height of both children of the current
///    node. for this, we will make two recursive calls similar to DFS
/// 2. the height of the current node will be equal to the maximum of the heights
///    of its left or right children, plus 1 for the current node
/// 3. the tree diameter at the current node will be equal to the height of the left
///    child plus the height fo the right child plus 1 for the current node:
///    diameter = left_three_height + right_three_height + 1. to find the overall
///    tree diameter, we will use a class level variable. this variable will store
///    the maximum diameter of all the nodes visited so far, hence, eventually, it
///    will have the final tree diameter
struct Solution;

// the idea to deal with any path that could not necessarily pass the root
// is to use dfs call on the left child and right child and combine both
// side result in the middle and update the final result
impl Solution {
    fn tree_diameter(root: Box<TreeNode>) -> u32 {
        let mut res = 0;
        Self::dfs(&Some(root), &mut res);
        res
    }

    // notice that the return value of the function could be used indrectly
    // with the final result
    fn dfs(node: &Option<Box<TreeNode>>, tree_diameter: &mut u32) -> u32 {
        match node.as_ref() {
            Some(node) => {
                let left_height = Self::dfs(&node.left, tree_diameter);
                let right_height = Self::dfs(&node.right, tree_diameter);
                if left_height > 0 && right_height > 0 {
                    *tree_diameter = (*tree_diameter).max(left_height + right_height + 1);
                }
                1 + left_height.max(right_height)
            }
            None => 0,
        }
    }
}

#[test]
fn test() {
    let root = Box::new(TreeNode {
        val: 12,
        left: Some(Box::new(TreeNode {
            val: 7,
            left: Some(Box::new(TreeNode {
                val: 4,
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
    assert_eq!(Solution::tree_diameter(root), 5);
}
