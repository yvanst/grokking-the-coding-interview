/// this problem follows the Two Pointers pattern and shares similarities with "Triplet
/// Sum to Zero"
///
/// we can follow a similar approach to iterate through the array, taking one number at
/// a time. at every step during the iteration, we will search for the quadruplets similar
/// Triplet Sum to Zero whose sum is equal to the given target.
///
/// a nested loop is initiated from the next index of the outer loop. this loop ensures
/// that the current element isn't the same as the previous one to avoid duplicates
///
/// the search function is called with the array, target value, indices of the first two
/// elements, and the quadruplets list. this function will find pairs in the array whose
/// sum with arr[first] and arr[second] equals the target. any valid quadruplets found are
/// added to the quadruplets list
///
/// in search, two pointers left and right are initialzied: left to second + 1, and right to
/// the last element of the array. it then enters a while loop that continues until left
/// is less than right
struct Solution;
impl Solution {
    fn quadruple_sum_to_target(mut nums: Vec<i32>, target: i32) -> Vec<(i32, i32, i32, i32)> {
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j > 0 && nums[j] == nums[j - 1] {
                    continue;
                }
                Self::search(&nums, target, i, j, &mut res)
            }
        }

        res
    }

    fn search(nums: &[i32], target: i32, i: usize, j: usize, res: &mut Vec<(i32, i32, i32, i32)>) {
        let mut left = j + 1;
        let mut right = nums.len() - 1;
        while left < right {
            match (nums[i] + nums[j] + nums[left] + nums[right]).cmp(&target) {
                std::cmp::Ordering::Equal => {
                    res.push((nums[i], nums[j], nums[left], nums[right]));
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
                std::cmp::Ordering::Less => {
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
    assert_eq!(
        Solution::quadruple_sum_to_target(vec![4, 1, 2, -1, 1, -3], 1),
        vec![(-3, -1, 1, 4), (-3, 1, 1, 2)]
    );

    assert_eq!(
        Solution::quadruple_sum_to_target(vec![2, 0, -1, 1, -2, 2], 2),
        vec![(-2, 0, 2, 2), (-1, 0, 1, 2)]
    );
}
