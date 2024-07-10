use std::collections::HashMap;
struct Solution;

impl Solution {
    fn longest_palindrome(text: String) -> u32 {
        let mut hashmap = HashMap::new();
        for ch in text.chars() {
            hashmap.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut has_odd = false;
        let res = hashmap
            .values()
            .map(|&v| {
                if v % 2 == 1 {
                    has_odd = true;
                    v - 1
                } else {
                    v
                }
            })
            .sum::<u32>();
        if has_odd {
            res + 1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome("bananas".to_string()), 5);
    assert_eq!(Solution::longest_palindrome("applepie".to_string()), 5);
    assert_eq!(Solution::longest_palindrome("aabbcc".to_string()), 6);
}
