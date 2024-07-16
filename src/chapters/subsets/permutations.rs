use std::collections::VecDeque;

/// this problem follows the subsets pattern and we can follow a similar Breadth
/// First Search approach, however, unlike subsets, every permutation must contain
/// all the numbers
///
/// how can we generate permutations in the 4th step from the permutations of the
/// 3rd step?
///
/// if we look closely, we will realize that when we add a new number, we take each
/// permutation of the previous step and insert the new number in every position
/// to generate teh new permutation
struct Solution;

impl Solution {
    fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut deque = VecDeque::new();
        deque.push_back(vec![]);

        let mut swap_deque = VecDeque::new();

        for &n in nums.iter() {
            while let Some(perm) = deque.pop_front() {
                // notice the + 1 here
                for idx in 0..(perm.len() + 1) {
                    let mut v = perm.clone();
                    v.insert(idx, n);
                    swap_deque.push_back(v);
                }
            }
            std::mem::swap(&mut deque, &mut swap_deque);
        }

        deque.into_iter().collect::<Vec<_>>()
    }
}

#[test]
fn test() {
    // dbg!(Solution::permutations(vec![1, 3, 5]));
    assert_eq!(Solution::permutations(vec![1, 3, 5]).len(), 6);
}
