//! this pattern is based on the Depth First Search technique to traverse a tree.
//! we will be using recursion(or we can also use a stack for the iterative approach) to keep track
//! of all the previous(parent) nodes while traversing.
//! this also means that the space complexity of the algorithm will be O(H), where H is the maximum
//! height of the tree

mod all_paths_for_a_sum;
mod binary_tree_path_sum;
mod count_paths_for_a_sum;
mod path_with_given_sequence;
mod path_with_maximum_sum;
mod sum_of_path_numbers;
mod tree_diameter;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct Note;
