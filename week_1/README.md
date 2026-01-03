# Week 1: Introduction to Rust for Data Engineering

## 📚 Overview
This week introduces the fundamentals of Rust programming with a focus on data engineering concepts. You'll learn Rust basics, set up your development environment, and create your first Rust project.

## 🎯 Learning Objectives
By the end of this week, you should be able to:
- Set up a Rust development environment
- Understand basic Rust syntax and concepts
- Use Cargo to manage Rust projects
- Write, compile, and run simple Rust programs
- Apply basic data structures in Rust

## 📂 Week Structure
```
week_1/
├── README.md        # This file
├── Makefile         # Build automation commands
└── [Your projects will go here]
```

## 🚀 Getting Started

### Prerequisites
Make sure you have Rust installed:
```bash
rustc --version
cargo --version
```

If not installed, visit: https://rustup.rs/

### Create Your First Project
```bash
# Navigate to week_1 folder
cd week_1

# Create a new Rust project
cargo new hello_rust
cd hello_rust

# Run your project
cargo run
```

## 🛠️ Available Commands

This week's folder includes a Makefile with helpful commands:

```bash
# Check Rust toolchain versions
make rust-version

# Format your code
make format

# Run linter (clippy)
make lint

# Run tests
make test

# Run your project
make run

# Build release version (optimized)
make release

# Run all checks (format, lint, test, run)
make all
```

## 📝 Weekly Tasks

### Task 1: Hello World
- Create a simple "Hello, World!" program
- Modify it to accept command-line input
- Add proper error handling

### Task 2: Data Types & Variables
- Practice with Rust's basic data types
- Understand ownership and borrowing
- Work with strings and vectors

### Task 3: Functions & Control Flow
- Write functions with different return types
- Use if/else statements and loops
- Implement pattern matching with `match`

## 💡 Key Concepts to Learn

### Ownership
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would error!
    println!("{}", s2); // This works
}
```

### Borrowing
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing with &
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## 📖 Resources
- [The Rust Book - Chapters 1-4](https://doc.rust-lang.org/book/)
- [Rust by Example - Basics](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## ✅ Checklist
Before moving to Week 2, make sure you can:
- [ ] Create a new Rust project with Cargo
- [ ] Write and run basic Rust programs
- [ ] Use variables, functions, and control flow
- [ ] Understand ownership and borrowing basics
- [ ] Run tests and format code
- [ ] Use the Makefile commands

## 🤔 Questions?
If you get stuck:
1. Check the Rust documentation
2. Review the error messages (Rust has great error messages!)
3. Ask in class discussions
4. Create an issue in the repository

---

**Next Week:** Week 2 will cover advanced data types, structs, and enums!