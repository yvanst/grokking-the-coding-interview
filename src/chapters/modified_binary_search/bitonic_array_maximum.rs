//! Find the maximum value in a given Bitonic array. An array is considered bitonic if it is first
//! monotonically increasing and then monotonically decreasing.
//!
//! In other words, a bitonic array starts with a sequence of increasing elements, reaches a peak
//! element, and then follows with a sequence of decreasing elements. The peak element is the
//! maximum value in the array.

/// since no two consecutive numbers are same (as the array is monotonically increasing or
/// decreasing), whenever we calculate the middle, we can compare the numbers pointed out by the
/// index middle and middle+1 to find if we are in the ascending or the descending part
/// - if arr[middle] > arr[middle+1], we are in the descending part of the bitonic array. therefore,
/// our required number either be pointed out by middle or will be before middle. this means we will
/// be doing end=middle
/// - if arr[middle] < arr[middle+1], we are in the ascending part of the bitonic array. therefore,
/// the required
/// - we can break when start==end. due to the two points mentioned above, both start and end will
/// be pointing at the maximum number of the bitonic array.
struct Solution;

impl Solution {
    fn bitonic_array_maximum(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&nums[mid + 1]) {
                std::cmp::Ordering::Equal => panic!(),
                std::cmp::Ordering::Less => low = mid + 1,
                // notice we don't have +1 here, we want to preserve the higher value in either low
                // or high
                std::cmp::Ordering::Greater => high = mid,
            }
        }
        // at the end of the loop, low == high
        nums[low]
    }
}

/// the time complexity is O(logN)
/// 
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(Solution::bitonic_array_maximum(vec![1, 3, 8, 12, 4, 2]), 12);
    assert_eq!(Solution::bitonic_array_maximum(vec![3, 8, 3, 1]), 8);
    assert_eq!(Solution::bitonic_array_maximum(vec![1, 3, 8, 12]), 12);
    assert_eq!(Solution::bitonic_array_maximum(vec![10, 9, 8]), 10);
}
