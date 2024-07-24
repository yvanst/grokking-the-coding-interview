//! Given a number n, write a function to return all structurally unique Binary Search Trees
//! (BST) that can store values 1 to n.

/// this problem follows the subsets pattern and is quite similar to evaluate expression. following
/// a similar approach, we can iterate from 1 to n and consider each number as the root of the
/// tree. all smaller numbers will make up the left sub-tree and bigger numbers will make up the
/// right sub-tree. we will make recursive calls for the left and right sub-trees
struct Solution;

#[derive(Clone, Debug)]
struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl Solution {
    fn structurally_unique_binary_search_trees(k: usize) -> Vec<Option<Box<TreeNode>>> {
        Self::dfs(1, k + 1)
    }

    fn dfs(start: usize, end: usize) -> Vec<Option<Box<TreeNode>>> {
        // base case
        if start == end {
            return vec![None];
        }

        let mut res = Vec::new();
        for i in start..end {
            // smaller values go to the left branch; larger values go to the
            // right branch.
            let left_res = Self::dfs(start, i);
            let right_res = Self::dfs(i + 1, end);
            // all possible combinations of the left branch and right branch
            for left in left_res.iter() {
                for right in right_res.iter() {
                    let root = TreeNode {
                        val: i as u32,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    res.push(Some(Box::new(root)))
                }
            }
        }
        res
    }
}

/// the time complexity of this algorithm will be exponential and will be similar to Balanced
/// Parentheses. estimated time complexity will be O(N*2^N), but the actual time complexity is
/// O(4^N/\sqrt(N)) is bounded by the Catalan number
///
/// the space complexity of this algorithm will be exponential too, estimated at O(2^N), but the
/// actual will be O(4^N/\sqrt(N))
#[test]
fn test() {
    assert_eq!(
        Solution::structurally_unique_binary_search_trees(2).len(),
        2
    );
    assert_eq!(
        Solution::structurally_unique_binary_search_trees(3).len(),
        5
    );
}
