//! Any image can be represented by a 2D integer array (i.e., a matrix) where each cell represents
//! the pixel value of the image.
//!
//! Flood fill algorithm takes a starting cell (i.e., a pixel) and a color. The given color is
//! applied to all horizontally and vertically connected cells with the same color as that of the
//! starting cell. Recursively, the algorithm fills cells with the new color until it encounters a
//! cell with a different color than the starting cell.
//!
//! Given a matrix, a starting cell, and a color, flood fill the matrix.

struct Solution;

impl Solution {
    fn flood_fill(mut matrix: Vec<Vec<u32>>, x: usize, y: usize, new_color: u32) -> Vec<Vec<u32>> {
        let old_color = matrix[x][y];
        Self::dfs(&mut matrix, x, y, old_color, new_color);
        matrix
    }
    fn dfs(matrix: &mut Vec<Vec<u32>>, x: usize, y: usize, old_color: u32, new_color: u32) {
        if x >= matrix.len() || y >= matrix[0].len() {
            return;
        }
        if matrix[x][y] != old_color {
            return;
        }
        matrix[x][y] = new_color;
        Self::dfs(matrix, x + 1, y, old_color, new_color);
        Self::dfs(matrix, x, y + 1, old_color, new_color);
        Self::dfs(matrix, x.saturating_sub(1), y, old_color, new_color);
        Self::dfs(matrix, x, y.saturating_sub(1), old_color, new_color);
    }
}

/// time complexity of the above algorithm will be O(MN)
///
/// space complexity is O(MN)
#[test]
fn test() {
    assert_eq!(
        Solution::flood_fill(
            vec![
                vec![0, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 1, 1, 1, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0]
            ],
            1,
            3,
            2
        ),
        vec![
            vec![0, 2, 2, 2, 0],
            vec![0, 0, 0, 2, 2],
            vec![0, 2, 2, 2, 0],
            vec![0, 2, 2, 0, 0],
            vec![0, 0, 0, 0, 0]
        ]
    );

    assert_eq!(
        Solution::flood_fill(
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0]
            ],
            3,
            2,
            5
        ),
        vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 5, 5, 0],
            vec![0, 0, 5, 0, 0],
            vec![0, 0, 5, 0, 0]
        ]
    );
}
