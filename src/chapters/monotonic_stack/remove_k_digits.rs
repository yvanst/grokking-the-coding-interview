//! Given a non-negative integer represented as a string num and an integer k, delete k digits from num
//! to obtain the smallest possible integer.
//! Return this minimum possible integer as a string.

/// the idea is to keep removing the peak digit from the number num. the peak digit in a number is a
/// digit after which the next digit is smaller.
/// by doing so, we are always trying to minimize the leftmost digit which will give us the smallest
/// possible number.
///
/// we can use a monotonically increasing stack to keep track of the decreasing peak digits.
struct Solution;

impl Solution {
    fn remove_k_digits(num: String, mut k: usize) -> u32 {
        let mut mono_inc_stack = Vec::new();
        for ch in num.chars() {
            // local optimal, we would like a smaller value to replace the top value in the stack
            while k > 0 && mono_inc_stack.last().filter(|top| **top > ch).is_some() {
                mono_inc_stack.pop();
                k -= 1;
            }
            mono_inc_stack.push(ch)
        }
        while k > 0 {
            mono_inc_stack.pop();
            k -= 1;
        }
        mono_inc_stack
            .iter()
            .map(|ch| ch.to_digit(10).unwrap())
            .reduce(|acc, e| acc * 10 + e)
            .unwrap()
    }
}

/// the time complexity is O(N)
///
/// the space complexity is O(N)
#[test]
fn test() {
    assert_eq!(Solution::remove_k_digits("1432219".to_string(), 3), 1219);
    assert_eq!(Solution::remove_k_digits("10200".to_string(), 1), 200);
    assert_eq!(Solution::remove_k_digits("1901042".to_string(), 4), 2);
}
