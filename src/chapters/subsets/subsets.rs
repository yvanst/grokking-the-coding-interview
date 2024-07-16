struct SolutionDFS;

impl SolutionDFS {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut current_set = Vec::new();

        Self::dfs(&nums, 0, &mut current_set, &mut res);

        res
    }

    fn dfs(nums: &[i32], index: usize, current_set: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            res.push(current_set.clone());
            return;
        }

        current_set.push(nums[index]);
        Self::dfs(nums, index + 1, current_set, res);

        current_set.pop();
        Self::dfs(nums, index + 1, current_set, res);
    }
}

struct SolutionBFS;

impl SolutionBFS {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        res.push(vec![]);
        for n in nums {
            let mut cur_res = res.clone();
            for subset in &mut cur_res {
                subset.push(n);
            }
            res.extend(cur_res);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(SolutionDFS::subsets(vec![1, 3]).len(), 4);
    assert_eq!(SolutionDFS::subsets(vec![1, 5, 3]).len(), 8);

    assert_eq!(SolutionBFS::subsets(vec![1, 3]).len(), 4);
    assert_eq!(SolutionBFS::subsets(vec![1, 5, 3]).len(), 8);
}
