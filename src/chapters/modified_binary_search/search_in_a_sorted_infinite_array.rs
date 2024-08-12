//! Given an infinite sorted array (or an array with unknown size), find if a given number ‘key’ is
//! present in the array. Write a function to return the index of the ‘key’ if it is present in the
//! array, otherwise return -1.
//!
//! Since it is not possible to define an array with infinite (unknown) size, you will be provided
//! with an interface ArrayReader to read elements of the array. ArrayReader.get(index) will return
//! the number at index; if the array’s size is smaller than the index, it will return
//! Integer.MAX_VALUE.

/// the problem follows the Binary Search pattern, we can use a modified version of the Binary
/// Search to find the key in an infinite sorted array.
///
/// the only issue with applying binary search in this problem is that we don't know the bounds of
/// the array. to handle this situation, we will first find the proper bounds of the array where we
/// can perform a binary search
///
/// an efficient way to find the proper bounds is to start at the beginning of the array with the
/// bounds's size as 1 and exponentially increase the bound's size until we find the bounds that can
/// have the key.
struct Solution;

struct ArrayReader {
    arr: Vec<i32>,
}

impl ArrayReader {
    fn get(&self, idx: usize) -> i32 {
        if idx < self.arr.len() {
            self.arr[idx]
        } else {
            i32::MAX
        }
    }
}

impl Solution {
    fn search_in_a_sorted_infinite_array(reader: &ArrayReader, target: i32) -> Option<usize> {
        let (mut start, mut end) = (0, 1);
        while reader.get(end) < target {
            let next_start = end + 1;
            end = end + (end - start + 1) * 2;
            start = next_start;
        }
        Self::binary_search(reader, start, end, target)
    }

    fn binary_search(
        reader: &ArrayReader,
        mut low: usize,
        mut high: usize,
        target: i32,
    ) -> Option<usize> {
        while low <= high {
            let mid = low + (high - low) / 2;
            match reader.get(mid).cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => high = mid - 1,
                std::cmp::Ordering::Less => low = mid + 1,
            }
        }
        None
    }
}

/// the time complexity is O(logN) + O(logN), which is asymptotically equivalent to O(logN)
/// 
/// the space complexity is O(1)
#[test]
fn test() {
    let reader = ArrayReader {
        arr: vec![4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30],
    };
    assert_eq!(
        Solution::search_in_a_sorted_infinite_array(&reader, 16),
        Some(6)
    );
    assert_eq!(
        Solution::search_in_a_sorted_infinite_array(&reader, 11),
        None
    );
    let reader = ArrayReader {
        arr: vec![1, 3, 8, 10, 15],
    };
    assert_eq!(
        Solution::search_in_a_sorted_infinite_array(&reader, 15),
        Some(4)
    );
    assert_eq!(
        Solution::search_in_a_sorted_infinite_array(&reader, 200),
        None
    );
}
