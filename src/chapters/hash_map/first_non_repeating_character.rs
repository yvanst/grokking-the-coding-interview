use std::collections::HashMap;

struct Solution;

impl Solution {
    fn first_non_repeating_character(s: String) -> i32 {
        let mut hashtable = HashMap::new();
        for ch in s.chars() {
            hashtable.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        for (idx, ch) in s.char_indices() {
            if hashtable[&ch] == 1 {
                return idx as i32;
            }
        }
        -1
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::first_non_repeating_character("apple".to_string()),
        0
    );

    assert_eq!(
        Solution::first_non_repeating_character("abcab".to_string()),
        2
    );

    assert_eq!(
        Solution::first_non_repeating_character("abab".to_string()),
        -1
    );
}
