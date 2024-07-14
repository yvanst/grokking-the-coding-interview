/// we will keep one pointer for iterating the array and one pointer for
/// placing the next non-duplicate number. so our algorithm will be to
/// iterate the array and whenever we see a non-duplicate number we move it
/// next to the last non-duplicate number we've seen
struct Solution;

impl Solution {
    fn find_non_duplicate_number_instances(mut nums: Vec<i32>) -> Vec<i32> {
        let mut next_non_duplicate = 1;
        let mut idx = 0;

        while idx < nums.len() {
            if nums[idx] != nums[next_non_duplicate - 1] {
                nums[next_non_duplicate] = nums[idx];
                next_non_duplicate += 1;
            }
            idx += 1;
        }

        nums
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_non_duplicate_number_instances(vec![2, 3, 3, 3, 6, 9, 9]),
        vec![2, 3, 6, 9, 6, 9, 9]
    );
    assert_eq!(
        Solution::find_non_duplicate_number_instances(vec![2, 2, 2, 11]),
        vec![2, 11, 2, 11]
    );
}
