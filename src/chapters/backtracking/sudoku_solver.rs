//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy all of the following rules:
//! - Each of the digits 1-9 must occur exactly once in each row.
//! - Each of the digits 1-9 must occur exactly once in each column.
//! - Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
//! The '.' character indicates empty cells.

use std::sync::atomic::AtomicU32;
/// here are the steps to solve a sudoku puzzle using backtracking:
/// 1. start with an empty 9x9 grid and fill in the numbers that are given in the puzzle.
/// 2. from left to right and top to bottom, find the first empty cell in the grid.
/// 3. try out every number from 1 to 9 as possible solution for the empty cell
/// 4. before filling in a number, check if it is a valid solution by checking if the number
///    is already present in the same row, column, or 3x3 sub-grid
/// 5. if the number is valid, fill it in the cell and recursively call the function to move
///    to the next empty cell
/// 6. if the number is not valid, backtrack to the previous state and try a different number
/// 7. if a valid solution is found, return true. if no solution is found, return false.
/// 8. if the function returns true, the grid is filled with a valid solution. else, the puzzle
///    is unsolvable
struct Solution;

static COUNT: AtomicU32 = AtomicU32::new(0);

impl Solution {
    fn sudoku_solver(mut board: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
        if Self::dfs(&mut board) {
            dbg!(&COUNT);
            Some(board)
        } else {
            None
        }
    }

    // the final result board is the side effect, we need a signal to denote if success, so bool
    // here
    fn dfs(board: &mut Vec<Vec<char>>) -> bool {
        COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == '.' {
                    for ch in '1'..='9' {
                        if Self::is_valid(board, i, j, ch) {
                            board[i][j] = ch;
                            // early return, save the final result
                            // we don't return on false case, rather, we try the next iteration
                            if Self::dfs(board) {
                                return true;
                            };
                        }
                    }
                    // tried all 0..=9 case, didn't hit early return,
                    // must something wrong previously, we return false here
                    // correspondes to the previous return true
                    board[i][j] = '.';
                    return false;
                }
            }
        }
        // edge case
        // didn't enter the board[i][j] = '.' case, return true
        // short hand for doing the if board is full check at the beginning.
        // even if we add a full check at the beginning,
        // we need to deal with the for loop didn't meet a return statement,
        // so we need a true here any way
        true
    }

    fn is_valid(board: &[Vec<char>], i: usize, j: usize, ch: char) -> bool {
        // we can do the is_valid check in one loop
        for n in 0..9 {
            if board[i][n] == ch {
                return false;
            }
            if board[n][j] == ch {
                return false;
            }
            // find the region the cell belongs to (i/3)*3 (j/3)*3  -> e.g., (0,0), (3,3), (6,6)
            // (n/3)(n%3) is all the combination of (0..3)*(0..3),
            // thus we can traverse all the cell in the region.
            if board[(i / 3) * 3 + n / 3][(j / 3) * 3 + n % 3] == ch {
                return false;
            }
        }
        true
    }
}

/// since the board size is fixed, the time complexity is O(1), as there is no variable input.
/// though the number of operations needed is (9!)^9.
///
/// the board size is fixed, so the space complexity is O(1).
#[test]
fn test() {
    assert_eq!(
        Solution::sudoku_solver(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ])
        .unwrap(),
        vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
        ]
    );

    assert_eq!(
        Solution::sudoku_solver(vec![
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '6', '.', '.', '.', '.', '.'],
            vec!['.', '7', '.', '.', '9', '.', '2', '.', '.'],
            vec!['.', '5', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '.', '4', '5', '7', '.', '.'],
            vec!['.', '.', '.', '1', '.', '.', '.', '3', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '6', '8'],
            vec!['.', '.', '8', '5', '.', '.', '.', '1', '.'],
            vec!['.', '9', '.', '.', '.', '.', '4', '.', '.']
        ])
        .unwrap(),
        vec![
            vec!['8', '1', '2', '7', '5', '3', '6', '4', '9'],
            vec!['9', '4', '3', '6', '8', '2', '1', '7', '5'],
            vec!['6', '7', '5', '4', '9', '1', '2', '8', '3'],
            vec!['1', '5', '4', '2', '3', '7', '8', '9', '6'],
            vec!['3', '6', '9', '8', '4', '5', '7', '2', '1'],
            vec!['2', '8', '7', '1', '6', '9', '5', '3', '4'],
            vec!['5', '2', '1', '9', '7', '4', '3', '6', '8'],
            vec!['4', '3', '8', '5', '2', '6', '9', '1', '7'],
            vec!['7', '9', '6', '3', '1', '8', '4', '5', '2'],
        ]
    );
}
