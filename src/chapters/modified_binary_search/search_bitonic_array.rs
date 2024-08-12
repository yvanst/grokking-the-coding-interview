//! Given a Bitonic array, find if a given key is present in it. An array is considered bitonic if
//! it is first monotonically increasing and then monotonically decreasing.
//!
//! In other words, a bitonic array starts with a sequence of increasing elements, reaches a peak
//! element, and then follows with a sequence of decreasing elements. The peak element is the
//! maximum value in the array.
//!
//! Write a function to return the index of the key. If the key appears more than once, return the
//! smaller index. If the key is not present, return -1.

/// this problem follows the Binary Search pattern. we can use a modified version of the Binary
/// Search.
///
/// - first, we can find the index of the maximum value of the bitonic array, let's call the index
/// of the maximum number max_index
/// - then, we can break the array into two sub-arrays, array from 0 to max_index sorted in
/// ascending order. and array from index max_index+1 to array_length - 1 sorted in descending order
/// - we can then call Binary Search separately in these two arrays to search the key. we can use
/// the same Order-agnostic Binary Search for searching.
///
struct Solution;

impl Solution {
    fn search_bitonic_array(nums: Vec<i32>, target: i32) -> Option<usize> {
        let max_idx = Self::find_max(&nums);
        // special case: max value compare
        if nums[max_idx] < target {
            return None;
        }
        let mut res = Self::order_agnostic_binary_search(&nums, target, 0, max_idx);
        if res.is_none() {
            res = Self::order_agnostic_binary_search(&nums, target, max_idx + 1, nums.len() - 1);
        }
        res
    }
    fn find_max(nums: &[i32]) -> usize {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&nums[mid + 1]) {
                std::cmp::Ordering::Equal => panic!(),
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid,
            }
        }
        low
    }
    fn order_agnostic_binary_search(
        nums: &[i32],
        target: i32,
        mut low: usize,
        mut high: usize,
    ) -> Option<usize> {
        let is_ascending = nums[low] < nums[high];
        // special case: two min value at the boundary compare
        if is_ascending && nums[low] > target {
            return None;
        }
        if !is_ascending && nums[high] > target {
            return None;
        }
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => {
                    if is_ascending {
                        low = mid + 1;
                    } else {
                        high = mid - 1;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if is_ascending {
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                }
            }
        }
        None
    }
}

// the time complexity is O(logN)
//
// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(
        Solution::search_bitonic_array(vec![1, 3, 8, 4, 3], 4),
        Some(3)
    );
    assert_eq!(Solution::search_bitonic_array(vec![3, 8, 3, 1], 8), Some(1));
    assert_eq!(
        Solution::search_bitonic_array(vec![1, 3, 8, 12], 12),
        Some(3)
    );
    assert_eq!(Solution::search_bitonic_array(vec![10, 9, 8], 10), Some(0));
    assert_eq!(Solution::search_bitonic_array(vec![10, 9, 8], 11), None);
}
