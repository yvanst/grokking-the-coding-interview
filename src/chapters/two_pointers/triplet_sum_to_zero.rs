/// to follow a similar approach, first, we will sort the array and then iterate
/// through it taking one number at a time. let's say during our iteration we are
/// at number X, so we need to find Y and Z such that X + Y + Z == 0, at this stage,
/// our problem translates into finding a pair whose sums is equal to -X
///
/// another difference from Pair with Target Sum is that we need to find all the unique
/// triplets. to handle this, we have to skip any duplicate number. since we will be
/// sorting the array, so all the duplicate numbers will be next to each other and are
/// easier to skip.
///
/// step-by-step algorithm
/// 1. sort the array:
///    - sort the input array in non-decreasing order. this helps in easily skipping
///      duplicate elements and applying the two-pointer technique
/// 2. iterate through the array:
///    - use a for loop to iterate through the array, stopping at the third-to-last
///      element.
///    - for each element, check if it's the same as the previous one to avoid duplicates.
///    - if it's the same, skip to the next iteration
/// 3. fix the current element and find pairs
///    - fix the current element and use two pointers to find pairs whose sum is the
///      negative of the fixed element
///    - the left pointer starts from the next element and right pointer starts from the
///      end of the array
/// 4. find pairs with two pointers
///    - calculate the sum of the left pointer and the right pointer
///    - if the current_sum equals target_sum, add the triplet to the list and move both
///      pointers to the next unique elements
///    - if current_sum is less than target_sum, move the left pointer to the right to
///      increase the sum
///    - if current_sum is greater than target_sum, move the right pointer to the left to
///      decrease the sum
/// 5. skip duplicates:
///    - ensure that the left and right pointers skip duplicate elements to avoid counting
///      the same triplet multiple tiems
/// 6. return the result
///    - after processing all elements, return the list of unique triplets.
struct Solution;

impl Solution {
    fn triplet_sum_to_zero(mut nums: Vec<i32>) -> Vec<(i32, i32, i32)> {
        let mut res = Vec::new();
        nums.sort();
        for (idx, &n) in nums.iter().enumerate() {
            if idx > 0 && n == nums[idx - 1] {
                continue;
            }
            Self::search(&nums, -n, idx + 1, nums.len() - 1, &mut res);
        }
        res
    }
    fn search(
        nums: &[i32],
        target: i32,
        mut left: usize,
        mut right: usize,
        res: &mut Vec<(i32, i32, i32)>,
    ) {
        while left < right {
            match (nums[left] + nums[right]).cmp(&target) {
                std::cmp::Ordering::Equal => {
                    res.push((-target, nums[left], nums[right]));
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    right -= 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::triplet_sum_to_zero(vec![-3, 0, 1, 2, -1, 1, -2]),
        vec![(-3, 1, 2,), (-2, 0, 2,), (-2, 1, 1,), (-1, 0, 1,),]
    );

    assert_eq!(
        Solution::triplet_sum_to_zero(vec![-5, 2, -1, -2, 3]),
        vec![(-5, 2, 3,), (-2, -1, 3,),]
    );
}
