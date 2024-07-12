use super::TreeNode;
/// this problem follows the Binary Tree Path Sum pattern. we can follow the same
/// DFS approach. but there are four differences:
/// 1. we will keep track of the current path in a list which will be passed to
///    every recursive call
/// 2. whenever we traverse a node we will do two things
///    - add the current node to the current path
///    - as we added a new node to the current path, we should find the sums of
///      all subpaths ending at the current node. if the sum of any sub-path is
///      equal to S, we will increment our path count
/// 3. we will traverse all paths and will not stop processing after finding the
///    first path
/// 4. remove the current node from the current path before returning from the
///    function. this is needed to backtrack while we are going up the recursive
///    call stack to process other paths
struct Solution;

impl Solution {
    fn count_paths_for_a_sum(root: Box<TreeNode>, sum: i32) -> u32 {
        let mut path = Vec::new();
        Self::dfs(&Some(root), sum, &mut path)
    }

    // the type of the node should be the same as the left/right node definition
    // in the TreeNode struct, since they are the same semantics
    fn dfs(node: &Option<Box<TreeNode>>, sum: i32, path: &mut Vec<i32>) -> u32 {
        match node.as_ref() {
            Some(node) => {
                let mut acc = node.val;
                let mut cur_res = if acc == sum { 1 } else { 0 };
                for num in path.iter().rev() {
                    acc += num;
                    if acc == sum {
                        cur_res += 1
                    }
                }
                path.push(node.val);
                cur_res += Self::dfs(&node.left, sum, path);
                cur_res += Self::dfs(&node.right, sum, path);
                path.pop();
                // use a variable to store the backtrack result
                // pop the current value to resotre the enviroment and then return
                cur_res
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
    assert_eq!(Solution::count_paths_for_a_sum(root, 11), 2);
}
