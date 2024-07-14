/// step by step algorithm
/// 1. initialize pointers: set low to 0 and high to the last index of the array
/// 2. find left boundary: move low to the right while the current element is less
///    than or equal to the next element
/// 3. check if sorted: if low reaches the end, the array is already sorted, return 0
/// 4. find right boundary: move high to the left while the current element is greater
///    than or equal to the previous element
/// 5. find min and max: iterate from low and high to find the minimum and maximum values
///    in this subarray
/// 6. extend left boundary: move low to the left while the previous element is greater
///    than the subarray's minimum
/// 7. extend right boundary: move high to the right while the next element is less than
///    the subarray's maximum
/// 8. calculate the length: the length of the subarray to be sorted is high - low + 1
struct Solution;

impl Solution {
    fn minimum_window_sort(nums: Vec<i32>) -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left + 1 < nums.len() && nums[left] <= nums[left + 1] {
            left += 1;
        }

        if left == nums.len() - 1 {
            return 0;
        }

        while right > 0 && nums[right] >= nums[right - 1] {
            right -= 1;
        }

        let mut range_min = i32::MAX;
        let mut range_max = i32::MIN;
        for k in left..=right {
            range_min = range_min.min(nums[k]);
            range_max = range_max.max(nums[k]);
        }
        // compare the valid one, make current idx invalid
        while left > 0 && nums[left - 1] > range_min {
            left -= 1;
        }
        while right < nums.len() - 1 && nums[right + 1] < range_max {
            right += 1;
        }
        right - left + 1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::minimum_window_sort(vec![1, 2, 5, 3, 7, 10, 9, 12]),
        5
    );
    assert_eq!(
        Solution::minimum_window_sort(vec![1, 3, 2, 0, -1, 7, 10]),
        5
    );
    assert_eq!(Solution::minimum_window_sort(vec![1, 2, 3]), 0);
    assert_eq!(Solution::minimum_window_sort(vec![3, 2, 1]), 3);
}
