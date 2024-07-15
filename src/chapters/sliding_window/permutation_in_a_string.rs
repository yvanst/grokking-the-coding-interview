use std::collections::HashMap;
struct Solution;

impl Solution {
    fn permutation_in_a_string(text: String, pattern: String) -> bool {
        let text = text.chars().collect::<Vec<_>>();
        let pattern = pattern.chars().collect::<Vec<_>>();
        let mut frequency_counter = HashMap::new();
        let mut left = 0;

        for &ch in pattern.iter() {
            frequency_counter
                .entry(ch)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for right in 0..text.len() {
            frequency_counter.entry(text[right]).and_modify(|c| *c -= 1);

            if right - left + 1 > pattern.len() {
                frequency_counter.entry(text[left]).and_modify(|c| *c += 1);
                left += 1;
            }

            if frequency_counter.values().all(|c| *c == 0) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::permutation_in_a_string(
        "oidbcaf".to_string(),
        "abc".to_string()
    ));

    assert!(!Solution::permutation_in_a_string(
        "odicf".to_string(),
        "dc".to_string()
    ));

    assert!(Solution::permutation_in_a_string(
        "bcdxabcdy".to_string(),
        "bcdyabcdx".to_string()
    ));

    assert!(Solution::permutation_in_a_string(
        "aaacb".to_string(),
        "abc".to_string()
    ));
}
