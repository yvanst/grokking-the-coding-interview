/// since the numbers at both end can give us the largest square, an alternative
/// approach could be to use two pointers starting at both ends of the input
/// array. at any step, whichever pointer gives us the bigger square, we add it
/// to the result array and move to the next/revious number
struct Solution;

impl Solution {
    fn squaring_a_sorted_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut idx = res.len();
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            idx -= 1;
            let left_square = nums[left] * nums[left];
            let right_square = nums[right] * nums[right];
            if left_square < right_square {
                res[idx] = right_square;
                right -= 1;
            } else {
                res[idx] = left_square;
                left += 1;
            }
            // idx = idx.saturating_sub(1);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::squaring_a_sorted_array(vec![-2, -1, 0, 2, 3]),
        vec![0, 1, 4, 4, 9]
    );
    assert_eq!(
        Solution::squaring_a_sorted_array(vec![-3, -1, 0, 1, 2]),
        vec![0, 1, 1, 4, 9]
    );
}
