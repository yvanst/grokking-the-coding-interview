//! in many problems dealing with an array, we are asked to find or calculate
//! something among all the subarrays of a given size
//!
//! the efficient way to sovle this problem would be to visualize each subarray as a sliding window
//! of n elements. this means that we will slide the window by one element when we move on to the
//! next subarray. to reuse the sum from the previous subarray, we will subtract the element going
//! out of the window and add the element now being included in the sliding window. this will save
//! us from going through the whole subarray to find the sum and, as a result, the algorithm
//! complexity will reduce to O(N).
//!
//! in some problems, the size of the sliding window is not fixed. we have to expand or shrink the
//! window based on the problem constraints.
mod fruits_into_baskets;
mod longest_subarray_with_ones_after_replacement;
mod longest_substring_with_k_distinct_characters;
mod longest_substring_with_same_letters_after_replacement;
mod maximum_sum_subarray_of_size_k;
mod permutation_in_a_string;
mod smallest_subarray_with_a_greater_sum;
mod smallest_window_containing_substring;
mod string_anagrams;
mod words_concatenation;
