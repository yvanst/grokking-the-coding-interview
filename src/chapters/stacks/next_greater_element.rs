/// we can use a more optimized approach using stack data structure. the algorithm
/// will leverage the nature of the stack data structure, where the most recently
/// added elements are the first ones to be removed.
///
/// starting from the end of the array, the algorithm always maintains elements in
/// the stack that are larger than the current element. this way, it ensures that it
/// has a candidate for the next larger element. if there is no larger element, it
/// assigns -1 to that position. it handles each element of the array only once, making
/// it an efficient solution.
///
/// detailed step-by-step walkthrough:
/// 3. start a loop that goes from the last index of the array to the first
/// 4. in each iteration, while there are elements in the stack and the top elements of
///    the stack is less than or equal to the current element in the array, remove
///    elements from the stack. this step ensures that we retain only the elements in
///    the stack that are larger than the current element
/// 5. after the popping process, if there is still an element left in the stack, it
///    is the next larger element for the current array element. so assign the top element
///    of the stack to the corresponding position in the res array
/// 6. now, push the current array element into the stack. this action considers the
///    current element as a possible next larger element for the upcoming elements in
///    the remaining iterations   -> think that n-1 is larger/smaller than n, stack can work in both cases
struct Solution;

impl Solution {
    fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut res = vec![-1; nums.len()];

        for (idx, &n) in nums.iter().enumerate().rev() {
            while let Some(&right_num) = stack.last() {
                if right_num > n {
                    break;
                } else {
                    stack.pop();
                }
            }
            if !stack.is_empty() {
                res[idx] = *stack.last().unwrap();
            }
            stack.push(n);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 5, 2, 25]),
        vec![5, 25, 25, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![13, 7, 6, 12]),
        vec![-1, 12, 12, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![1, 2, 3, 4, 5]),
        vec![2, 3, 4, 5, -1]
    );
}
