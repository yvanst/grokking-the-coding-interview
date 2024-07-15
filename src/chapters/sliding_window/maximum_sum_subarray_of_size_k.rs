/// the sliding window approach works well because it maintains a running sum of a
/// fixed number of elements.
struct Solution;

impl Solution {
    fn maximum_sum_subarray_of_size_k(nums: Vec<u32>, k: usize) -> u32 {
        let mut max_sum = 0;
        let mut window_sum = 0;
        let mut window_start = 0;
        for window_end in 0..nums.len() {
            window_sum += nums[window_end];
            if window_end < k {
                continue;
            }
            window_sum -= nums[window_start];
            max_sum = max_sum.max(window_sum);
            window_start += 1;
        }
        max_sum
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::maximum_sum_subarray_of_size_k(vec![2, 1, 5, 1, 3, 2], 3),
        9
    );
    assert_eq!(
        Solution::maximum_sum_subarray_of_size_k(vec![2, 3, 4, 1, 5], 2),
        7
    );
}
