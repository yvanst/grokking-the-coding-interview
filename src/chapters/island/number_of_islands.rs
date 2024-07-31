//! Given a 2D array (i.e., a matrix) containing only 1s (land) and 0s (water), count the number of
//! islands in it.
//!
//! An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water).
//! Each cell is considered connected to other cells horizontally or vertically (not diagonally).

/// we can traverse the matrix linearly to find islands.
///
/// whenever we find a cell with the value 1, we have found an island. using that cell as the root
/// node, we will performan a DFS or BFS search to find all of its connected land cells.
///
/// during our DFS or BFS traversal, we will find and mark all the horizontally and vertically
/// connected land cells.
///
/// we need to have a mechanism to mark each land cell to ensure that each land cell is visited only
/// once. to mark a cell visited, we have two options:
/// 1. we can update the given input matrix. whenever we see a 1, we will make it 0
/// 2. a separate boolean matrix can be used to record whether or not each cell has been visited
struct SolutionInPlaceModification;

impl SolutionInPlaceModification {
    fn number_of_islands(mut matrix: Vec<Vec<u32>>) -> u32 {
        let mut res = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 {
                    res += 1;
                    Self::dfs(&mut matrix, i, j);
                }
            }
        }

        res
    }

    fn dfs(matrix: &mut Vec<Vec<u32>>, i: usize, j: usize) {
        if i >= matrix.len() || j >= matrix[0].len() {
            return;
        }
        if matrix[i][j] == 0 {
            return;
        }
        matrix[i][j] = 0;

        Self::dfs(matrix, i + 1, j);
        Self::dfs(matrix, i.saturating_sub(1), j);
        Self::dfs(matrix, i, j + 1);
        Self::dfs(matrix, i, j.saturating_sub(1));
    }
}

struct SolutionVisitedMatrix;

impl SolutionVisitedMatrix {
    fn number_of_islands(matrix: Vec<Vec<u32>>) -> u32 {
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];
        let mut res = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && !visited_matrix[i][j] {
                    res += 1;
                    Self::dfs(&matrix, &mut visited_matrix, i, j)
                }
            }
        }
        res
    }
    fn dfs(matrix: &Vec<Vec<u32>>, visited_matrix: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if i >= matrix.len() || j >= matrix[0].len() {
            return;
        }
        if matrix[i][j] == 0 || visited_matrix[i][j] {
            return;
        }
        visited_matrix[i][j] = true;
        Self::dfs(matrix, visited_matrix, i + 1, j);
        Self::dfs(matrix, visited_matrix, i, j + 1);
        Self::dfs(matrix, visited_matrix, i.saturating_sub(1), j);
        Self::dfs(matrix, visited_matrix, i, j.saturating_sub(1));
    }
}

/// time complexity of the above algorithm will be O(MN), where M is the number of rows and N is the
/// number of columns.
///
/// because of the visited array and max size of the call stack, the space complexity will be O(MN)
#[test]
fn test1() {
    assert_eq!(
        SolutionInPlaceModification::number_of_islands(vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0]
        ]),
        3
    );

    assert_eq!(
        SolutionInPlaceModification::number_of_islands(vec![
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0]
        ]),
        1
    );
}

#[test]
fn test2() {
    assert_eq!(
        SolutionVisitedMatrix::number_of_islands(vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0]
        ]),
        3
    );

    assert_eq!(
        SolutionVisitedMatrix::number_of_islands(vec![
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0]
        ]),
        1
    );
}
