/// this pattern is based on the Breadth First Search(BFS) technique to traverse a tree.
///
/// any problem involving the travesal of a tree in a level-by-level order can be efficiently
/// solved using this approach.
/// we will use a Queue to keep track of all the nodes of a level before we jump onto the next
/// level. this also means that the space complexity of the algorithm will be O(W), where W is the
/// maximum number of nodes on any level
mod binary_tree_level_order_traversal;
mod connect_all_level_order_siblings;
mod connect_level_order_siblings;
mod level_averages_in_a_binary_tree;
mod level_order_successor;
mod minimum_depth_of_a_binary_tree;
mod reverse_level_order_traversal;
mod right_view_of_a_binary_tree;
mod zigzag_traversal;
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
