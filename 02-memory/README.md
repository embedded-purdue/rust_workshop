# 02: Memory, ownership, and borrowing

**Time: 10–25 minutes.** [Previous: print and loops](../01-print_loops/README.md) · [Next: flight controller](../03-flight_controller/README.md)

## Goal

Learn why Rust lets one function read a value, another change it, and another take ownership. Use those ideas to store a controller's memory.

## Background

| Example | What happens |
| --- | --- |
| `let b = a;` with `u32` | Copy: both bindings remain usable |
| `let b = a;` with `String` | Move: b takes ownership, a becomes unusable |
| `&value` | Shared reference: borrow for reading |
| `&mut value` | Exclusive mutable reference: borrow for changing |
| `*reference` | Access the referenced value |

An owned `String` contains metadata (pointer, length, capacity) and owns text stored in a heap allocation. Moving the String transfers ownership rather than duplicating its text. Rust drops it automatically when its owner leaves scope. Simple local integers do not need a separate heap allocation. Exact storage placement can change with optimization; learn the ownership rules rather than relying on addresses.

While references are in use, Rust allows shared reading or one exclusive mutable borrow. You do not call `free` in these exercises. A reference cannot outlive its referent.

## Your tasks

Open [main.rs](main.rs) and run `cargo run --bin memory` from the repository root. The starter compiles, but its counter and integral still print zero until you complete the tasks below.

### A. Copy versus move (3 minutes)

The integer example prints both `first` and `copied`. Find the String move, uncomment the marked print using `name`, and run:

```sh
cargo check --bin memory
```

Read the use-of-moved-value error. Comment that print again. `owner` now owns the String. Do not fix it by cloning: observe the ownership transfer first.

### B. Borrow without taking ownership (2 minutes)

Trace `label(&owner)`. Its parameter `&str` borrows text, and `owner` remains usable afterward. Predict the output, then run it.

### C. Change the caller's state (3 minutes)

Implement `count_sample` with `*samples += 1;`. Remove its placeholder `let _ = samples;`. It receives `&mut u32`, so it can change the caller's value.

Checkpoint: two calls print `Samples collected: 2 (goal: 2)`.

### D. Remember an error (3 minutes)

Replace the integral placeholder with `integral += error * dt;`. With error 2 and dt 0.02, the result is 0.04.

This is the same memory operation the PID's I term will perform repeatedly. Each controller update borrows `&mut self` to modify its stored integral.

## Checkpoint

Explain why the borrowed device name is still usable and why the sample counter needs `&mut`. Finish with `cargo check --bin memory` passing.

[Solution code](sol/main.rs) · [Next: flight controller](../03-flight_controller/README.md)
