// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue:
// Slow Performance -> [Stopping/Resuming the program]

// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner.]

// Ownership rules
// 1. Each value in Rust has a variable that is its owner.
// 2. There can be one owner at a time.
// 3. When the owner goes out of the scope, the value will be dropped.

fn main() {

}