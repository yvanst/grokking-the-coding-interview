//! the fast & slow pointer approach, also known as the Hare & Tortoise algorithm, is a pointer
//! algorithm that uses two pointers which move through the array(or sequence/LinkedList) at a
//! different speeds. this approach is quite useful when dealing with cyclic LinkedLists or arrays.
//!
//! by moving at different speeds, the algorithm proves that the two pointers are bound to meet.
//! the fast pointer should catch the slow pointer once both the pointers are in a cyclic loop
//!
//! one of the famous problems solved using this technique was finding a cycle in a LinkedList.
mod linkedlist_cycle;
