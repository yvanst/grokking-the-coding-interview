use std::cell::RefCell;
use std::rc::Rc;
/// backtrack is used for the "all possible combinations" cases
struct Solution;

impl Solution {
    fn factor_combinations(num: u32) -> Vec<Vec<u32>> {
        let current = Rc::new(RefCell::new(Vec::new()));
        let result = Rc::new(RefCell::new(Vec::new()));

        Self::backtrack(num, 2, current.clone(), result.clone());

        let borrow = result.borrow();
        borrow.clone()
    }

    fn backtrack(
        num: u32,
        start: u32,
        current: Rc<RefCell<Vec<u32>>>,
        result: Rc<RefCell<Vec<Vec<u32>>>>,
    ) {
        // didn't include the right end ((num as f32).sqrt() as u32 + 1)
        // to not duplicate
        for n in start..((num as f32).sqrt() as u32 + 1) {
            if num % n == 0 {
                current.borrow_mut().push(n);

                // edge case could not be in the beginning of the function
                // sometimes it's easier to deal with it in the middle
                current.borrow_mut().push(num / n);
                result.borrow_mut().push(current.borrow().clone());
                current.borrow_mut().pop();

                Self::backtrack(num / n, n, current.clone(), result.clone());
                current.borrow_mut().pop();
            }
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::factor_combinations(8),
        vec![vec![2, 4], vec![2, 2, 2]]
    );
    assert_eq!(
        Solution::factor_combinations(20),
        vec![vec![2, 10], vec![2, 2, 5], vec![4, 5],]
    );
}
