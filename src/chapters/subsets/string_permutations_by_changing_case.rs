//! Given a string, find all of its permutations preserving the character sequence but changing
//! case.

use std::collections::VecDeque;
/// this problem follows the subsets pattern and can be mapped to permutations
///
/// since we need to preserve the character sequence, we can start with the actual
/// string and process each character one by one
///
/// e.g., start with the actual string "ab7c"; processing the first character, we will
/// get two permutations: "ab7c", "Ab7c"; ...
struct Solution;

impl Solution {
    fn string_permutations_by_changing_case(str: String) -> Vec<String> {
        let str = str.chars().collect::<Vec<_>>();
        let mut deque = VecDeque::new();
        let mut swap_deque = VecDeque::new();
        deque.push_back(str.clone());

        for idx in 0..str.len() {
            while let Some(mut variant) = deque.pop_front() {
                if variant[idx].is_ascii_digit() {
                    swap_deque.push_back(variant);
                } else {
                    variant[idx] = variant[idx].to_ascii_lowercase();
                    swap_deque.push_back(variant.clone());

                    variant[idx] = variant[idx].to_ascii_uppercase();
                    swap_deque.push_back(variant);
                }
            }
            std::mem::swap(&mut deque, &mut swap_deque);
        }
        deque.into_iter().map(String::from_iter).collect()
    }
}

/// since we can have 2^N*2 permutations at the most and while processing each permutation we
/// convert it into a character array, the overall time complexity of the algorithm will be
/// O(N*2^N)
///
/// all the additional space used by our algorithm is for the output list. since we can have a
/// total of O(2^N) permutations, the space complexity of our algorithm is O(N*2^N).
#[test]
fn test() {
    assert_eq!(
        Solution::string_permutations_by_changing_case("ad52".to_string()).len(),
        4
    );
    assert_eq!(
        Solution::string_permutations_by_changing_case("ab7c".to_string()).len(),
        8
    );
}
