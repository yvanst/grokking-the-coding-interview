/// let's say during our iteration we are at number X, so we need to find Y and
/// Z such that X + Y + Z < target. at this stage, our problem translates into
/// finding a pair whose sum is less than target - X
struct Solution;

impl Solution {
    fn triplets_with_smaller_sum(mut nums: Vec<i32>, target: i32) -> usize {
        let mut res = 0;
        nums.sort();

        for (idx, &num) in nums.iter().enumerate() {
            Self::search(&nums, target - num, idx + 1, nums.len() - 1, &mut res)
        }

        res
    }
    fn search(nums: &[i32], target: i32, mut left: usize, mut right: usize, count: &mut usize) {
        // we need three numbers; a number cannot be used twice, thus left and right can't be equal
        while left < right {
            match (nums[left] + nums[right]).cmp(&target) {
                std::cmp::Ordering::Equal => {
                    right -= 1;
                }
                std::cmp::Ordering::Less => {
                    // since nums[right] >= nums[left], we can replace arr[right] by any number
                    // between left and right to get a sum less than the target sum
                    *count += right - left;
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    right -= 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::triplets_with_smaller_sum(vec![-1, 0, 2, 3], 3), 2);
    assert_eq!(
        Solution::triplets_with_smaller_sum(vec![-1, 4, 2, 1, 3], 5),
        4
    );
}
