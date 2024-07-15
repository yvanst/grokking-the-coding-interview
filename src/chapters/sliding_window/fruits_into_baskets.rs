use std::collections::HashMap;
/// to solve this problem, we use a sliding window approach. this technique is effective
/// for problems involving contiguous subarrays or sublists. the sliding window approach
/// allows us to maintain a window of elements that meet a certain condition.
///
/// we expand the window by adding elements from the right and contract it from the left
/// when the condition is violated. this approach ensures we check all possible subarrays
/// without starting from each element repeatedly, making it efficient
///
/// this method works well because it continuously adjusts the window size based on the
/// elements it encounters. by using a dictionary to keep track of the fruit counts within
/// the window, we can efficiently manage and adjust the window size to find the maximum
/// length subarray that satisfies the condition.
///
/// the efficiency comes from the fact that each element is processed at most twice, once
/// when added and once when removed from the window.
struct Solution;

impl Solution {
    fn fruits_into_baskets(fruits: Vec<char>) -> usize {
        let mut res = 0;
        let mut left = 0;
        let mut frequency_counter = HashMap::new();

        for right in 0..fruits.len() {
            frequency_counter
                .entry(fruits[right])
                .and_modify(|c| *c += 1)
                .or_insert(1);

            while frequency_counter.len() > 2 {
                let ch = fruits[left];
                frequency_counter.entry(ch).and_modify(|c| *c -= 1);
                left += 1;
                if frequency_counter[&ch] == 0 {
                    frequency_counter.remove(&ch);
                }
            }

            res = res.max(right - left + 1);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::fruits_into_baskets(vec!['A', 'B', 'C', 'A', 'C']),
        3
    );
    assert_eq!(
        Solution::fruits_into_baskets(vec!['A', 'B', 'C', 'B', 'B', 'C']),
        5
    );
}
