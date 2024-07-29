//! Given an array of points in a 2D plane, find ‘K’ closest points to the origin.

use std::collections::BinaryHeap;
/// this problem follows the top K numbers pattern. the only difference in this problem is that we
/// need to find the closest point to the origin as compared to finding the largest numbers.
struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapElement {
    distance: i32, // order matters due to derive Ord
    point: (i32, i32),
}

impl Solution {
    fn k_closest_points_to_the_origin(points: Vec<(i32, i32)>, k: usize) -> Vec<(i32, i32)> {
        let mut heap = BinaryHeap::new();
        for point in points {
            if heap.len() < k {
                heap.push(HeapElement {
                    distance: point.0.pow(2) + point.1.pow(2),
                    point,
                })
            } else {
                let point_distance = point.0.pow(2) + point.1.pow(2);
                if heap
                    .peek()
                    .filter(|top| top.distance > point_distance)
                    .is_some()
                {
                    heap.pop();
                    heap.push(HeapElement {
                        distance: point_distance,
                        point,
                    })
                }
            }
        }
        heap.into_iter().map(|element| element.point).collect()
    }
}

/// the time complexity of this algorithm is O(N*logK) as we iterating all points and pushing them
/// into the heap
///
/// the space complexity will be O(K) beacause we need to store K points in the heap
#[test]
fn test() {
    assert_eq!(
        Solution::k_closest_points_to_the_origin(vec![(1, 3), (3, 4), (2, -1)], 2),
        vec![(1, 3,), (2, -1,),]
    );
}
