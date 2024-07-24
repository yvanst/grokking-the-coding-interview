//! Given a number ‘n’, write a function to return the count of structurally unique Binary Search
//! Trees (BST) that can store values 1 to ‘n’.

/// this problem is similar to Structurally Unique Binary Search Trees. following a similar
/// approach, we can iterate from 1 to N and consider each number as the root of a tree and make
/// two recursive calls to count the number of left and right sub-trees 
struct Solution;

impl Solution {
    fn count_of_structurally_unique_binary_search_trees(k: u32) -> u32 {
        Self::dfs(1, k as usize + 1)
    }

    fn dfs(start: usize, end: usize) -> u32 {
        if start == end {
            return 1;
        }
        let mut res = 0;
        // all split ways
        for i in start..end {
            let left_res = Self::dfs(start, i);
            let right_res = Self::dfs(i + 1, end);
            // cartesian product of all possible combinations
            res += left_res * right_res
        }
        res
    }
}

/// estimated time complexity will be O(N*2^N), but the actual time complexity is O(4^N/N)
///
/// estimated space complexity will be O(2^N), but the actual is O(4^N/n)
#[test]
fn test() {
    assert_eq!(
        Solution::count_of_structurally_unique_binary_search_trees(2),
        2
    );
    assert_eq!(
        Solution::count_of_structurally_unique_binary_search_trees(3),
        5
    );
}
