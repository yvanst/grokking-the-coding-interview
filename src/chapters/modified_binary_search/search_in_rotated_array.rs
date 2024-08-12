//! Given an array of numbers which is sorted in ascending order and also rotated by some arbitrary
//! number, find if a given key is present in it.
//!
//! Write a function to return the index of the key in the rotated array. If the key is not present,
//! return -1. You can assume that the given array does not have any duplicates.
//!
//! Note: You need to solve the problem in  time complexity.

/// this problem follows the Binary Search pattern.
///
/// after calculating the middle, we can compare the numbers at indices start and middle. this will
/// give us two options:
/// - if arr[start] <= arr[middle], the number from start to middle are sorted in ascending order
/// - otherwise, the number from middle+1 to end are sorted in ascending order
///
/// once we know which part of the array is sorted, it is easy to adjust our ranges. for example, if
/// option 1 is true, we have two choices:
/// - by comparing the key with the numbers at index start and middle we can easily find out if the
/// key lies between indices start and middle; if it does, we can skip the second part and set
/// end = middle-1
/// - otherwise, we can skip the first part and set start = middle + 1
struct Solution;

impl Solution {
    fn search_in_rotated_array(nums: Vec<i32>, target: i32) -> Option<usize> {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid] == target {
                return Some(mid);
            }

            if nums[low] < nums[mid] {
                // left side is sorted in ascending order
                if target >= nums[low] && target < nums[mid] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                // right side is sorted in ascending order
                if target > nums[mid] && target <= nums[high] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        None
    }
}

/// the time complexity is O(logN)
/// 
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(
        Solution::search_in_rotated_array(vec![10, 15, 1, 3, 8], 15),
        Some(1)
    );
    assert_eq!(
        Solution::search_in_rotated_array(vec![4, 5, 7, 9, 10, -1, 2], 10),
        Some(4)
    );
}
