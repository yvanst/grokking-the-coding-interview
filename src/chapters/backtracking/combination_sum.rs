use std::cell::RefCell;
use std::rc::Rc;
/// what backtrack means is that we set a list as our answer, and iterate the candidate array,
/// adding each candidate to the answer and recursively call the function to add more candidate
/// and update the remaining target, if the target becomes 0, we add current solution to the
/// result list, if the target becomes negative, we backtrack and remove the last added candidate
/// from the list.
///
/// backtrack doesn't return anything? we use side effect to update result and have the iteration
/// of remaining cases no matter what
struct Solution;

impl Solution {
    fn combination_sum(candidates: Vec<u32>, target: u32) -> Vec<Vec<u32>> {
        let candidates = Rc::new(candidates);
        let res = Rc::new(RefCell::new(Vec::new()));
        let combination = Rc::new(RefCell::new(Vec::new()));

        Self::backtrack(
            candidates.clone(),
            0,
            target,
            combination.clone(),
            res.clone(),
        );
        let borrow = res.borrow();
        borrow.clone()
    }
    fn backtrack(
        candidates: Rc<Vec<u32>>,
        index: usize,
        target: u32,
        combination: Rc<RefCell<Vec<u32>>>,
        res: Rc<RefCell<Vec<Vec<u32>>>>,
    ) {
        if target == 0 {
            res.borrow_mut().push(combination.borrow().to_vec());
            return;
        }

        // combination: we don't need to the subsequent items from forward, only afterward is enough
        // since we allow for count multiple times for one item, we include the current index in the next recursion
        for i in index..candidates.len() {
            let candidate = candidates[i];
            if candidate <= target {
                combination.borrow_mut().push(candidate);
                Self::backtrack(
                    candidates.clone(),
                    i,
                    target - candidate,
                    combination.clone(),
                    res.clone(),
                );
                combination.borrow_mut().pop();
            }
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );

    assert_eq!(
        Solution::combination_sum(vec![2, 4, 6, 8], 10),
        vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 4],
            vec![2, 2, 6],
            vec![2, 4, 4],
            vec![2, 8],
            vec![4, 6]
        ]
    );
}
