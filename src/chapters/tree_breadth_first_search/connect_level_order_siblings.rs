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
    fn connect_level_order_siblings(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let mut deque = VecDeque::new();
        deque.push_back(root.clone());

        while !deque.is_empty() {
            let mut previous_node: Option<Rc<RefCell<TreeNode>>> = None;
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();

                if let Some(next) = node.clone().borrow().left.clone() {
                    deque.push_back(next.clone());
                }

                if let Some(next) = node.clone().borrow().right.clone() {
                    deque.push_back(next.clone());
                }

                if previous_node.is_some() {
                    previous_node.unwrap().borrow_mut().next = Some(node.clone());
                }
                previous_node = Some(node.clone());
            }
        }
        root
    }

    fn print_level_order(root: Rc<RefCell<TreeNode>>) {
        let mut next_level_head = Some(root);
        while next_level_head.is_some() {
            let mut current_node = next_level_head;
            let mut values = Vec::new();
            next_level_head = None;
            while current_node.is_some() {
                let current = current_node.clone().unwrap();
                if next_level_head.is_none() && current.borrow().left.is_some() {
                    next_level_head.clone_from(&current.borrow().left);
                }
                if next_level_head.is_none() && current.borrow().right.is_some() {
                    next_level_head.clone_from(&current.borrow().right);
                }
                values.push(current.borrow().val);
                current_node.clone_from(&current.borrow().next);
            }
            println!("{:?}", values);
        }
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
    Solution::print_level_order(Solution::connect_level_order_siblings(root));
}
