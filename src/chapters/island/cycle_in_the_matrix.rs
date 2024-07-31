//! You are given a 2D matrix containing different characters, you need to find if there exists any
//! cycle consisting of the same character in the matrix.
//!
//! A cycle is a path in the matrix that starts and ends at the same cell and has four or more
//! cells. From a given cell, you can move to one of the cells adjacent to it - in one of the four
//! directions (up, down, left, or right), if it has the same character value of the current cell.
//!
//! Write a function to find if the matrix has a cycle.

/// the question follows the Island pattern and is quite similar to Number of Islands problem.
///
/// whenever we reach a cell that have already been visited, we can conclude that we have found a
/// cycle. this also means that we need to be careful to not start traversing the parent cell and
/// wrongly finding a cycle.
/// that is, while traversing, when initiating DFS recursive calls to all the neighboring cell, we
/// should not start a DFS call to the previous cell.
struct Solution;

impl Solution {
    fn cycle_in_the_matrix(matrix: Vec<Vec<char>>) -> bool {
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if !visited_matrix[i][j]
                    && Self::dfs(
                        &matrix,
                        &mut visited_matrix,
                        matrix[i][j],
                        i,
                        j,
                        matrix.len(),
                        matrix[0].len(),
                    )
                {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        matrix: &Vec<Vec<char>>,
        visited_matrix: &mut Vec<Vec<bool>>,
        ch: char,
        i: usize,
        j: usize,
        prev_i: usize,
        prev_j: usize,
    ) -> bool {
        if i >= matrix.len() || j >= matrix[0].len() {
            return false;
        }
        // must test the character match before the visited match
        if matrix[i][j] != ch {
            return false;
        }
        if visited_matrix[i][j] {
            return true;
        }

        visited_matrix[i][j] = true;
        let mut res = false;

        if i + 1 != prev_i {
            res |= Self::dfs(matrix, visited_matrix, ch, i + 1, j, i, j);
        }
        if j + 1 != prev_j {
            res |= Self::dfs(matrix, visited_matrix, ch, i, j + 1, i, j);
        }
        // this i > 0 then i - 1 test won't make us miss any valid search,
        // just move the base case upfront.
        if i > 0 && i - 1 != prev_i {
            res |= Self::dfs(matrix, visited_matrix, ch, i - 1, j, i, j);
        }
        if j > 0 && j - 1 != prev_j {
            res |= Self::dfs(matrix, visited_matrix, ch, i, j - 1, i, j);
        }
        res
    }
}

/// time complexity of the above algorithm will be O(MN), where M is the number of rows and N is the
/// number of columns of the input matrix. this is due to the fact that we have to traverse the
/// whole matrix to find cycles.
///
/// DFS recursion stack can go MN deep when the whole matrix is filled with the same character.
/// hence, the space complexity will be O(MN).
#[test]
fn test() {
    assert!(Solution::cycle_in_the_matrix(vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['b', 'a', 'c', 'a'],
        vec!['b', 'a', 'c', 'a'],
        vec!['b', 'a', 'a', 'a'],
    ]));
    assert!(Solution::cycle_in_the_matrix(vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'b', 'a', 'a'],
        vec!['a', 'a', 'a', 'c']
    ]));
    assert!(!Solution::cycle_in_the_matrix(vec![
        vec!['a', 'b', 'e', 'b'],
        vec!['b', 'b', 'b', 'b'],
        vec!['b', 'c', 'c', 'd'],
        vec!['c', 'c', 'd', 'd']
    ]));
}
