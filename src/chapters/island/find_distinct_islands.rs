//! You are given a 2D matrix containing only 1s (land) and 0s (water).
//!
//! An island is a connected set of 1s (land) and is surrounded by either an edge or 0s (water).
//! Each cell is considered connected to other cells horizontally or vertically (not diagonally).
//!
//! Two islands are considered the same if and only if they can be translated (not rotated or
//! reflected) to equal each other.
//!
//! Write a function to find the number of distinct islands in the given matrix.

/// the question follows the Island pattern and is quite similar to Number of Islands problem.
///
/// if two islands are same, their traversal path should be same too. this means that if we perform
/// a DFS or BFS on two equal islands starting from their top-left cell, the traversal pattern
/// should be exactly same for both the islands
///
/// while traversing an island, we can construct a string that maps the traversal path of the
/// island. we can use U for up, D for Down, L for left, R for right, O for start, B for back.
///
/// we can start inserting these traversal strings of each island in a HashSet. this will ensure
/// that we will not have any duplicate traversal string in the HashSet, thus giving us distinct
/// islands.
///
/// when we finish traversing the matrix, the HashSet will contain the distinct traversal path of
/// all islands.
struct Solution;

use std::collections::HashSet;

impl Solution {
    fn find_distinct_islands(matrix: Vec<Vec<i32>>) -> u32 {
        let mut hash_set = HashSet::new();
        let mut visited_matrix = vec![vec![false; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && !visited_matrix[i][j] {
                    let mut path = Vec::new();
                    Self::dfs(&matrix, &mut visited_matrix, i, j, &mut path);
                    hash_set.insert(String::from_iter(path));
                }
            }
        }

        // dbg!(&hash_set);
        hash_set.len() as u32
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        visited_matrix: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        path: &mut Vec<char>,
    ) {
        if i >= matrix.len() || j >= matrix[0].len() {
            return;
        }
        if matrix[i][j] == 0 || visited_matrix[i][j] {
            return;
        }
        visited_matrix[i][j] = true;

        path.push('R');
        Self::dfs(matrix, visited_matrix, i + 1, j, path);

        path.push('D');
        Self::dfs(matrix, visited_matrix, i, j + 1, path);

        path.push('L');
        Self::dfs(matrix, visited_matrix, i.saturating_sub(1), j, path);

        path.push('U');
        Self::dfs(matrix, visited_matrix, i, j.saturating_sub(1), path);

        path.push('B');
    }
}

/// time complexity of the above algorithm will be O(MN), where M is the number of rows and N is the
/// number of columns of the input matrix. this is due to the fact that we have to traverse the
/// whole matrix to find islands
///
/// DFS recursion stack can go M*N deep when the whole matrix is filled with 1s. hence, the space
/// complexity will be O(MN)
#[test]
fn test() {
    assert_eq!(
        Solution::find_distinct_islands(vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 1, 0, 1]
        ]),
        2
    );
    assert_eq!(
        Solution::find_distinct_islands(vec![
            vec![1, 1, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0]
        ]),
        2
    );
}
