//! hashing is a great tool to quickly access, protect, and verify data. here are a few of the
//! common use cases of hashing:
//!
//! 1. quick data retrieval
//! 2. data integrity checks
//! 3. passoword security
//! 4. hash tables
//! 5. cryptograph
//! 6. data deduplication
//! 7. load balancing
//!
//! for hashtables, based on how we resolve the collisions, the collision resolution techniques are
//! classified into two types:
//!   - open addressing / closed hashing
//!   - chaining / open hashing
//!
//!
//!
//! - open addressing(closed hashing):
//!   - open addressing techniques resolve hash collisions by probing for the next available slot
//!     within the hash table itself.
//!   - this approach is called open addressing since it opens up the entire hash table for
//!     finding an empty slot during insertion
//!   - it is also known as closed hashing because the insertion is restricted to existing slots
//!     within the table without using any external data structures
//!   - depending on how you jump or probe to find the next empty slot, the closed hashing is
//!   further divided into multiple types. here are the main collision resolution shcemes used in
//!   the open-addressing scheme:
//!     - linear probing
//!     - quadratic probing
//!     - double hashing
//!     - random probing
//!
//! - separate chaining(open hashing)
//!   - separate chaining offers a rather simpler chaining mechanism to resovle collisions. each
//!   slot in the hash table points to a separate data structure, such as a linked-list.
//!   - this linked-list or chain stores multiple elements that share the same hash index
//!   - when a collision occurs, new elements are simply appended to the existing list in the
//!   corresponding slot
//!   - separate chaining is an open hashing technique because the hash table is open to
//!   accommodate multiple elements within a single bucket to handle collisions

mod first_non_repeating_character;
mod largest_unique_number;
mod longest_palindrome;
mod maximum_number_of_balloons;
mod ransom_note;

struct Note;
