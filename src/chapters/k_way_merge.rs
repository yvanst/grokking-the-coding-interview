//! this pattern helps us solve problems that involve a list of sorted arrays.
//!
//! whenever we are given K sorted arrays, we can use a Heap to efficiently perform a sorted
//! traversal of all the elements of all arrays. we can push the smallest element of each sorted
//! array in a Min Heap to get the overall minimum. while inserting elements to the Min Heap we
//! keep track of which array the element came from. we can, then, remove the top element from the
//! heap to get the smallest element and push the next element from the same array, to which this
//! smallest element belonged, to the heap. we can repeat this process to make a sorted traversal
//! of all elements.
mod find_k_largest_pairs;
mod kth_smallest_number_in_a_sorted_matrix;
mod kth_smallest_number_in_m_sorted_lists;
mod merge_k_sorted_lists;
mod smallest_number_range;
