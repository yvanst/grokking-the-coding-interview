//! backtrackign is an algorithmic technique that uses brute force approach to solve a problem
//! 
//! brute force approach states that for any problem, we should try out all possible solutions and
//! pick up those solutions that satisfy the problem constraints.
//!
//! in backtracking, we build a solution incrementally and follow the approach that if the current
//! solution can't lead to a valid solution, abandon it and backtrack(or go back) to try another
//! solution.
//!
//! because of this, recursion becomes a suitable technique for solving backtracking problems
//!
//! dynamic programming uses a simialr approach where we try out all possible solutions (using
//! memoization) to pick up the most optimized solution.
//!
//! DP is used to solve optimization problems; backtracking, on the other hand, is mostly used when
//! multiple valid solution are possible for a problem. -> max(res) v.s. Vec<res> 
//!
//! this approach of evaluating all possible solutions, going back whnever we see that a certain
//! constraint can't be met, and trying out other possible solutions is called backtracking.

mod combination_sum;
mod factor_combinations;
mod split_a_string_into_the_max_number_of_unique_substrings;
mod sudoku_solver;
mod word_search;

struct Note;
