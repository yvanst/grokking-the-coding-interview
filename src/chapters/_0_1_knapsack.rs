//! dynamic programming(DP) is an algorithm technique for solving an optimization problem by
//! breaking it down into simpler subproblems and utilizing the fact that the optimal solution to
//! the overall problem depends upon the optimal solution to its subproblems
//!
//! characteristics of dynamic programming:
//! - overlapping subproblems: subproblems are smaller versions of the original problem. any problem
//! has overlapping sub-problem if finding its solution involves solving the same subproblem
//! multiple times
//! - optimal substructure property: any problem has optimal substructure property if its overall
//! optimal solution can be constructed from the optimal solution of its subproblems
//!
//! top-down with memoization:
//! in this approach, we try to solve the bigger problem by recursively finding the solution to
//! smaller sub-problems. whenever we solve a sub-problem, we cache its result so that we don't end
//! up solving it repeatedly if it's called multiple times.
//! instead, we can just return the saved result. this technique of storing the result of already
//! solved subproblems is called memoization
//!
//! bottom-up with tabulation:
//! tabulation is the opposite of the top-down approach and avoids recursion. in this approach, we
//! solve the problem "bottom-up"(i.e., by solving all the related sub-problems first). this is
//! typically done by filling up an n-dimensional table. based on the results in the table, the
//! solution to the top/original problem is then computed
//!
//! in this course, we will always start with a brute-force recursive solution, which is the best
//! way to start solving any DP problem! once we have a recursive solution then we will apply
//! memoization and tabulation techniques
mod __0_1_knapsack;
mod count_of_subset_sum;
mod equal_subset_sum_partition;
mod minimum_subset_sum_difference;
mod subset_sum;
mod target_sum;
