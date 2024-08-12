//! Given an array of numbers which is sorted in ascending order and is rotated k times around a
//! pivot, find k.
//!
//! You can assume that the array does not have any duplicates.
//!
//! Note: You need to solve the problem in O(logN) time complexity.

/// this problem follows the Binary Search pattern
///
/// in this problem, actually we are asked to find the index of the minimum element. the number of
/// times the minimum elements is moved to the right will be equal to the number of rotations.
/// an interesting fact about the minimum element is that it is the only element in the given array
/// which is smaller than its previous element.
///
/// to adjust the ranges we can follow the same approach as discussed in Search in Rotated Array.
/// comparing the numbers at indices start and middle will give us two options:
/// - if arr[start] < arr[middle], the numbers from start to middle are sorted
/// - otherwise, the numbers from middle+1 to end are sorted
///
/// after calculating the middle, we can compare the number at index middle with its previous and
/// next number. this will give us two options:
/// - if arr[middle] > arr[middle+1], then the element at middle+1 is the smallest
/// - if arr[middle-1] > arr[middle], then the element at middle is the smallest
struct Solution;

impl Solution {
    fn rotation_count(nums: Vec<i32>) -> usize {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            if mid < high && nums[mid] > nums[mid + 1] {
                return mid + 1;
            }
            if mid > low && nums[mid - 1] > nums[mid] {
                return mid;
            }
            if nums[mid] > nums[low] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        0
    }
}

/// the time complexity is O(logN)
///
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(Solution::rotation_count(vec![10, 15, 1, 3, 8]), 2);
    assert_eq!(Solution::rotation_count(vec![4, 5, 7, 9, 10, -1, 2]), 5);
    assert_eq!(Solution::rotation_count(vec![1, 3, 8, 10]), 0);
}
