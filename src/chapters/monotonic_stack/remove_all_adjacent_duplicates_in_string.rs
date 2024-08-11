//! You are given a string s consisting of lowercase English letters. A duplicate removal consists
//! of choosing two adjacent and equal letters and removing them.
//!
//! We repeatedly make duplicate removals on s until we no longer can.
//!
//! Return the final string after all such duplicate removals have been made.

/// this problem can be solved efficiently using a stack, which can mimic the process of eliminating
/// adjacent duplicates.
struct Solution;

impl Solution {
    fn remove_all_adjacent_duplicates_in_string(text: String) -> String {
        let mut mono_stack = Vec::new();
        for ch in text.chars() {
            if mono_stack.last().filter(|top| **top == ch).is_some() {
                mono_stack.pop();
            } else {
                mono_stack.push(ch);
            }
        }
        mono_stack.iter().collect()
    }
}

/// the time complexity is O(N)
///
/// the space complexity is O(N)
#[test]
fn test() {
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string("abccba".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string("foobar".to_string()),
        "fbar".to_string()
    );
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string("abcd".to_string()),
        "abcd".to_string()
    );
}
