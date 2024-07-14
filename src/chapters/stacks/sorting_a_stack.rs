/// this algorithm leverages the LIFO nature of a stack. it removes elements from
/// the original stack one by one and uses a second stack to keep the elements sorted.
/// if the top element of the sorted stack is larger than the current element, it moves
/// the larger elements back to the original stack until it finds the correct spot for
/// the current element, at which point it pushes the current element onto the sorted
/// stack. because of this, the smaller elements end up at the bottom of the sorted
/// stack and the largest element on the top, resulting in a stack sorted in descending
/// order from top to bottom
struct Solution;

impl Solution {
    fn sorting_a_stack(mut num_stack: Vec<i32>) -> Vec<i32> {
        let mut res_stack = Vec::new();
        while let Some(n) = num_stack.pop() {
            while let Some(&top) = res_stack.last() {
                if top <= n {
                    break;
                } else {
                    num_stack.push(res_stack.pop().unwrap());
                }
            }
            res_stack.push(n);
        }
        res_stack
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sorting_a_stack(vec![34, 3, 31, 98, 92, 23]),
        vec![3, 23, 31, 34, 92, 98]
    );

    assert_eq!(
        Solution::sorting_a_stack(vec![4, 3, 2, 10, 12, 1, 5, 6]),
        vec![1, 2, 3, 4, 5, 6, 10, 12]
    );

    assert_eq!(
        Solution::sorting_a_stack(vec![20, 10, -5, -1]),
        vec![-5, -1, 10, 20]
    );
}
