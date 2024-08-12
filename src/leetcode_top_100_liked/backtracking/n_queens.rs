//! 按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。
//! n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
//! 给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。
//! 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

/// - 为了降低总时间复杂度，每次放置皇后时需要快速判断每个位置是否可以放置皇后，显然，最理想的情况是在 O(1)
///   的时间内判断该位置所在的列和两条斜线上是否已经有皇后。
/// - 为了判断一个位置所在的列和两条斜线上是否已经有皇后，使用三个集合 columns、diagonals1 和 diagonals2
///   分别记录每一列以及两个方向的每条斜线上是否有皇后。
/// - 从左上到右下方向，同一条斜线上的每个位置满足行下标和列下标之差相等，例如(0,0)和(3,3)在同一斜线上。
/// - 从右上到左下方向，同一条斜线上的每个位置满足行下标和列下标之和相等，例如(3,0)和(1,2)在同一斜线上。
/// - 每次放置皇后时，对于每个位置判断其是否在三个集合中，如果三个集合都不包含当前位置，则当前位置是可以放置
///   皇后的位置
struct Solution;

impl Solution {
    fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n]; n];
        let mut res = Vec::new();

        Self::backtrack(&mut board, &mut res, 0, n);
        res
    }

    fn backtrack(board: &mut Vec<Vec<char>>, res: &mut Vec<Vec<String>>, row: usize, n: usize) {
        if row == n {
            let res_board = board
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect();
            res.push(res_board);
            return;
        }

        for col in 0..n {
            if Self::is_valid(board, row, col, n) {
                board[row][col] = 'Q';
                Self::backtrack(board, res, row + 1, n);
                board[row][col] = '.';
            }
        }
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 'Q' && (j == col || i + j == row + col || i + col == row + j) {
                    return false;
                }
            }
        }

        true
    }
}
struct SolutionHashSet;
use std::collections::HashSet;

impl SolutionHashSet {
    fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n]; n];
        let mut res = Vec::new();
        let mut col_hashset = HashSet::new();
        let mut diag_hashset = HashSet::new();
        let mut back_diag_hashset = HashSet::new();

        Self::backtrack(
            &mut board,
            &mut res,
            0,
            n,
            &mut col_hashset,
            &mut diag_hashset,
            &mut back_diag_hashset,
        );
        res
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        res: &mut Vec<Vec<String>>,
        row: usize,
        n: usize,
        col_hashset: &mut HashSet<usize>,
        diag_hashset: &mut HashSet<usize>,
        back_diag_hashset: &mut HashSet<usize>,
    ) {
        if row == n {
            let res_board = board
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect();
            res.push(res_board);
            return;
        }

        for col in 0..n {
            if Self::is_valid_with_set(row, col, n, col_hashset, diag_hashset, back_diag_hashset) {
                board[row][col] = 'Q';
                col_hashset.insert(col);
                diag_hashset.insert(n + row - col);
                back_diag_hashset.insert(row + col);
                Self::backtrack(
                    board,
                    res,
                    row + 1,
                    n,
                    col_hashset,
                    diag_hashset,
                    back_diag_hashset,
                );
                board[row][col] = '.';
                col_hashset.remove(&col);
                diag_hashset.remove(&(n + row - col));
                back_diag_hashset.remove(&(row + col));
            }
        }
    }

    fn is_valid_with_set(
        row: usize,
        col: usize,
        n: usize,
        col_hashset: &HashSet<usize>,
        diag_hashset: &HashSet<usize>,
        back_diag_hashset: &HashSet<usize>,
    ) -> bool {
        !col_hashset.contains(&col)
            && !diag_hashset.contains(&(n + row - col))
            && !back_diag_hashset.contains(&(row + col))
    }
}

#[test]
fn test() {
    assert_eq!(Solution::solve_n_queens(8).len(), 92);
    assert_eq!(SolutionHashSet::solve_n_queens(8).len(), 92);
}
