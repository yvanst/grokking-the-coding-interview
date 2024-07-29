//! Given a string s, return the maximum number of unique substrings that the given string can be
//! split into.
//!
//! You can split string s into any list of non-empty substrings, where the concatenation of the
//! substrings forms the original string. However, you must split the substrings such that all of
//! them are unique.
//!
//! A substring is a contiguous sequence of characters within a string.

/// we can use backtracking to solve this problem.
///
/// we can use a HashSet to keep track of the unique substrings that have been split so far.
///
/// the function uses a for loop to iterate through all possible substrings starting from the
/// current start position. for each substring, it checks if the substring is already in the set. if
/// it is not, the substring is added to the set and the function is recursively called with the new
/// start position being the end of the current substring. this continues until all possible
/// substrings have been processed.
///
/// after the recursive call, the substring is removed from the set to backtrack. the function keeps
/// track of the maximum number of unique substrings found so far and returns this maximum count
/// when all substrings have been processed
use std::collections::HashSet;
struct Solution;

impl Solution {
    fn split_a_string_into_the_max_number_of_unique_substrings(text: String) -> usize {
        let mut hashset = HashSet::new();
        Self::backtrack(&text, 0, &mut hashset)
    }

    fn backtrack<'a>(text: &'a str, index: usize, hashset: &mut HashSet<&'a str>) -> usize {
        if index == text.len() {
            return hashset.len();
        }
        let mut res = 0;
        // notice the two + 1 here
        for i in index + 1..text.len() + 1 {
            let str = &text[index..i];
            if !hashset.contains(str) {
                hashset.insert(str);
                // bacause the base case can have the total count of the substrings, we don't need
                // to do the aggregation in the middle recursive node, i.e., no cur + backtrack(),
                // even though we start the recursive call in the middle, the final return value
                // contains previous substring.
                //
                // we only need to get the max answer from all branches
                res = res.max(Self::backtrack(text, i, hashset));
                hashset.remove(str);
            }
        }
        res
    }
}

/// we can split any given string of length N in 2^N ways. hence the time complexity will be O(2^N).
///
/// the space complexity will be O(N) as we need to save only one way of splitting the given string
/// while in the recursion, and our recursion tree won't get bigger than O(N) steps.
#[test]
fn test() {
    assert_eq!(
        Solution::split_a_string_into_the_max_number_of_unique_substrings("abcabc".to_string()),
        4
    );
    assert_eq!(
        Solution::split_a_string_into_the_max_number_of_unique_substrings("aab".to_string()),
        2
    );
    assert_eq!(
        Solution::split_a_string_into_the_max_number_of_unique_substrings("ababab".to_string()),
        4
    );
    assert_eq!(
        Solution::split_a_string_into_the_max_number_of_unique_substrings("".to_string()),
        0
    );
    assert_eq!(
        Solution::split_a_string_into_the_max_number_of_unique_substrings("a".to_string()),
        1
    );
}
