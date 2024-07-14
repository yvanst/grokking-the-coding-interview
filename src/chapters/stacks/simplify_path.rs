/// to simplify the path, we'll use a stack to track the directories we're currently
/// in. we'll split the input path into components by the "/" character, then process
/// each component one by one.
/// if a component is "..", we go to the previous directory by popping the top of the
/// stack.
/// if a component is "." or an empty string, we do nothing.
/// otherwise, we push the component into the stack as a new directory.
struct Solution;

// for a more optimized version, maybe we should first use split to implement the
// algorithm, then replace split with a hand written iterator
impl Solution {
    fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for p in path.split('/') {
            match p {
                "." | "" => (),
                ".." => {
                    stack.pop();
                }
                p => {
                    stack.push(p);
                }
            }
        }

        String::from_iter(["/", &stack.join("/")])
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::simplify_path("/a//b////c/d//././/..".to_string()),
        "/a/b/c".to_string()
    );

    assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());

    assert_eq!(
        Solution::simplify_path("/home//foo/".to_string()),
        "/home/foo".to_string()
    );
}
