//! Given an array of numbers sorted in an ascending order, find the ceiling of a given number key.
//! The ceiling of the key will be the smallest element in the given array greater than or equal to
//! the key.
//!
//! Write a function to return the index of the ceiling of the key. If there isn’t any ceiling
//! return -1.

/// this problem follows the Binary Search pattern. since binary search helps us find a number in a
/// sorted array efficiently, we can use a modified version of the binary search to find the ceiling
/// of a number
///
/// we will try to search for the key in the given array. if we find the key, we return its index as
/// the ceiling. if we can't find the key, the next big number will be pointed out by the index start
///
/// because (low+high)/2 is round toward low, so eventually mid will point to low when high-low=1,
/// in that case, we will move low beyond target in the next loop, and the mid will be the same
/// value as low and high which will be larger than the target, in the next round, high will move
/// left by 1 to break the low<=high condition
struct Solution;

impl Solution {
    fn ceiling_of_a_number(nums: Vec<i32>, target: i32) -> Option<usize> {
        // if low is 0, high will be -1, which causes subtract overflow
        if nums.first().filter(|first| **first > target).is_some() {
            return Some(0);
        }
        // we can check if target is bigger than the biggest number in the input array, if so, we
        // can return None
        nums.last().filter(|last| **last >= target)?;
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        // since the loop is runnning until low <= high, so at the end of the while loop, start ==
        // end + 1 indicating we are not able to find the element in the given array, so the next
        // bigger number will be nums[low]
        Some(low)
    }
}

/// since we are reducing the search range by half at every step, this means that the time
/// complexity of our algorithm will be O(logN) where N is the total elements in the given array.
/// 
/// the algorithm runs in constant space O(1)
#[test]
fn test() {
    assert_eq!(Solution::ceiling_of_a_number(vec![4, 6, 10], 6), Some(1));
    assert_eq!(
        Solution::ceiling_of_a_number(vec![1, 3, 8, 10, 15], 12),
        Some(4)
    );
    assert_eq!(Solution::ceiling_of_a_number(vec![4, 6, 10], 17), None);
    assert_eq!(Solution::ceiling_of_a_number(vec![4, 6, 10], -1), Some(0));
}
