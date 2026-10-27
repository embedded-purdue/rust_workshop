# 01: Hello World

**Time: 2 minute.** 
## Goal

Make Rust print a message.

## Background

`fn main()` is our program entry point. `println!` prints a line and is a macro, so its name ends with `!`. Rust statements usually end with `;`. 
`let` creates a binding and Rust can infer its type.

## TASK

1. Open [main.rs](main.rs).
2. Inside `main`, write a statement that prints `Jared Goff is the Goat` using `println!("x");`.
3. Run the program from the repository root:

```sh
cargo run --bin hello
```

4. Add `let author = "your name";` and a second print using variables (hint: similar to python).
5. Change the name and run again. = Cargo rebuilds the changed program.

## Checkpoint

The terminal displays Jared Goff is the goat and your author line. Explain which part prints and which part stores a value. Cargo's build messages may appear above your output.

