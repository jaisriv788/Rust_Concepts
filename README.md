+281


Original file line number	Diff line number	Diff line change
@@ -0,0 +1,281 @@
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
- **Installing Rust:**
  - Use the Rustup tool: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Follow the prompts to add Rust to your system.
- **Verifying Installation:**
  - Run `rustc --version` to ensure Rust is installed.
- **Installing a Code Editor:**
  - Use Visual Studio Code (recommended).
  - Install the Rust extension for syntax highlighting and other tools.
- **Setting Up Cargo:**
  - Cargo is Rust's package manager and build system.
  - Initialize a project: `cargo new project_name`
  - Build and run: `cargo build` and `cargo run`.
---
## 2. Variables
- Variables in Rust are immutable by default.
  ```rust
  let x = 5; // Immutable variable
  let mut y = 10; // Mutable variable
  y += 5;
  ```
- Shadowing allows re-declaring variables with the same name.
  ```rust
  let x = 5;
  let x = x + 1;
  ```
---
## 3. Data Types
- **Scalar Types:**
  - Integers: `i8`, `i16`, `i32`, `u8`, etc.
  - Floating-point: `f32`, `f64`.
  - Boolean: `bool`.
  - Character: `char`.
- **Compound Types:**
  - Tuples: Fixed-size collections.
    ```rust
    let tup: (i32, f64, char) = (1, 3.5, 'a');
    ```
  - Arrays: Fixed-length lists.
    ```rust
    let arr: [i32; 3] = [1, 2, 3];
    ```
---
## 4. Functions and Methods
- **Functions:**
  ```rust
  fn main() {
      let result = add(5, 3);
      println!("Result: {}", result);
  }
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```
- **Methods:** Associated with structs or enums.
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
- **Conditional Statements:**
  ```rust
  let number = 5;
  if number > 0 {
      println!("Positive");
  } else {
      println!("Negative");
  }
  ```
- **Loops:**
  - Infinite loop: `loop {}`.
  - Conditional loop: `while condition {}`.
  - Iterative loop: `for element in collection {}`.
---
## 6. Ownership and References
- Rust's memory safety is ensured by ownership rules:
  - Each value has a single owner.
  - When ownership transfers, the previous owner loses access.
  ```rust
  let s1 = String::from("hello");
  let s2 = s1; // Ownership transferred.
  ```
- Borrowing allows temporary access:
  ```rust
  fn calculate_length(s: &String) -> usize {
      s.len()
  }
  ```
---
## 7. Slices
- Slices provide references to parts of collections.
  ```rust
  let s = String::from("hello world");
  let slice = &s[0..5];
  ```
---
## 8. Structs
- Structs define custom types.
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
  }
  let user1 = User {
      username: String::from("john_doe"),
      email: String::from("john@example.com"),
      sign_in_count: 1,
  };
  ```
---
## 9. Enums
- Enums represent a value from a defined set.
  ```rust
  enum Direction {
      Up,
      Down,
      Left,
      Right,
  }
  let dir = Direction::Up;
  ```
---
## 10. Generics
- Generics allow code to operate on different types.
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list {
          if item > largest {
              largest = item;
          }
      }
      largest
  }
  ```
---
## 11. Option and Result Enums
- Handle optional and error-prone operations.
  ```rust
  let some_number = Some(5);
  let no_number: Option<i32> = None;
  fn divide(a: i32, b: i32) -> Result<i32, String> {
      if b == 0 {
          Err(String::from("Cannot divide by zero"))
      } else {
          Ok(a / b)
      }
  }
  ```
---
## 12. Vectors
- Dynamic arrays.
  ```rust
  let mut v = vec![1, 2, 3];
  v.push(4);
  ```
---
## 13. Project Structure
- Organize code with modules and crates.
  ```rust
  mod module_name {
      pub fn function_name() {}
  }
  use module_name::function_name;
  ```
---
## 14. Strings
- Strings are UTF-8 encoded and mutable.
  ```rust
  let mut s = String::from("hello");
  s.push_str(" world");
  ```
---
## 15. Hash Maps
- Store key-value pairs.
  ```rust
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  ```
---
## 16. Error Handling
- Handle recoverable errors with `Result`.
  ```rust
  let file = File::open("hello.txt");
  match file {
      Ok(f) => println!("File opened successfully"),
      Err(e) => println!("Failed to open file: {}", e),
  }
  ```
---
## 17. Traits
- Traits define shared behavior.
  ```rust
  pub trait Summary {
      fn summarize(&self) -> String;
  }
  ```
---
## 18. Lifetimes
- Lifetimes prevent dangling references.
  ```rust
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }
