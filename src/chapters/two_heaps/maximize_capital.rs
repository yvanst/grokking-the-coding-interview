//! Given a set of investment projects with their respective profits, we need to find the most
//! profitable projects. We are given an initial capital and are allowed to invest only in a fixed
//! number of projects. Our goal is to choose projects that give us the maximum profit. Write a
//! function that returns the maximum total capital after selecting the most profitable projects.

//! We can start an investment project only when we have the required capital. After selecting a
//! project, we can assume that its profit has become our capital, and that we have also received
//! our capital back.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// while selecting projects we have two constraints:
/// 1. we can select a project only when we have the required capital.
/// 2. there is a maximum limit on how many projects we can select.
///
/// since we don't have any constraint on time, we should choose a project, among the projects for
/// which we have enough capital, which gives us a maximum profit.
///
/// following this greedy approach will give us the best solution.
///
/// while selecting a project, we will do two things:
/// 1. find all the projects that we can choose with the available capital.
/// 2. from the list of projects in the 1st step, choose the project that gives us a maximum
///    profit.
///
/// we can follow the Two Heaps approach similar to Find the Median of a Number Stream.
/// 1. add all project capitals to a min-heap, so that we can select a project with the smallest
///    capital requirement
/// 2. go through the top projects of the min-heap and filter the projects that can be completed
///    within our available capital. insert the profits of all these projects into a max_heap, so
///    that we can choose a project with the maximum profit.
/// 3. finally, select the top project of the max-heap for investment.
/// 4. repeat the 2nd and 3rd steps for the required number of projects.
struct Solution;

impl Solution {
    fn maximize_capital(
        capital_list: Vec<i32>,
        profit_list: Vec<i32>,
        num_of_project: usize,
        initial_capital: i32,
    ) -> i32 {
        let mut capital_heap = BinaryHeap::new();
        let mut profit_heap = BinaryHeap::new();
        let mut current_capital = initial_capital;
        let mut remaining_num_of_project = num_of_project;

        for (idx, capital) in capital_list.iter().enumerate() {
            capital_heap.push((Reverse(*capital), idx));
        }

        loop {
            while let Some((Reverse(capital), idx)) = capital_heap.pop() {
                if capital <= current_capital {
                    profit_heap.push(profit_list[idx]);
                } else {
                    capital_heap.push((Reverse(capital), idx));
                    break;
                }
            }
            if let Some(profit) = profit_heap.pop() {
                current_capital += profit;
                remaining_num_of_project -= 1;
            } else {
                break;
            }
            if remaining_num_of_project == 0 {
                break;
            }
        }

        current_capital
    }
}


/// since, at the most, all the projects will be pushed to both the heaps once, the time complexity
/// of our algorithm is O(NlogN + KlogN), where N is the total number of projects and K is the
/// number of projects we are selecting. so the overall time complexity will be O(NlogN)
///
/// the space complexity will be O(N) because we will be storing all the projects in the heaps.
#[test]
fn test() {
    assert_eq!(
        Solution::maximize_capital(vec![0, 1, 2], vec![1, 2, 3], 2, 1),
        6
    );
    assert_eq!(
        Solution::maximize_capital(vec![0, 1, 2, 3], vec![1, 2, 3, 5], 3, 0),
        8
    );
}
