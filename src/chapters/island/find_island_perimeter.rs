//! You are given a 2D matrix containing only 1s (land) and 0s (water).
//!
//! An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water).
//! Each cell is considered connected to other cells horizontally or vertically (not diagonally).
//!
//! There are no lakes on the island, so the water inside the island is not connected to the water
//! around it. A cell is a square with a side length of 1..
//!
//! The given matrix has only one island, write a function to find the perimeter of that island.

/// the question follows the Island pattern and is quite similar to Number of Islands problem
///
/// each cell has four sides. each side of an island can be shared with another cell, we can include
/// the side in the island perimeter only if the other cell is a water.
///
/// also if a cell side is on boundary, we should include that side in the perimeter
///
/// -> basically we are counting the number of outer 0 cells
struct Solution;

impl Solution {
    fn find_island_perimeter(matrix: Vec<Vec<u32>>) -> u32 {
        let mut res = 0;
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && !visited_matrix[i][j] {
                    res += Self::dfs(&matrix, &mut visited_matrix, i, j)
                }
            }
        }

        res
    }
    fn dfs(matrix: &Vec<Vec<u32>>, visited_matrix: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
        if i >= matrix.len() || j >= matrix[0].len() {
            return 1;
        }
        // we are traversing and passing the count, once the cell is marked as visited, it means all
        // leaf nodes that the current node can reach have been entountered
        if visited_matrix[i][j] {
            return 0;
        }
        if matrix[i][j] == 0 {
            return 1;
        }
        visited_matrix[i][j] = true;
        let mut res = 0;
        res += Self::dfs(matrix, visited_matrix, i + 1, j);
        res += Self::dfs(matrix, visited_matrix, i, j + 1);
        if i > 0 {
            res += Self::dfs(matrix, visited_matrix, i - 1, j);
        } else {
            res += 1;
        }
        if j > 0 {
            res += Self::dfs(matrix, visited_matrix, i, j - 1);
        } else {
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_island_perimeter(vec![
            vec![1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ]),
        14
    );
    assert_eq!(
        Solution::find_island_perimeter(vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 0, 0]
        ]),
        12
    );
}
