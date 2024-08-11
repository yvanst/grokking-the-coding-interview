//! You are given a string s and an integer k. Your task is to remove groups of identical,
//! consecutive characters from the string such that each group has exactly k characters. The
//! removal of groups should continue until it's no longer possible to make any more removals. The
//! result should be the final version of the string after all possible removals have been made.

/// this problem can be solved by using a stack to keep track of the characters and their counts.
///
/// we iterate through the string and add each character and its count to the stack. if the count of
/// the top character of the stack becomes k, we remove that entry from the stack.
///
/// the result string is then constructed from the remaining characters in the stack.
struct Solution;

struct Item {
    ch: char,
    cnt: usize,
}

impl Solution {
    fn remove_all_adjacent_duplicates_in_string_ii(text: String, k: usize) -> String {
        let mut mono_stack: Vec<Item> = Vec::new();

        for ch in text.chars() {
            if mono_stack.last().filter(|top| top.ch == ch).is_some() {
                if let Some(mut top) = mono_stack.pop() {
                    top.cnt += 1;
                    if top.cnt != k {
                        mono_stack.push(top);
                    }
                }
            } else {
                mono_stack.push(Item { ch, cnt: 1 })
            }
        }
        mono_stack.iter().flat_map(|i| vec![i.ch; i.cnt]).collect()
    }
}

/// the time complexity is O(N)
///
/// the space complexity is O(N)
#[test]
fn test() {
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string_ii("abbbaaca".to_string(), 3),
        "ca".to_string()
    );
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string_ii("abbaccaa".to_string(), 3),
        "abbaccaa".to_string()
    );
    assert_eq!(
        Solution::remove_all_adjacent_duplicates_in_string_ii("abbacccaa".to_string(), 3),
        "abb".to_string()
    );
}
