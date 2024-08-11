//! Given an array of integers temperatures representing daily temperatures, your task is to
//! calculate how many days you have to wait until a warmer temperature. If there is no future day
//! for which this is possible, put 0 instead.

/// this problem is quite similar to next greater element. we can use a monotonically increasing
/// stack to find the next higher temperature
///
/// we will use a stack to store the indices of the temperature array. we iterate over the array,
/// and for each temperature, we check whether the current temperature is greater than the
/// temperature at the index on the top of the stack.
/// if it is, we update the corresponding position in the result array and pop the index from the
/// stack
struct Solution;

impl Solution {
    fn daily_temperatures(temperatures: Vec<i32>) -> Vec<usize> {
        let mut res = vec![0; temperatures.len()];
        let mut mono_dec_stack = Vec::new();

        for (idx, temp) in temperatures.iter().enumerate() {
            while mono_dec_stack
                .last()
                .filter(|top| temperatures[**top] < *temp)
                .is_some()
            {
                if let Some(update_idx) = mono_dec_stack.pop() {
                    res[update_idx] = idx - update_idx;
                }
            }
            mono_dec_stack.push(idx);
        }

        res
    }
}

/// the time complexity is O(N)
/// 
/// the space complexity is O(N)
#[test]
fn test() {
    assert_eq!(
        Solution::daily_temperatures(vec![70, 73, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![73, 72, 71, 70]),
        vec![0, 0, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![70, 71, 72, 73]),
        vec![1, 1, 1, 0]
    );
}
