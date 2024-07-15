/// to solve this problem, we use a sliding window approach. this technique involves
/// expanding and contracting a window over the array to find the shortest subarray
/// that meets the condition. by keeping a running sum of the elements in the window,
/// we can check if the current window meets or exceeds the target sum S.
///
/// if it does, we try to shrink the window from the left to see if we can get a smaller
/// subarray that still meets the requirement.
///
/// this approach is efficient because it processes each element of the array only once,
/// resulting in a linear time complexity.
struct Solution;

impl Solution {
    fn smallest_subarray_with_a_greater_sum(target: u32, nums: Vec<u32>) -> usize {
        let mut min_range = usize::MAX;
        let mut window_sum = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            window_sum += nums[right];
            while window_sum >= target {
                min_range = min_range.min(right - left + 1);
                window_sum -= nums[left];
                left += 1;
            }
        }
        if min_range == usize::MAX {
            0
        } else {
            min_range
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::smallest_subarray_with_a_greater_sum(7, vec![2, 1, 5, 2, 3, 2]),
        2
    );

    assert_eq!(
        Solution::smallest_subarray_with_a_greater_sum(7, vec![2, 1, 5, 2, 8]),
        1
    );
    assert_eq!(
        Solution::smallest_subarray_with_a_greater_sum(8, vec![3, 4, 1, 1, 6]),
        3
    );
}
