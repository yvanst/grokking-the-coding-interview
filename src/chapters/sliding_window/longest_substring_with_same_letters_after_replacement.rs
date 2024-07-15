use std::collections::HashMap;
struct Solution;

impl Solution {
    fn longest_substring_with_same_letters_after_replacement(text: String, k: usize) -> usize {
        let text = text.chars().collect::<Vec<_>>();
        let mut res = 0;
        let mut left = 0;
        let mut frequency_counter = HashMap::new();
        let mut max_count = 0;

        for right in 0..text.len() {
            frequency_counter
                .entry(text[right])
                .and_modify(|c| *c += 1)
                .or_insert(1);

            max_count = max_count.max(frequency_counter[&text[right]]);

            while frequency_counter.values().sum::<i32>()
                - frequency_counter.values().max().unwrap()
                > k as i32
            {
                let ch = text[left];
                frequency_counter.entry(ch).and_modify(|c| *c -= 1);
                left += 1;
                if frequency_counter[&ch] == 0 {
                    frequency_counter.remove(&ch);
                }
            }

            res = res.max(right - left + 1);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_substring_with_same_letters_after_replacement("aabccbb".to_string(), 2),
        5
    );

    assert_eq!(
        Solution::longest_substring_with_same_letters_after_replacement("abbcb".to_string(), 1),
        4
    );

    assert_eq!(
        Solution::longest_substring_with_same_letters_after_replacement("abccde".to_string(), 1),
        3
    );
}
