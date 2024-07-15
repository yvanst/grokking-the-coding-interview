struct Solution;

impl Solution {
    fn longest_subarray_with_ones_after_replacement(nums: Vec<i32>, k: u32) -> usize {
        let mut res = 0;
        let mut left = 0;
        let mut frequency_counter = [0; 2];

        for right in 0..nums.len() {
            if nums[right] == 0 {
                frequency_counter[0] += 1;
            } else {
                frequency_counter[1] += 1;
            }

            while frequency_counter[0] > k {
                if nums[left] == 0 {
                    frequency_counter[0] -= 1;
                } else {
                    frequency_counter[1] -= 1;
                }
                left += 1
            }
            res = res.max(right - left + 1);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_subarray_with_ones_after_replacement(
            vec![0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1],
            2
        ),
        6
    );

    assert_eq!(
        Solution::longest_subarray_with_ones_after_replacement(
            vec![0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1],
            3
        ),
        9
    );
}
