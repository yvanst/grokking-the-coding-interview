/// we can use a stack to efficiently create the binary representation of a given
/// decimal number. our algorithm will take advantage of the last in, first out
/// property of stacks to reverse the order of the binary digits, since the binary
/// representation is constructed from least significant bit to most significant bit,
/// but needs to be displayed in the opposite order. the procedure involves repeatedly
/// dividing the decimal number by 2, pushing the remainder onto the stack, which
/// corresponds to the binary digit. when the number is reduced to 0, the algorithm
/// pops elemetns off the stack and appends to the result string until the stack is
/// empty, thereby reversing the order of digits
struct Solution;

impl Solution {
    fn decimal_to_binary_conversion(mut num: u32) -> String {
        let mut stack = Vec::new();
        while num != 0 {
            stack.push(num % 2);
            num /= 2;
        }
        let mut res = String::with_capacity(stack.len());
        while let Some(d) = stack.pop() {
            res.push(char::from_digit(d, 10).unwrap());
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::decimal_to_binary_conversion(2), "10".to_string());
    assert_eq!(Solution::decimal_to_binary_conversion(7), "111".to_string());
    assert_eq!(
        Solution::decimal_to_binary_conversion(18),
        "10010".to_string()
    );
}
