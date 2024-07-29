//! Given a sorted number array and two integers K and X, find K closest numbers to X in the array.
//! Return the numbers in the sorted order.
//! X is not necessarily present in the array.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// this problem follows the top K numbers pattern. the biggest difference in this problem is that
/// we need to find the closet numbers compared to finding the overall largest numbers. another
/// difference is that the given array is sorted.
///
/// utilizing similar approach, we can find the numbers closest to X through the following
/// algorithm:
/// 1. since the array is sorted, we can first find the number closest to X through Binary Search.
/// let's say that number is Y.
/// 2. the K closest numbers to Y will be adjacent to Y in the array. we can search in both
/// directions of Y to find the closest numbers
/// 3. we can use a heap to efficiently search for the closest numbers. we will take K numbers in
/// both directions of Y and push them in a Min Heap sorted by their absolute difference from X.
/// this will ensure that the numbers with the smallest difference from X can be extracted from the
/// Min Heap
/// 4. finally, we will extract the top K numbers from the Min Heap to find the required numbers
struct Solution;

impl Solution {
    fn k_closest_numbers(nums: Vec<i32>, k: usize, x: i32) -> Vec<i32> {
        // let target_idx = Self::binary_search(&nums, x);
        let target_idx = match nums.binary_search(&x) {
            Ok(idx) => idx,
            // notice the idx - 1 here, index of the first element that is less than target, make
            // sure the index exists
            Err(idx) => idx - 1,
        };
        let mut heap = BinaryHeap::new();

        if let Some(&num) = nums.get(target_idx) {
            heap.push((Reverse(num.abs_diff(x)), num))
        }
        for i in 1..k {
            if let Some(&num) = nums.get(target_idx + i) {
                heap.push((Reverse(num.abs_diff(x)), num))
            }
            if let Some(&num) = nums.get(target_idx - i) {
                heap.push((Reverse(num.abs_diff(x)), num))
            }
        }
        let mut res = Vec::new();
        for _ in 0..k {
            if let Some((_, num)) = heap.pop() {
                res.push(num)
            }
        }
        res.sort();
        res
    }

    fn k_closest_numbers_using_two_pointers(nums: Vec<i32>, k: usize, x: i32) -> Vec<i32> {
        let n = nums.len();
        // find the first value that is equal or greater than x
        // because we want the first value in the duplicate case, thus partition_point return the
        // first index of the second partition
        let target_idx = nums.partition_point(|num| *num < x);
        let mut left = target_idx;
        let mut right = target_idx;
        // include left, exclude right
        while right - left < k {
            if left == 0 {
                return nums[0..k].to_vec();
            }
            if right == n {
                return nums[n - k..n].to_vec();
            }
            // notice left - 1; we are testing two values that is not in the answer
            if nums[left - 1].abs_diff(x) <= nums[right].abs_diff(x) {
                left -= 1;
            } else {
                right += 1;
            }
        }
        nums[left..right].to_vec()
    }

    fn binary_search(nums: &[i32], target: i32) -> usize {
        // make both end valid to test
        let mut low = 0;
        let mut high = nums.len() - 1;
        // continue the loop until we test the last one value
        while low <= high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                // if there are mutliple values that are equal to target, we may return the the mid
                // value?
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        // because it's either equal or the nearest less than, we return low - 1
        if low > 0 {
            return low - 1;
        }
        low
    }
}

/// the time complexity of the above algorithm is O(logN + K*logK). we need O(logN) for Binary
/// Search and O(K*logK) to insert the numbers in the Min Heap, as well as to sort the output array.
///
/// the space complexity will be O(K), as we need to put a maximum of 2K numbers in the heap.
#[test]
fn test() {
    assert_eq!(
        Solution::k_closest_numbers(vec![5, 6, 7, 8, 9], 3, 7),
        vec![6, 7, 8]
    );
    assert_eq!(
        Solution::k_closest_numbers(vec![2, 4, 5, 6, 9], 3, 6),
        vec![4, 5, 6]
    );
    assert_eq!(
        Solution::k_closest_numbers(vec![2, 4, 5, 6, 9], 3, 10),
        vec![5, 6, 9]
    );
    assert_eq!(
        Solution::k_closest_numbers_using_two_pointers(vec![5, 6, 7, 8, 9], 3, 7),
        vec![6, 7, 8]
    );
    assert_eq!(
        Solution::k_closest_numbers_using_two_pointers(vec![2, 4, 5, 6, 9], 3, 6),
        vec![4, 5, 6]
    );
    assert_eq!(
        Solution::k_closest_numbers_using_two_pointers(vec![2, 4, 5, 6, 9], 3, 10),
        vec![5, 6, 9]
    );
}
