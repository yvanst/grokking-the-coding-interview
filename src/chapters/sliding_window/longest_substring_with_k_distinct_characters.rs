use std::collections::HashMap;
/// to solve this problem, we can use a sliding window approach. the idea is to maintain
/// a window that contains at most K distinct characters. we will expand the window
/// by including one character at a time from the right, and once the window contains
/// more than K distinct characters, we will shrink it from the left until it again
/// contains K or fewer distinct characters
///
/// this approach is effective because it allows us to process the string in linear time,
/// which is efficient for large inputs. by keeping track of the frequency of characters
/// within the window using a hashmap, we can efficiently manage the characters and ensure
/// that the window always contains at most K distinct characters.
struct Solution;

impl Solution {
    fn longest_substring_with_k_distinct_characters(text: String, k: usize) -> usize {
        let text = text.chars().collect::<Vec<_>>();
        let mut res = 0;
        let mut left = 0;
        let mut frequency_counter = HashMap::new();
        for right in 0..text.len() {
            let ch = text[right];
            frequency_counter
                .entry(ch)
                .and_modify(|c| *c += 1)
                .or_insert(1u32);
            while frequency_counter.len() > k {
                let ch = text[left];
                frequency_counter.entry(ch).and_modify(|c| *c -= 1);
                if frequency_counter[&ch] == 0 {
                    frequency_counter.remove(&ch);
                }
                left += 1;
            }
            res = res.max(right - left + 1);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_substring_with_k_distinct_characters("araaci".to_string(), 2),
        4
    );
    assert_eq!(
        Solution::longest_substring_with_k_distinct_characters("araaci".to_string(), 1),
        2
    );
    assert_eq!(
        Solution::longest_substring_with_k_distinct_characters("cbbebi".to_string(), 3),
        5
    );
}
