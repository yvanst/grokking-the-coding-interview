use std::cell::RefCell;
use std::rc::Rc;
/// image we have a slow and a fast pointer to traverse the LinkedList. in each iteration, the slow
/// pointer moves one step and the fast pointer moves two steps. this gives us two conclusions:
///   1. if the LinkedList doesn't have a cycle in it, the fast pointer will reach the end of the
///      LinkedList before the slow pointer to reveal that there is cycle in the LinkedList.
///   2. The slow pointer will never be able to catch up to the fast pointer if there is no cycle
///      in the LinkedList.
///
/// if the LinkedList has a cycle, the fast pointer enters the cycle first, followed by the slow
/// pointer. after this, both pointer will keep moving in the cycle infinitely. if at any stage
/// both of these pointers meet, we can conclude that the LinkedList has a cycle in it.
///
/// let's analyze if it possible for the two pointers to meet. when the fast pointer is approaching
/// the slow pointer from behind we have two possibilities:
///   1. the fast pointer is one step behind the slow pointer.
///   2. the fast pointer is two steps behind the slow pointer.
///
/// all other distances between the fast and slow pointers will reduce to one of these two
/// possibilities. let's analyze these scenarios, considering the fast pointer always moves first:
///   1. if the fast pointer is one step behind the slow pointer: the fast pointer moves two steps
///      and the slow pointer moves one step, and they both meet.
///   2. if the fast pointer is two steps behind the slow pointer: the fast pointer mvoes two steps
///      and the slow pointer moves one step. after the moves, the fast pointer will be one step
///      behind the slow pointer, which reduces this scenario to the first scenario. this means
///      that the two pointers will meet in the next step.
struct Solution;

impl Solution {
    fn linkedlist_cycle(head: Option<Rc<RefCell<Node>>>) -> bool {
        let mut fast = head.clone();
        let mut slow = head.clone();
        while fast.is_some() {
            fast = fast.and_then(|node| node.borrow().next.clone());
            fast = fast.and_then(|node| node.borrow().next.clone());
            slow = slow.and_then(|node| node.borrow().next.clone());
            if fast == slow {
                return true;
            }
        }
        false
    }
}

#[derive(PartialEq)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    fn add_next(&mut self, node: Option<Rc<RefCell<Node>>>) {
        // self.next.clone_from(&node);
        self.next = node
    }
}
// impl Drop for Node {
//     fn drop(&mut self) {
//         let mut next = self.next.take();
//         while let Some(node) = next.take() {
//             next = node.borrow_mut().next.take();
//         }
//     }
// }

#[test]
fn test() {
    let nodes = (1..=6)
        .map(Node::new)
        .map(|node| Some(Rc::new(RefCell::new(node))))
        .collect::<Vec<_>>();
    for i in 0..(nodes.len() - 1) {
        nodes[i]
            .clone()
            .unwrap()
            .borrow_mut()
            .add_next(nodes[i + 1].clone());
    }
    // nodes[nodes.len() - 1]
    //     .clone()
    //     .unwrap()
    //     .borrow_mut()
    //     .add_next(nodes[0].clone());

    assert!(!Solution::linkedlist_cycle(nodes[0].clone()));
}
