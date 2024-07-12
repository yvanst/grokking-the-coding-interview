use super::TreeNode;

/// as we are trying to search for a root-to-leaf path, we can use the Depth First Search
/// (DFS) technique to solve this problem.
///
/// to recursively traverse a binary tree in a DFS fashion, we can start from the root
/// and at every step, make two recursive calls one for the left and one for the right
/// child.
///
/// here are the steps for our binary tree path sum problem:
///   1. start DFS with the root of the tree
///   2. if the current node is not a leaf node, do two things:
///   3. subtract the value of the current node from the given number to get a new sum
///      S = S - node.value
///   4. make two recursive calls for both the children of the current node with the new
///      number calculated in the previous step
///   5. at every step, see if the current node being visited is a leaf node and if its
///      value is equal to the given number S. if both these conditions are true, we have
///      found the required root-to-leaf path, therefore return true
///   6. if the current node is a leaf but its value is not equal to the given number S,
///      return false
struct Solution;

impl Solution {
    fn binary_tree_path_sum(root: Box<TreeNode>, sum: i32) -> bool {
        Self::dfs(&Some(root), sum)
    }

    fn dfs(node: &Option<Box<TreeNode>>, sum: i32) -> bool {
        match node.as_ref() {
            Some(node) => {
                if node.val == sum && node.left.is_none() && node.right.is_none() {
                    true
                } else {
                    // branch case in the graph, || serves as a aggregation of two branches
                    Self::dfs(&node.left, sum - node.val) || Self::dfs(&node.right, sum - node.val)
                }
            }
            None => false,
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
    assert!(Solution::binary_tree_path_sum(root, 23));
}
