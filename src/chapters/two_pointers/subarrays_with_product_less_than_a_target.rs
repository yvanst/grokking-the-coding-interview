/// this problem follows the sliding window and the two pointers pattern and shares
/// similarities with triplets with smaller sum
///
/// the window starts from the leftmost side of the array, both left and right at
/// position 0, and slides to the right one element at a time, expanding the window.
/// this expansion is represented by incrementing right
///
/// as we add a new element into the window, we multiply our current product with this
/// new element
///
/// if at any point the product of the numbers within the window becomes greater than
/// or equal to the target, we need to make the product smaller, this is achieved by moving
/// the left pointer to the right, effectively excluding the left element from our window,
/// and dividing our product by this removed element. we keep doing this until our product
/// is less than target again
///
/// for each position of right, we create all possible subarrays that end at right, and
/// that have a product less than the target. we do this by starting with a subarray that
/// only includes the right element, then progressively adding the element to its left, and
/// so on, until we reach the left pointer. all of these subarrays are added to the result
///
/// in summary, this algorithm slides a window across the array, expanding and contracting
/// this window as necessary, to find all subarrays where the product of the numbers is less
/// than a target value. for each window, it generates all valid subarrays ending at the right
/// edge of the window
struct Solution;

impl Solution {
    fn subarrays_with_product_less_than_a_target(nums: Vec<u32>, target: u32) -> Vec<Vec<u32>> {
        let mut res = Vec::new();
        let mut left = 0;
        let mut product = 1;

        // right in the outer loop, we are trying to solve a smaller problem
        for right in 0..nums.len() {
            product *= nums[right];

            // find a max region from left to right such that the product of all nums within
            // smaller than target
            while left <= right && product >= target {
                product /= nums[left];
                left += 1;
            }

            // append all the subarrays in the left..right to product
            // the reverse is necessary, it determines whether the leftmost element of the subarry
            // is in the result, or the rightmost element.
            let mut tmp_array = Vec::new();
            for i in (left..=right).rev() {
                tmp_array.insert(0, nums[i]);
                res.push(tmp_array.clone());
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::subarrays_with_product_less_than_a_target(vec![2, 5, 3, 10], 30),
        vec![vec![2], vec![5], vec![2, 5], vec![3], vec![5, 3], vec![10]]
    );

    assert_eq!(
        Solution::subarrays_with_product_less_than_a_target(vec![8, 2, 6, 5], 50),
        vec![
            vec![8],
            vec![2],
            vec![8, 2],
            vec![6],
            vec![2, 6],
            vec![5],
            vec![6, 5]
        ]
    );
}
