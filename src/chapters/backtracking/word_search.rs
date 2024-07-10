use std::cell::RefCell;
use std::rc::Rc;
/// the basic approach to solving the work search problem using backtrack is to
/// start at the first character of the word and check all 8 adjacent cells in
/// the grid to see if any of them match the next character of the word
///
/// if a match is found, mark the cell as visited and recursively check the next
/// character of the word in the adjacent cells of the newly visited cell.
///
/// if the entire word is found, return true.
///
/// if no match is found, backtrack to the previous cell and try a different path
///
/// repeat this process until the entire grid has been searched or the word is found
struct Solution;

impl Solution {
    fn word_search(board: Vec<Vec<char>>, word: String) -> bool {
        let board = Rc::new(RefCell::new(board));
        // word search, turn String to Vec<char> for easy use
        let word = Rc::new(word.chars().collect::<Vec<_>>());
        let (n, m) = {
            let borrow = board.borrow();
            (borrow.len(), borrow[0].len())
        };
        for i in 0..n {
            for j in 0..m {
                if Solution::dfs(board.clone(), word.clone(), 0, i, j) {
                    return true;
                }
            }
        }
        false
    }

    /// dfs takes three additional parameters:
    ///   i and j are the current coordinates of the cell that is being visited
    ///   k is the index of the current character of the word being matched
    ///
    fn dfs(
        board: Rc<RefCell<Vec<Vec<char>>>>,
        word: Rc<Vec<char>>,
        k: usize,
        i: usize,
        j: usize,
    ) -> bool {
        if i >= board.borrow().len()
            || j >= board.borrow()[0].len()
            || board.borrow()[i][j] != word[k]
        {
            return false;
        }

        if k == word.len() - 1 {
            return true;
        }
        let current = board.borrow()[i][j];

        // mark the element as visited, because the rules says the same letter cell may not
        // be used more than once
        //
        // change the input parameter to mark subsequent invalid operations is ok in the backtrack program
        board.borrow_mut()[i][j] = '/';

        // the dfs checks if the next character of the word exists in the 4 adjacent cells, and it will
        //  mark the cell as visited and move to the next index of the word by incrementing k by 1
        //
        // if the next character is found, the function returns true, if not it backtracks to the previous
        // cell.
        //
        // if the entire word is found, the function returns true, otherwise it return false
        //
        // when use the saturating_sub, we can skip the index less than zero check
        let res = Self::dfs(board.clone(), word.clone(), k + 1, i + 1, j)
            || Self::dfs(board.clone(), word.clone(), k + 1, i, j + 1)
            || Self::dfs(board.clone(), word.clone(), k + 1, i.saturating_sub(1), j)
            || Self::dfs(board.clone(), word.clone(), k + 1, i, j.saturating_sub(1));

        board.borrow_mut()[i][j] = current;
        res
    }
}

#[test]
fn test1() {
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
