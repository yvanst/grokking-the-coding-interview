/// this problem follows the Two Pointers pattern and is quite similar to "Triplet
/// Sum to Zero".
///
/// we can follow a similar approach to iterate through the array, taking one number
/// at a time. at every step, we will save the difference between the triplet and the
/// target number, so that in the end, we can return the triplet with the closet sum
struct Solution;

impl Solution {
    fn triplet_sum_close_to_target(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut res = i32::MAX;
        // without loss of generality, select the smallest one as the first number,
        // the index of second number should at least be idx + 1
        nums.sort();

        for (idx, &n) in nums.iter().enumerate() {
            Self::search(&nums, target, n, idx + 1, nums.len() - 1, &mut res)
        }
        res
    }

    fn search(
        nums: &[i32],
        target: i32,
        n: i32,
        mut left: usize,
        mut right: usize,
        close_sum: &mut i32,
    ) {
        while left < right {
            let candidate = n + nums[left] + nums[right];
            let diff = target - candidate;
            match diff {
                0 => {
                    *close_sum = 0;
                    break;
                }
                n if n.abs() < (target - *close_sum).abs() => {
                    *close_sum = candidate;
                }
                n if n.abs() == (target - *close_sum).abs() => {
                    if candidate < *close_sum {
                        *close_sum = candidate;
                    }
                }
                _ => {}
            }
            if diff > 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::triplet_sum_close_to_target(vec![-1, 0, 2, 3], 3),
        2
    );
    assert_eq!(
        Solution::triplet_sum_close_to_target(vec![-3, -1, 1, 2], 1),
        0
    );
    assert_eq!(
        Solution::triplet_sum_close_to_target(vec![1, 0, 1, 1], 100),
        3
    );
    assert_eq!(
        Solution::triplet_sum_close_to_target(vec![0, 0, 1, 1, 2, 6], 5),
        4
    );
}
