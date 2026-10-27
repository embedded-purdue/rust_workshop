# 02: Memory solution

Run `cargo run --bin memory-solution`. 

Key checks: both copied integers print as 3, the moved String prints through its new owner, a shared borrow leaves that owner usable, the counter reaches 2, and the integral then prints 0.04.

The deliberate compiler error is `println!("{name}")` after `let owner = name;`. Restore compilation by removing the invalid use or borrowing the original instead of moving it. 
