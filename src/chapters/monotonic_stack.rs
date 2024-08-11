//! a monotonic stack is a special kind of stack, a common data structure in computer science, which
//! maintains its elements in a specific order.
//! unlike a traditional stack, where elements are placed on top of one another based on when they
//! arrive, a monotonic stack ensures that the elements inside the stack remain in an increasing or
//! decreasing order.
//! this is achieved by enforcing specific push and pop rules, depending on whether we want an
//! increasing or decreasing monotonic stack.
//!
//! monotonic stacks are particularly effective when it comes to problems requiring us to find the
//! next smaller or larger element in a sequence, often referred to as Next Greater Element(NGE) or
//! Next Smaller Element(NSE) problems
//!
//! monotonic stacks can be broadly classified into two types:
//! 1. monotonically increasing stack
//!    - a monotonically increasing stack is a stack where elements are arranged in an ascending
//!    order from the bottom to the top
//!    - here, every new element that's pushed onto the stack is greater than or equal to the
//!    element below it
//!    - if a new element is smaller, we pop the elements from the top of the stack until we find an
//!    element smaller than or equal to the new element, or the stack is empty
//!    - this way, the stack always maintains an increasing order
//!  2. monotonically decreasing stack
//!    - when a new element is arrives, if it's larger than the element on the top, we keep popping
//!    the elements from the stack until we find an element that's larger than or equal to the new
//!    element, or the stack is empty.
//!    - this ensures that the stack always maintains a decreasing order.
//!
//! problem characteristics
//! - one classic sign that a monotonic stack might be helpful is when the problem description
//!   mentions finding the "next greater element" or the "next smaller element" in an array.
//! - problem that involve finding maximum areas, such as in histograms, can also be solved
//!   effectively using monotonic stacks.
//! - the key is to identify patterns where a sequential step-by-step comparison is necessary.
//!
mod daily_temperatures;
mod next_greater_element;
mod remove_all_adjacent_duplicates_in_string;
mod remove_all_adjacent_duplicates_in_string_ii;
mod remove_k_digits;
mod remove_nodes_from_linked_list;
