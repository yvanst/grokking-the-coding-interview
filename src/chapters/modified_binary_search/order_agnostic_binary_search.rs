//! Given a sorted array of numbers, find if a given number ‘key’ is present in the array. Though we
//! know that the array is sorted, we don’t know if it’s sorted in ascending or descending order.
//! You should assume that the array can have duplicates.
//!
//! Write a function to return the index of the ‘key’ if it is present in the array, otherwise
//! return -1.

/// we can compare the numbers pointed out by start and end index to find the sort order.
struct Solution;

impl Solution {
    fn order_agnostic_binary_search(nums: Vec<i32>, target: i32) -> Option<usize> {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let is_ascending = nums[low] < nums[high];
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => {
                    if is_ascending {
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    if is_ascending {
                        low = mid + 1;
                    } else {
                        high = mid - 1;
                    }
                }
            }
        }
        None
    }
}

/// since we are reducing the search range by half at every step, the time complexity is O(logN)
///
/// the algorithm runs in constant space O(1)
#[test]
fn test() {
    assert_eq!(
        Solution::order_agnostic_binary_search(vec![4, 6, 10], 10),
        Some(2)
    );
    assert_eq!(
        Solution::order_agnostic_binary_search(vec![1, 2, 3, 4, 5, 6, 7], 5),
        Some(4)
    );
    assert_eq!(
        Solution::order_agnostic_binary_search(vec![10, 6, 4], 10),
        Some(0)
    );
    assert_eq!(
        Solution::order_agnostic_binary_search(vec![10, 6, 4], 4),
        Some(2)
    )
}
