//! Given a 2D array (i.e., a matrix) containing only 1s (land) and 0s (water), find the biggest
//! island in it. Write a function to return the area of the biggest island.
//!
//! An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water).
//! Each cell is considered connected to other cells horizontally or vertically (not diagonally).

struct Solution;

impl Solution {
    fn biggest_island(matrix: Vec<Vec<u32>>) -> u32 {
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];
        let mut res = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && !visited_matrix[i][j] {
                    res = res.max(Self::dfs(&matrix, &mut visited_matrix, i, j))
                }
            }
        }
        res
    }

    fn dfs(matrix: &Vec<Vec<u32>>, visited_matrix: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
        if i >= matrix.len() || j >= matrix[0].len() {
            return 0;
        }
        if matrix[i][j] == 0 || visited_matrix[i][j] {
            return 0;
        }
        let mut res = 1;
        visited_matrix[i][j] = true;

        // because in the deeper dfs we didn't restore the visited matrix, 
        // once the matrix is set, it won't be counted again
        // so the four branches is not parallel, previous dfs call influence 
        // the later dfs call
        res += Self::dfs(matrix, visited_matrix, i + 1, j);
        res += Self::dfs(matrix, visited_matrix, i, j + 1);
        res += Self::dfs(matrix, visited_matrix, i.saturating_sub(1), j);
        res += Self::dfs(matrix, visited_matrix, i, j.saturating_sub(1));
        res
    }
}

/// time complexity of the above algorithm will be O(MN), where M is the number of rows and N is the
/// number of columns of the input matrix. this is due to the fact that we have to traverse the
/// whole matrix to find the islands
///
/// DFS recursion can go MN deep when the whole matrix is filled with 1s, hence the space complexity
/// will be O(MN).
#[test]
fn test() {
    assert_eq!(
        Solution::biggest_island(vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0]
        ]),
        5
    );
    assert_eq!(
        Solution::biggest_island(vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 0, 1],
            vec![0, 0, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0]
        ]),
        10
    );
}
