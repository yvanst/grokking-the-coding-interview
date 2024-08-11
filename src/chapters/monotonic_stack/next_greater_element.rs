//! Given two integer arrays nums1 and nums2, return an array answer such that answer[i] is the next
//! greater number for every nums1[i] in nums2.
//!
//! The next greater element for an element x is the first element to the right of x that is greater
//! than x. If there is no greater number, output -1 for that number.
//!
//! The numbers in nums1 are all present in nums2.

/// this problem can be solved using a monotonic stack and hashmap. the monotonic stack helps to
/// find the next greater element for each element in nums2. the hashmap then maps each element to
/// its next greater element
///
/// - loop through nums2 and find the next greater number using monotonic stack and store the result
/// in the hashmap
/// - after the loop, any numbers remaining in on the stack do not have a next greater element, so
/// for these numbers, use -1 as the value
/// - create an output array by mapping each numbers in nums1 to its corresponding value in the
/// hashmap
///
struct Solution;

use std::collections::HashMap;
impl Solution {
    fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut mono_dec_stack = Vec::new();
        let mut hashmap = HashMap::new();
        for num in nums2 {
            while mono_dec_stack.last().filter(|top| **top < num).is_some() {
                if let Some(top) = mono_dec_stack.pop() {
                    hashmap.entry(top).or_insert(num);
                }
            }
            mono_dec_stack.push(num);
        }
        nums1
            .iter()
            .map(|n| *hashmap.get(n).unwrap_or(&-1))
            .collect()
    }
}

/// the time complexity is O(N)
///
/// the space complexity is O(N)
#[test]
fn test() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 2, 6], vec![6, 2, 4, 5, 3, 7]),
        vec![5, 4, 7]
    );
}
