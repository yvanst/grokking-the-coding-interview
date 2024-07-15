use std::collections::HashMap;
struct Solution;

impl Solution {
    fn string_anagrams(text: String, pattern: String) -> Vec<usize> {
        let text = text.chars().collect::<Vec<_>>();
        let pattern = pattern.chars().collect::<Vec<_>>();
        let mut res = Vec::new();
        let mut frequency_map = HashMap::new();

        for &ch in &pattern {
            frequency_map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut left = 0;
        for right in 0..text.len() {
            frequency_map.entry(text[right]).and_modify(|c| *c -= 1);
            if right - left + 1 > pattern.len() {
                frequency_map.entry(text[left]).and_modify(|c| *c += 1);
                left += 1;
            }
            if frequency_map.values().all(|c| *c == 0) {
                res.push(left);
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::string_anagrams("ppqp".to_string(), "pq".to_string()),
        vec![1, 2]
    );
    assert_eq!(
        Solution::string_anagrams("abbcabc".to_string(), "abc".to_string()),
        vec![2, 3, 4]
    );
}
