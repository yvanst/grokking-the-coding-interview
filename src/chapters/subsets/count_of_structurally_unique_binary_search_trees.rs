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
