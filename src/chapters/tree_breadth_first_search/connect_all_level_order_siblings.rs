use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeRef = Rc<RefCell<TreeNode>>;
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    next: Option<NodeRef>,
}
struct Solution;

impl Solution {
    fn connect_all_level_order_siblings(node: NodeRef) -> NodeRef {
        let mut deque = VecDeque::new();
        deque.push_back(node.clone());
        let mut prev_node: Option<NodeRef> = None;
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();

            if let Some(next) = node.clone().borrow().left.clone() {
                deque.push_back(next)
            }
            if let Some(next) = node.clone().borrow().right.clone() {
                deque.push_back(next)
            }
            prev_node = match prev_node {
                Some(prev) => {
                    prev.borrow_mut().next = Some(node.clone());
                    Some(node.clone())
                }
                None => Some(node.clone()),
            }
        }
        node
    }

    fn print_level_order(root: NodeRef) {
        let mut current_node = Some(root);
        let mut values = Vec::new();
        while current_node.is_some() {
            let current = current_node.clone().unwrap();
            values.push(current.borrow().val);
            current_node.clone_from(&current.borrow().next);
        }
        println!("{:?}", values);
    }
}

#[test]
fn test() {
    let root = Rc::new(RefCell::new(TreeNode {
        val: 12,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
                next: None,
            }))),
            right: None,
            next: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: None,
                next: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
                next: None,
            }))),
            next: None,
        }))),
        next: None,
    }));
    Solution::print_level_order(Solution::connect_all_level_order_siblings(root));
}
