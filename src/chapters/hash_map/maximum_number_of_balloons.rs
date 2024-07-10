use std::collections::HashMap;
struct Solution;

impl Solution {
    fn maximum_number_of_balloons(text: String) -> u32 {
        let mut hashmap = HashMap::new();
        for ch in text.chars() {
            hashmap.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut res = u32::MAX;
        for ch in ['b', 'a', 'l', 'o', 'n'] {
            if ch == 'l' || ch == 'o' {
                res = res.min(*hashmap.get(&ch).unwrap_or(&0) / 2);
            } else {
                res = res.min(*hashmap.get(&ch).unwrap_or(&0));
            }
        }

        res
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::maximum_number_of_balloons("balloonballoon".to_string()),
        2
    );
    assert_eq!(
        Solution::maximum_number_of_balloons("bbaall".to_string()),
        0
    );
    assert_eq!(
        Solution::maximum_number_of_balloons("balloonballoooon".to_string()),
        2
    );
}
