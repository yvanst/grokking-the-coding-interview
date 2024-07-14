struct Solution;

impl Solution {
    fn balanced_parentheses(parentheses: String) -> bool {
        let parentheses = parentheses.chars().collect::<Vec<_>>();
        let mut stack = Vec::new();

        for p in parentheses {
            match p {
                '(' | '[' | '{' => stack.push(p),
                right => match stack.pop() {
                    Some(left) => {
                        if left == '(' && right == ')' {
                            continue;
                        }
                        if left == '[' && right == ']' {
                            continue;
                        }
                        if left == '{' && right == '}' {
                            continue;
                        }
                        return false;
                    }
                    None => return false,
                },
            }
        }

        stack.is_empty()
    }
}

#[test]
fn test() {
    assert!(Solution::balanced_parentheses("{[()]}".to_string()));
    assert!(!Solution::balanced_parentheses("{[}]".to_string()));
    assert!(!Solution::balanced_parentheses("(]".to_string()));
}
