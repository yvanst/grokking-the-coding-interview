//! Given the head node of a singly linked list, modify the list such that any node that has a node
//! with a greater value to its right gets removed. The function should return the head of the
//! modified list.

/// we use the monotonic stack startegy to keep track of the highest-valued nodes in the linked
/// list, ensureing that any node with a higher value to its right gets removed.
///
/// starting from the head of the list, we will traverse each node. at each node, we will check the
/// value of the node against the node at the top of the stack.
/// if the current node has a higher value, we will pop the top value from the stack. we will keep
/// repeating this until we encounter a node with a higher value or the stack is empty. then, the
/// current node is pushed onto the stack.
/// this process ensures that the stack only contains node in decreasing order from bottom to top.
struct Solution;

impl Solution {
    fn remove_nodes_from_linked_list(nums: Vec<i32>) -> Vec<i32> {
        let mut mono_stack = Vec::new();
        for n in nums {
            while mono_stack.last().filter(|top| **top < n).is_some() {
                mono_stack.pop();
            }
            mono_stack.push(n);
        }
        mono_stack
    }
}

/// the complexity of this algorithm is O(n), because each node is pushed and popped from the stack
/// at most once.
/// 
/// the space complexity is O(n).
#[test]
fn test() {
    assert_eq!(
        Solution::remove_nodes_from_linked_list(vec![5, 3, 7, 4, 2, 1]),
        vec![7, 4, 2, 1]
    );
    assert_eq!(
        Solution::remove_nodes_from_linked_list(vec![1, 2, 3, 4, 5]),
        vec![5]
    );
    assert_eq!(
        Solution::remove_nodes_from_linked_list(vec![5, 4, 3, 2, 1]),
        vec![5, 4, 3, 2, 1]
    );
}
