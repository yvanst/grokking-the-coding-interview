use std::collections::HashMap;
/// as we move the right pointer to include characters, we check if the current window
/// contains all the characters from pattern in the requried frequency. once it does,
/// we try to shrink the window from the left to find the smallest possible window.
struct Solution;

impl Solution {
    fn smallest_window_containing_substring(text: String, pattern: String) -> String {
        let text = text.chars().collect::<Vec<_>>();
        let pattern = pattern.chars().collect::<Vec<_>>();

        let mut frequency_map = HashMap::new();
        let mut res_len = usize::MAX;
        let mut res_start = 0;

        for &ch in &pattern {
            frequency_map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut left = 0;
        for right in 0..text.len() {
            frequency_map.entry(text[right]).and_modify(|c| *c -= 1);

            if frequency_map.values().all(|c| *c <= 0) {
                while frequency_map.values().all(|c| *c <= 0) {
                    if res_len > right - left + 1 {
                        res_len = right - left + 1;
                        res_start = left;
                    }
                    frequency_map.entry(text[left]).and_modify(|c| *c += 1);
                    left += 1;
                }
                left -= 1;
                frequency_map.entry(text[left]).and_modify(|c| *c -= 1);
            }
        }
        if res_len == usize::MAX {
            "".to_string()
        } else {
            String::from_iter(&text[res_start..(res_start + res_len)])
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::smallest_window_containing_substring("aabdec".to_string(), "abc".to_string()),
        "abdec".to_string()
    );

    assert_eq!(
        Solution::smallest_window_containing_substring("aabdec".to_string(), "abac".to_string()),
        "aabdec".to_string()
    );

    assert_eq!(
        Solution::smallest_window_containing_substring("abdbca".to_string(), "abc".to_string()),
        "bca".to_string()
    );

    assert_eq!(
        Solution::smallest_window_containing_substring("adcad".to_string(), "abc".to_string()),
        "".to_string()
    );
}
