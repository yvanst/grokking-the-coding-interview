//! Given an array of numbers sorted in ascending order, find the element in the array that has the
//! minimum difference with the given key.

/// the problem follows the Binary Search pattern, we can use a modified version of the Binary
/// Search to find the number that has the minimum difference with the given key
///
/// in the binary search, if we find the key we will return it as the minimum difference number. if
/// we can't find the key, we can find the differences between the key and the two numbers pointed
/// by start and end, as these two numbers will be closest to the key.
struct Solution;

impl Solution {
    fn minimum_difference_element(nums: Vec<i32>, target: i32) -> i32 {
        if nums.first().filter(|f| **f >= target).is_some() {
            return *nums.first().unwrap();
        }
        if nums.last().filter(|f| **f <= target).is_some() {
            return *nums.last().unwrap();
        }
        let (mut low, mut high) = (0, nums.len() - 1);
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return target,
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        nums[low].min(nums[high])
    }
}

/// the time complexity is O(logN)
/// 
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(Solution::minimum_difference_element(vec![4, 6, 10], 7), 6);
    assert_eq!(Solution::minimum_difference_element(vec![4, 6, 10], 4), 4);
    assert_eq!(
        Solution::minimum_difference_element(vec![1, 3, 8, 10, 15], 12),
        10
    );
    assert_eq!(Solution::minimum_difference_element(vec![4, 6, 10], 17), 10);
}
