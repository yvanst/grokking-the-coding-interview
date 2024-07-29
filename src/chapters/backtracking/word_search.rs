//! Given an m x n grid of characters board and a string word, return true if the word exists in the
//! grid.
//!
//! The word can be constructed from letters of sequentially adjacent cells, where adjacent cells
//! are horizontally or vertically neighboring. The same letter cell may not be used more than once.

/// the basic approach to solving the work search problem using backtrack is to start at the first
/// character of the word and check all 8 adjacent cells in the grid to see if any of them match the
/// next character of the word
///
/// if a match is found, mark the cell as visited and recursively check the next character of the
/// word in the adjacent cells of the newly visited cell.
///
/// if the entire word is found, return true.
///
/// if no match is found, backtrack to the previous cell and try a different path
///
/// repeat this process until the entire grid has been searched or the word is found
struct Solution;

impl Solution {
    fn word_search(mut board: Vec<Vec<char>>, word: String) -> bool {
        // word search, turn String to Vec<char> for easy use
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                // use if to expand the search space, give another try in the next loop
                if Solution::dfs(&mut board, &word, 0, i, j) {
                    return true;
                }
            }
        }
        false
    }

    /// dfs takes three additional parameters:
    ///   i and j are the current coordinates of the cell that is being visited
    ///   k is the index of the current character of the word being matched
    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], k: usize, i: usize, j: usize) -> bool {
        if i >= board.len() || j >= board[0].len() || board[i][j] != word[k] {
            return false;
        }

        if k == word.len() - 1 {
            return true;
        }
        let current = board[i][j];

        // mark the element as visited, because the rules says the same letter cell may not be used
        // more than once
        //
        // change the input parameter to mark subsequent invalid operations is ok in the backtrack
        // program
        board[i][j] = '/';

        // the dfs checks if the next character of the word exists in the 4 adjacent cells, and it
        // will mark the cell as visited and move to the next index of the word by incrementing k by
        // 1
        //
        // if the next character is found, the function returns true, if not it backtracks to the
        // previous cell.
        //
        // if the entire word is found, the function returns true, otherwise it return false
        //
        // when use the saturating_sub, we can skip the index less than zero check; and becaues we
        // mark the character when index = 0 as '/', it will not match the following character even
        // though we pass the 0 again
        let res = Self::dfs(board, word, k + 1, i + 1, j)
            || Self::dfs(board, word, k + 1, i, j + 1)
            || Self::dfs(board, word, k + 1, i.saturating_sub(1), j)
            || Self::dfs(board, word, k + 1, i, j.saturating_sub(1));

        // if not exist early, it means all the previous path couldn't work, we restore the site
        board[i][j] = current;
        res
    }
}

/// the overall time complexity of the algorithm is O(MN4^L), MN is the number of cells in the
/// board, 4^L indicates that each cell can lead to up to 4 recursive calls for a word of length L.
///
/// the space complexity of the function is O(L), where N is the length of the word. the maximum
/// number of function calls at any point in time is N.
#[test]
fn test() {
    assert!(Solution::word_search(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "ABCCED".to_string()
    ));

    assert!(Solution::word_search(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "SEE".to_string()
    ));

    assert!(!Solution::word_search(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ],
        "ABCB".to_string()
    ));

    assert!(!Solution::word_search(
        vec![vec!['a', 'a']],
        "aaa".to_string()
    ));

    assert!(Solution::word_search(vec![vec!['a']], "a".to_string()));

    assert!(Solution::word_search(
        vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
            vec!['z', 'a', 'b', 'c', 'd']
        ],
        "abcde".to_string()
    ));
    assert!(Solution::word_search(
        vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
            vec!['z', 'a', 'b', 'c', 'd']
        ],
        "zabcd".to_string()
    ));
}
