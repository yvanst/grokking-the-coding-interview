/// we can use a Two Pointers approach while iterating through the array. let's
/// say the two pointers are called low and high which are pointing to the first
/// and the last element of the array respectively.
/// so while iterating, we will move all 0s before low and all 2s after high so that
/// in the end, all 1s will be between low and high. in the end, all 0s are on the
/// left and, all 1s are in the middle, and all 2s are on the right.
///
/// we start by initializing three variables: low to 0, high to the last index of the
/// array, and i also to be 0. low is meant to keep track of the latest position
/// where a 0 should be placed, high is meant to keep track of the latest position
/// where a 2 should be placed, and i is used to iterate through the array
///
/// we then start a loop that continues as long as i is less than or equal to high.
/// in each iteration of the loop, we check the value of the array at the index i
///
/// if the value is 0, we swap the values at the indices i and low. we then increment
/// both i and low, since we know that the new element at low is 0 and we can move to
/// the next index
///
/// if the value is 1, we do nothing other than increment i. this is because we want
/// 1s to be in the middle of the array.
///
/// if the value is 2, we swap the values at i and high. however, after the swap, we
/// only decrement high without incrementing i. this is because the new value at index
/// i could be 0, 1, or 2, and we need to check this value again in the next iteration.
struct Solution;

impl Solution {
    fn dutch_national_flag_problem(mut nums: Vec<u32>) -> Vec<u32> {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut i = 0;
        // we need to deal with the equal case, if it's 1 or 2, the swap didn't bother
        // but if it's 0, we need to swap with 0
        while i <= high {
            match nums[i] {
                0 => {
                    nums.swap(low, i);
                    low += 1;
                    i += 1;
                }
                1 => {
                    i += 1;
                }
                2 => {
                    nums.swap(high, i);
                    high -= 1;
                }
                _ => panic!(),
            }
        }
        nums
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::dutch_national_flag_problem(vec![1, 0, 2, 1, 0]),
        vec![0, 0, 1, 1, 2]
    );
    assert_eq!(
        Solution::dutch_national_flag_problem(vec![2, 2, 0, 1, 2, 0]),
        vec![0, 0, 1, 2, 2, 2]
    );
}
