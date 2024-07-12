use super::TreeNode;
/// for binary trees, there exists only one path to reach any leaf node,
/// we can easily say that total root-to-leaf path in a binary tree can't
/// be more than the number of leaves.
/// as we know there can't be more than (N+1)/2 leaves in a binary tree,
/// therefore the maximum number of elements in all paths will be O(N)
///
/// now, each of these paths can have many nodes in them, for a balanced
/// binary tree, each leaf node will be at maximum depths. as we know that
/// the depth of a balanced tree is O(logN) we can say that, at the most,
/// each path can have logN nodes in it.
///
/// this means that space complexity will be O(N*logN) for a balanced tree.
/// for the worse case, the tree will become a linkedlist.
///
/// since for each node, in the worst case, we have to copy log(N) nodes to
/// store its path; therefore the time complexity of our algorithm will also
/// be O(N*logN)
struct Solution;

impl Solution {
    fn all_paths_for_a_sum(node: Box<TreeNode>, sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur_ans = Vec::new();
        Self::dfs(&Some(node), sum, &mut res, &mut cur_ans);
        res
    }

    // the Option to denote whether we reach a child of a leaf node coincide with
    // the data structure to denote whether the leaf node has a child
    // thus the function signature accept Option, but we can call with &node.left
    // directly
    fn dfs(
        node: &Option<Box<TreeNode>>,
        sum: i32,
        res: &mut Vec<Vec<i32>>,
        cur_ans: &mut Vec<i32>,
    ) {
        match node.as_ref() {
            Some(node) if node.val <= sum => {
                if node.val == sum && node.left.is_none() && node.right.is_none() {
                    cur_ans.push(node.val);
                    res.push(cur_ans.clone());
                    cur_ans.pop();
                    return;
                }
                cur_ans.push(node.val);
                // nice we don't have to construct the Option
                Self::dfs(&node.left, sum - node.val, res, cur_ans);
                Self::dfs(&node.right, sum - node.val, res, cur_ans);
                // backtrack
                cur_ans.pop();
            }
            _ => (),
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
    assert_eq!(
        Solution::all_paths_for_a_sum(root, 23),
        vec![vec![12, 7, 4], vec![12, 1, 10]]
    );
}
