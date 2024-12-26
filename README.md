# Rust Programming: Detailed Overview

## Table of Contents

1. [Setup & Installation](#setup-installation)
2. [Variables](#variables)
3. [Data Types](#data-types)
4. [Functions and Methods](#functions-and-methods)
5. [Control Flow](#control-flow)
6. [Ownership and References](#ownership-and-references)
7. [Slices](#slices)
8. [Structs](#structs)
9. [Enums](#enums)
10. [Generics](#generics)
11. [Option and Result Enums](#option-and-result-enums)
12. [Vectors](#vectors)
13. [Project Structure](#project-structure)
14. [Strings](#strings)
15. [Hash Maps](#hash-maps)
16. [Error Handling](#error-handling)
17. [Traits](#traits)
18. [Lifetimes](#lifetimes)

---

## 1. Setup & Installation

Rust is a systems programming language designed for performance and safety, especially safe concurrency. Before writing your first Rust program, you need to set up your development environment.

- **Installing Rust:**
  - The recommended way to install Rust is through Rustup, a toolchain installer for Rust.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
  - Follow the on-screen instructions to complete the installation.

- **Verifying Installation:**
  - After installation, verify Rust by running:
    ```bash
    rustc --version
    ```
  - This command outputs the installed version of Rust.

- **Installing a Code Editor:**
  - Use **Visual Studio Code** (recommended) and install the Rust extension for features like syntax highlighting, error detection, and debugging support.

- **Setting Up Cargo:**
  - Cargo is Rust's build system and package manager. It simplifies project management, dependency handling, and builds.
  - Common commands include:
    ```bash
    cargo new project_name # Create a new Rust project
    cargo build            # Compile the project
    cargo run              # Compile and run the project
    cargo test             # Run tests
    ```

---

## 2. Variables

In Rust, variables play a crucial role in managing data. Rust emphasizes immutability and type safety.

- **Immutability:**
  - By default, variables are immutable, meaning their values cannot be changed after being assigned.
    ```rust
    let x = 5; // Immutable variable
    println!("Value of x: {}", x);
    ```
  - To make a variable mutable, use the `mut` keyword.
    ```rust
    let mut y = 10; // Mutable variable
    y += 5;
    println!("Updated value of y: {}", y);
    ```

- **Shadowing:**
  - Shadowing allows re-declaring variables with the same name. This is useful for transforming values without creating new variables.
    ```rust
    let x = 5;
    let x = x + 1; // Shadowing x
    println!("Shadowed value of x: {}", x);
    ```

---

## 3. Data Types

Rust is a statically typed language, meaning all variables must have a known type at compile time. Rust provides several primitive and compound data types.

- **Scalar Types:**
  - Represent a single value.
    - **Integers:** Whole numbers like `i32` (signed) and `u32` (unsigned).
      ```rust
      let num: i32 = 42;
      ```
    - **Floating-point Numbers:** Numbers with decimal points like `f32` and `f64`.
      ```rust
      let pi: f64 = 3.14159;
      ```
    - **Boolean:** Logical values (`true` or `false`).
      ```rust
      let is_valid: bool = true;
      ```
    - **Character:** Single Unicode characters (`char`).
      ```rust
      let letter: char = 'A';
      ```

- **Compound Types:**
  - Group multiple values together.
    - **Tuples:** Fixed-size collections of values of different types.
      ```rust
      let tup: (i32, f64, char) = (1, 3.5, 'a');
      let (x, y, z) = tup; // Destructuring
      ```
    - **Arrays:** Fixed-length lists of values of the same type.
      ```rust
      let arr: [i32; 3] = [1, 2, 3];
      ```

---

## 4. Functions and Methods

Functions are building blocks of Rust programs, used to encapsulate code for reuse. Methods are similar but associated with a particular type.

- **Functions:**
  - Defined using the `fn` keyword.
    ```rust
    fn main() {
        let result = add(5, 3);
        println!("Result: {}", result);
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

- **Methods:**
  - Associated with structs or enums and defined within an `impl` block.
    ```rust
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    ```

---

## 5. Control Flow

Control flow in Rust allows you to dictate the execution order of statements based on conditions or loops.

- **Conditional Statements:**
  - Use `if`, `else if`, and `else` for branching logic.
    ```rust
    let number = 5;
    if number > 0 {
        println!("Positive");
    } else {
        println!("Negative");
    }
    ```

- **Loops:**
  - **Infinite loop:**
    ```rust
    loop {
        println!("This will print forever!");
    }
    ```
  - **Conditional loop:**
    ```rust
    while condition {
        // Code here
    }
    ```
  - **Iterative loop:**
    ```rust
    for element in collection {
        println!("Element: {}", element);
    }
    ```

---

## 6. Ownership and References

Rust’s unique memory management model relies on ownership, ensuring safety without a garbage collector. Ownership rules:

1. Each value in Rust has a variable that is its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

- **Borrowing:**
  - Use references to access data without transferring ownership.
    ```rust
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```

... *(Sections like Slices, Structs, Enums, etc., would continue in similar detail, expanding on the theoretical background and providing more real-world examples where applicable)* ...
