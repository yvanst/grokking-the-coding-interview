//! Given an array of numbers sorted in ascending order, find the range of a given number key. The
//! range of the key will be the first and last position of the key in the array.
//!
//! Write a function to return the range of the key. If the key is not present return [-1, -1].

/// the problem follows the Binary Search pattern. we can use a modified version of the Binary
/// Search to find the first and last position of a number.
/// we will try to search for the key in the given array, if the key is found, we have two options:
/// - when trying to find the first position of the key, we can update end = mid - 1 to see if the key
/// in present before mid
/// - when trying to find the last position of the key, we can update start = mid + 1 to see if the
/// key is present after mid
/// in both cases, we will keep track of the last position where we found the key. these positions
/// will be the required range.
struct Solution;

impl Solution {
    fn number_range(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
        let left = Self::binary_search(&nums, target, true);
        left.and_then(|left| {
            let right = Self::binary_search(&nums, target, false);
            right.map(|right| (left, right))
        })
    }

    fn binary_search(nums: &[i32], target: i32, is_left_most: bool) -> Option<usize> {
        let mut res = None;
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => {
                    res.replace(mid);
                    if is_left_most {
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                }
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        res
    }
}

/// the time complexity is O(logN)
///
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(Solution::number_range(vec![4, 6, 6, 6, 9], 6), Some((1, 3)));
    assert_eq!(
        Solution::number_range(vec![1, 3, 8, 10, 15], 10),
        Some((3, 3))
    );
    assert_eq!(Solution::number_range(vec![1, 3, 8, 10, 15], 12), None);
}
