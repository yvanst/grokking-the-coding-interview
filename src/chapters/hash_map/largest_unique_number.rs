use std::collections::BTreeMap;

struct Solution;

impl Solution {
    fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut btreemap = BTreeMap::new();
        for num in nums.iter() {
            btreemap.entry(*num).and_modify(|v| *v += 1).or_insert(1);
        }
        for (k, v) in btreemap.iter().rev() {
            if *v == 1 {
                return *k;
            }
        }
        -1
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::largest_unique_number(vec![5, 7, 3, 7, 5, 8]), 8);
    assert_eq!(
        Solution::largest_unique_number(vec![1, 2, 3, 2, 1, 4, 4]),
        3
    );
    assert_eq!(Solution::largest_unique_number(vec![9, 9, 8, 8, 7, 7]), -1);
}
