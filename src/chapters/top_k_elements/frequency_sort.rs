//! Given a string, sort it based on the decreasing frequency of its characters.

use std::collections::HashMap;
struct Solution;

impl Solution {
    fn frequency_sort(text: &str) -> String {
        let mut frequency_map = HashMap::new();
        for ch in text.chars() {
            frequency_map.entry(ch).and_modify(|f| *f += 1).or_insert(1);
        }

        let mut res = text.chars().collect::<Vec<_>>();
        res.sort_by(|b, a| (frequency_map[a], a).cmp(&(frequency_map[b], b)));
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::frequency_sort("Programming"),
        "rrmmggoniaP".to_string()
    );
    assert_eq!(Solution::frequency_sort("abcbab"), "bbbaac".to_string());
}
