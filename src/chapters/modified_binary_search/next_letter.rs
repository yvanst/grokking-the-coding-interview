//! Given an array of lowercase letters sorted in ascending order, find the smallest letter in the
//! given array greater than a given key.
//!
//! Assume the given array is a circular list, which means that the last letter is assumed to be
//! connected with the first letter. This also means that the smallest letter in the given array is
//! greater than the last letter of the array and is also the first letter of the array.
//!
//! Write a function to return the next letter of the given key.

/// the problem follows the Binary Search pattern. we can use a modified version of it to find the
/// next letter.
///
/// we can use a similar approach as discussed in Ceiling of a Number. there are a couple of
/// differences though:
/// - the array is considered circular, which means if the key is bigger than the last letter of the
/// array or if it is smaller than the first letter of the array, the key's next letter will be the
/// first letter of the array.
/// - the other difference is that we have to find the next biggest letter which can't be equal to
/// the key. this means that we will ignore the case where key == arr[middle]. to handle this case,
/// we can update our start range to start = middle + 1
/// - in the end, instead of returning the element pointed out by start, we have to return the
/// letter pointed out by start % array_length. this is needed because if the last letter of the
/// array is equal to the key. in that case, we have to return the first letter of the input array.
struct Solution;

impl Solution {
    fn next_letter(letters: Vec<char>, target: char) -> char {
        // if we want to access/return the value in the array, we need to deal with two special
        // case:
        // case 1. low is 0, and high cannot be -1 because of usize
        if letters.first().filter(|first| **first >= target).is_some() {
            return letters[0];
        }
        let mut low = 0;
        let mut high = letters.len() - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            match letters[mid].cmp(&target) {
                std::cmp::Ordering::Equal => low = mid + 1,
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        // case 2, no greater element in the array. high cannot be increased after
        // initialization. so the last step is low == high == mid == letters.len() - 1, after which
        // low = mid + 1, bigger than high, the while loop breaks, but we can't access letters[low]
        // in such case
        if low == letters.len() {
            letters[0]
        } else {
            letters[low]
        }
    }
}

/// the time complexity is O(logN)
/// 
/// the space complexity is O(1)
#[test]
fn test() {
    assert_eq!(Solution::next_letter(vec!['a', 'c', 'f', 'h'], 'f'), 'h');
    assert_eq!(Solution::next_letter(vec!['a', 'c', 'f', 'h'], 'b'), 'c');
    assert_eq!(Solution::next_letter(vec!['a', 'c', 'f', 'h'], 'm'), 'a');
    assert_eq!(Solution::next_letter(vec!['a', 'c', 'f', 'h'], 'a'), 'a');
}
