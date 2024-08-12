/// To complete the implementation, we need to implement the partitioning method. We use the
/// following general strategy:
/// - First, we arbitrarily choose a[lo] to be the partitioning item—the one that will go into its
/// final position.
/// - Next, we scan from the left end of the array until we find an entry that is greater than (or
/// equal to) the partitioning item, and we scan from the right end of the array until we find an
/// entry less than (or equal to) the partitioning item.
/// - The two items that stopped the scans are out of place in the final partitioned array, so we
/// exchange them.
/// - When the scan indices cross, all that we need to do to complete the partitioning process is to
/// exchange the partitioning item a[lo] with the rightmost entry of the left subarray (a[j]) and
/// return its index j.
struct Solution;

impl Solution {
    fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let (low, high) = (0, nums.len() - 1);
        Self::inner(&mut nums, low, high);
        nums
    }
    fn inner(nums: &mut Vec<i32>, low: usize, high: usize) {
        if low >= high {
            return;
        }
        let pivot = Self::partition(nums, low, high);
        Self::inner(nums, low, pivot - 1);
        Self::inner(nums, pivot + 1, high);
    }

    fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
        let v = nums[low];
        let mut left = low;
        let mut right = high;
        loop {
            while left < high && nums[left] <= v {
                left += 1;
            }
            while right > low && nums[right] >= v {
                right -= 1;
            }
            if left >= right {
                break;
            }
            nums.swap(left, right);
        }
        nums.swap(low, right);
        right
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::quick_sort(vec![8, 7, 2, 1, 0, 9, 6]),
        vec![0, 1, 2, 6, 7, 8, 9]
    );
}
