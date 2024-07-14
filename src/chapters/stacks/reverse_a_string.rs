struct Solution;

impl Solution {
    fn reverse_a_string(text: String) -> String {
        let mut stack = Vec::new();
        let mut res = String::with_capacity(text.len());
        for ch in text.chars() {
            stack.push(ch);
        }
        while let Some(ch) = stack.pop() {
            res.push(ch);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_a_string("Hello, World!".to_string()),
        "!dlroW ,olleH".to_string()
    );
    assert_eq!(
        Solution::reverse_a_string("OpenAI".to_string()),
        "IAnepO".to_string()
    );
    assert_eq!(
        Solution::reverse_a_string("Stacks are fun!".to_string()),
        "!nuf era skcatS".to_string()
    )
}
