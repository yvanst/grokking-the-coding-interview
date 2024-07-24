//! Given two sorted arrays in descending order, find ‘K’ pairs with the largest sum where each pair
//! consists of numbers from both the arrays.

/// this problem follows the K-way merge pattern and we can follow a similar approach as discussed
/// in Merge K Sorted Lists.
///
/// we can go through all the numbers of the two input arrays to create pairs and initially insert
/// them all in the heap until we have K pairs in Min Heap. after that, if a pair is bigger than the
/// top(smallest) pair in the heap, we can remove the smallest pair and insert this pair in the
/// heap.
///
/// we can optimize our algorithm in two ways:
///
/// 1. instead of iterating over all the numbers of both arrays, we can iterate only the first K
/// numbers from both arrays. since the arrays are sorted in descending order, the pairs with the
/// maximum sum will be constituted by the first K numbers from both the arrays.  
///
/// 2. as soon as we encounter a pair with a sum that is smaller than the smallest(top) element of
/// the heap, we don't need to process the next elements of the array. since the arrays are sorted
/// in descending order, we won't be able to find a pair with a higher sum moving forward.
use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    sum: Reverse<i32>,
    num1: i32,
    num2: i32,
}

impl Solution {
    pub fn find_k_largest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> Vec<[i32; 2]> {
        let mut heap = BinaryHeap::new();
        for &num1 in nums1.iter().take(k.min(nums1.len())) {
            for &num2 in nums2.iter().take(k.min(nums2.len())) {
                if heap.len() < k {
                    heap.push(HeapElement {
                        sum: Reverse(num1 + num2),
                        num1,
                        num2,
                    })
                } else if heap.peek().filter(|e| e.sum.0 < num1 + num2).is_some() {
                    heap.pop();
                    heap.push(HeapElement {
                        sum: Reverse(num1 + num2),
                        num1,
                        num2,
                    });
                } else {
                    break;
                }
            }
        }
        heap.into_iter().map(|e| [e.num1, e.num2]).collect()
    }
}

/// since, at most, we'll be going through all the elements of both arrays and we will add/remove
/// one element in the heap in each step, the time complexity of the above algorithm will be
/// O(NMlogK) where N and M are the total number elements in both arrays, respectively.
///
/// if we assume that both arrays have at least K elements then the time complexity can be
/// simplified to O(K^2logK), because we are not iterating more than K elements in both arrays.
///
/// the space complexity will be O(K), because at any time, out Min Heap will be storing K largest
/// pairs.
#[test]
fn test_case() {
    use std::collections::HashSet;
    assert_eq!(
        Solution::find_k_largest_pairs(vec![9, 8, 2], vec![6, 3, 1], 3)
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([[9, 3], [9, 6], [8, 6]].iter())
    );

    assert_eq!(
        Solution::find_k_largest_pairs(vec![5, 2, 1], vec![2, -1], 3)
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([[5, 2], [5, -1], [2, 2]].iter())
    );
}
