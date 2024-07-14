use std::collections::HashMap;
struct Solution;

impl Solution {
    fn pair_with_target_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let current_sum = nums[left] + nums[right];
            if current_sum == target {
                return Some((left, right));
            }
            if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        None
    }
}

struct SolutionHashMap;
impl SolutionHashMap {
    fn pair_with_target_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
        let mut hashmap = HashMap::new();
        for (idx, &n) in nums.iter().enumerate() {
            if hashmap.contains_key(&(target - n)) {
                return Some((hashmap[&(target - n)], idx));
            } else {
                hashmap.insert(n, idx);
            }
        }
        None
    }
}
#[test]
fn test1() {
    assert_eq!(
        Solution::pair_with_target_sum(vec![1, 2, 3, 4, 6], 6),
        Some((1, 3))
    );
    assert_eq!(
        Solution::pair_with_target_sum(vec![2, 5, 9, 11], 11),
        Some((0, 2))
    );
}

#[test]
fn test2() {
    assert_eq!(
        SolutionHashMap::pair_with_target_sum(vec![1, 2, 3, 4, 6], 6),
        Some((1, 3))
    );
    assert_eq!(
        SolutionHashMap::pair_with_target_sum(vec![2, 5, 9, 11], 11),
        Some((0, 2))
    );
}
