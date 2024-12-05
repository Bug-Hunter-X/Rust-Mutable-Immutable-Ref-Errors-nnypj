# Rust Mutable vs. Immutable References

This example showcases a frequent error in Rust: attempting to modify a value through an immutable reference. Rust's ownership and borrowing system prevents data races by enforcing strict rules about mutability.  The `bug.rs` file contains code that will fail to compile due to this violation, while `bugSolution.rs` demonstrates a corrected approach.

## How to Reproduce

1. Save the code in `bug.rs`.
2. Attempt to compile it using `rustc bug.rs`.
3. Observe the compiler error, indicating that an immutable borrow occurs. 

## Understanding the Error

Rust's borrow checker prevents situations where multiple mutable references point to the same data, thereby preventing data races. The error message explicitly states that the attempted modification is through an immutable reference.