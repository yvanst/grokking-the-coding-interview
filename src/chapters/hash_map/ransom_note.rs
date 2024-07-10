use std::collections::HashMap;
struct Solution;

impl Solution {
    fn random_note(note: String, magazine: String) -> bool {
        let mut hashmap = HashMap::new();
        for ch in magazine.chars() {
            hashmap.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        for ch in note.chars() {
            let v = *hashmap.entry(ch).and_modify(|v| *v -= 1).or_insert(-1);
            if v < 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::random_note(
        "hello".to_string(),
        "hellworld".to_string()
    ));

    assert!(Solution::random_note(
        "notes".to_string(),
        "stoned".to_string()
    ));

    assert!(!Solution::random_note(
        "apple".to_string(),
        "pale".to_string()
    ));
}
