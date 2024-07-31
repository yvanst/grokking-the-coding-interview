//! You are given a 2D matrix containing only 1s (land) and 0s (water).
//!
//! An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water).
//! Each cell is considered connected to other cells horizontally or vertically (not diagonally).
//!
//! A closed island is an island that is totally surrounded by 0s (i.e., water). This means all
//! horizontally and vertically connected cells of a closed island are water. This also means that,
//! by definition, a closed island can't touch an edge (as then the edge cells are not connected to
//! any water cell).
//!
//! Write a function to find the number of closed islands in the given matrix.

/// the question follows the Island pattern and is quite similar to Number of Islands problem.
///
/// we will traverse the matrix linearly to find the islands. we can use the DFS or BFS to traverse
/// an island. i.e., to find all of its connected land cells
///
/// to decide if an island is a closed island, we need to ensure two things when traversing:
/// 1. the island does not touch an edge
/// 2. outside boundary of the island are water cells
///
/// for the first condition, whenever we go outside the boundary of the matrix during the search, it
/// means one of the cells of the island is touching an edge.
/// for the second conditino, we need to ensure that all the boundary cells of the island are water.
/// i.e., encouter water, `if matrix[x][y] == 0 || visited[x][y]`
struct Solution;

impl Solution {
    fn number_of_closed_islands(matrix: Vec<Vec<i32>>) -> u32 {
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];
        let mut res = 0;

        for i in 0..visited_matrix.len() {
            for j in 0..visited_matrix[0].len() {
                if matrix[i][j] == 1
                    && !visited_matrix[i][j]
                    && Self::dfs(&matrix, &mut visited_matrix, i, j)
                {
                    res += 1;
                }
            }
        }

        res
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        visited_matrix: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
    ) -> bool {
        if (i == 0 || j == 0 || i == matrix.len() - 1 || j == matrix[0].len() - 1)
            && matrix[i][j] == 1
        {
            return false;
        }
        if matrix[i][j] == 0 || visited_matrix[i][j] {
            return true;
        }
        visited_matrix[i][j] = true;

        let res1 = Self::dfs(matrix, visited_matrix, i + 1, j);
        let res2 = Self::dfs(matrix, visited_matrix, i, j + 1);
        let res3 = Self::dfs(matrix, visited_matrix, i.saturating_sub(1), j);
        let res4 = Self::dfs(matrix, visited_matrix, i, j.saturating_sub(1));
        res1 && res2 && res3 && res4
    }
}

/// time complexity of the above algorithm will be O(MN), where M is the number of rows and N is the
/// number of columns of the input matrix. this is due to the fact that we have to traverse the
/// whole matrix to find islands
///
/// DFS recursion stack can go MN deep when the whole matrix is filled with 1s. hence, the space
/// complexity will be O(MN)
#[test]
fn test() {
    assert_eq!(
        Solution::number_of_closed_islands(vec![
            vec![1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ]),
        1
    );
    assert_eq!(
        Solution::number_of_closed_islands(vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        2
    );
}
