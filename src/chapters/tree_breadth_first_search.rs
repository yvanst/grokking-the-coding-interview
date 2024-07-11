/// this pattern is based on the Breadth First Search(BFS) technique to traverse a tree.
///
/// any problem involving the travesal of a tree in a level-by-level order can be efficiently
/// solved using this approach.
/// we will use a Queue to keep track of all the nodes of a level before we jump onto the next
/// level. this also means that the space complexity of the algorithm will be O(W), where W is the
/// maximum number of nodes on any level
mod binary_tree_level_order_traversal;
mod level_averages_in_a_binary_tree;
mod reverse_level_order_traversal;
mod zigzag_traversal;
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
