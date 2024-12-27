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

Rust is a modern, fast, and memory-safe programming language. To get started:

- **Installing Rust:** Rust uses a tool called Rustup to manage installations.
  - Command to install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - This command downloads and installs Rustup, followed by Rust itself.
  - Follow the on-screen instructions to add Rust to your system.
- **Verifying Installation:**
  - After installation, run `rustc --version`. This should display the installed Rust version, confirming a successful setup.
- **Installing a Code Editor:**
  - The recommended editor is Visual Studio Code (VS Code).
  - Enhance the experience by installing the "Rust-analyzer" extension for features like auto-completion, syntax highlighting, and error detection.
- **Setting Up Cargo:**
  - Cargo is Rust's package manager and build system.
  - Create a new project using `cargo new project_name`.
  - Commands to build and run a project:
    - `cargo build`: Compiles the project.
    - `cargo run`: Builds and runs the project in one step.

---

## 2. Variables

Variables in Rust are fundamental building blocks for storing values during program execution. They have unique properties:

- **Immutability:**
  - By default, variables in Rust are immutable, ensuring safety and predictability in your code.
  ```rust
  let x = 5; // Immutable variable
  ```
- **Mutability:**
  - Use the `mut` keyword to make a variable mutable, allowing reassignment.
  ```rust
  let mut y = 10;
  y += 5; // Now y is 15
  ```
- **Shadowing:**
  - Allows re-declaring a variable with the same name, potentially changing its type.
  ```rust
  let x = 5;
  let x = x + 1; // x is now 6
  ```

---

## 3. Data Types

Rust is a statically-typed language, meaning all variables must have a type. These are the primary data types:

- **Scalar Types:**
  - **Integers:** Represent whole numbers. Examples include `i8`, `i16`, `i32`, `u8`, etc.
  - **Floating-point numbers:** For decimal values, such as `f32` and `f64`.
  - **Boolean:** `bool` can hold either `true` or `false`.
  - **Character:** The `char` type stores a single character, represented by single quotes (`'a'`).
- **Compound Types:**
  - **Tuples:** Group multiple values of various types together.
    ```rust
    let tup: (i32, f64, char) = (1, 3.5, 'a');
    ```
  - **Arrays:** Fixed-size collections of elements of the same type.
    ```rust
    let arr: [i32; 3] = [1, 2, 3];
    ```

---

## 4. Functions and Methods

Functions and methods in Rust define reusable code blocks:

- **Functions:**
  - Functions are defined using the `fn` keyword.
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
  - Methods are functions associated with structs or enums.
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

Control flow in Rust determines the execution path based on conditions and iterations.

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
  - **Infinite loop:** Use `loop` for continuous execution until explicitly stopped.
    ```rust
    loop {
        println!("Running...");
        break;
    }
    ```
  - **Conditional loop:** Use `while` to repeat based on a condition.
    ```rust
    while number > 0 {
        number -= 1;
    }
    ```
  - **Iterative loop:** Use `for` to iterate through collections.
    ```rust
    for element in collection {
        println!("{}", element);
    }
    ```

---

## 6. Ownership and References

Ownership is one of Rust's most unique features, ensuring memory safety without garbage collection.

- **Ownership Rules:**
  - Each value has a single owner.
  - When the owner goes out of scope, the value is dropped.
  ```rust
  let s1 = String::from("hello");
  let s2 = s1; // Ownership transferred
  ```
- **Borrowing:**
  - Allows functions to temporarily access variables without taking ownership.
  ```rust
  fn calculate_length(s: &String) -> usize {
      s.len()
  }
  ```

---

## 7. Slices

### Overview
Slices in Rust provide a way to access a contiguous sequence of elements in a collection without taking ownership of the collection. They are particularly useful when you need to reference parts of a collection temporarily, such as substrings or subarrays.

### Creating a Slice
To create a slice, use a range to specify the start and end indices of the desired segment. The syntax follows the pattern `&collection[start..end]`. The slice includes elements from the `start` index up to, but not including, the `end` index.

```rust
let s = String::from("hello world");
let slice = &s[0..5]; // "hello"
```

### Key Points
1. **Inclusive Start, Exclusive End**: The range `start..end` includes the element at `start` but excludes the element at `end`. For example, `0..5` includes indices 0, 1, 2, 3, and 4.

2. **Type of Slices**:
   - String slices (`&str`): Used to borrow a part of a `String` or string literal.
   - Array slices (`&[T]`): Used to borrow a part of an array or vector.

3. **Borrowing**: A slice is a reference, so it does not take ownership of the collection. The collection must remain valid while the slice is in use.

4. **Syntax Shortcuts**: You can omit the start or end of the range:
   - `&s[..5]` is equivalent to `&s[0..5]`.
   - `&s[6..]` takes from index 6 to the end.
   - `&s[..]` creates a slice of the entire collection.

### Practical Examples
#### Slicing Strings
```rust
let s = String::from("hello world");
let hello = &s[..5];
let world = &s[6..];

println!("{} {}", hello, world); // Output: "hello world"
```

#### Slicing Arrays
```rust
let numbers = [1, 2, 3, 4, 5];
let slice = &numbers[1..4]; // [2, 3, 4]

println!("{:?}", slice); // Output: [2, 3, 4]
```

### Memory Safety
Rust ensures memory safety with slices by:
1. Validating bounds: Accessing out-of-range indices with a slice will cause a runtime panic.
2. Enforcing borrowing rules: The collection cannot be modified while it has active slices.

```rust
let s = String::from("hello");
let slice = &s[..];
// s.push('!'); // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
```

### Use Cases
- Parsing strings: Extract substrings without copying the data.
- Processing arrays: Work with subarrays for algorithms or functions.

### Summary
Slices are a powerful and efficient way to work with parts of collections in Rust. They allow you to borrow data without duplication, ensuring both performance and safety.


---

## 8. Structs

Structs are used to create complex data types that group related variables.

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

Enums represent a value from a predefined set of options:

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

Generics enable type flexibility:

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

Handle optional and error-prone operations effectively:

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

Vectors are dynamic arrays that grow as needed:

```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

---

## 13. Project Structure

Organize code with modules and crates:

```rust
mod module_name {
    pub fn function_name() {}
}
use module_name::function_name;
```

---

## 14. Strings

Strings in Rust are UTF-8 encoded and can be mutable:

```rust
let mut s = String::from("hello");
s.push_str(" world");
```

---

## 15. Hash Maps

Store key-value pairs with hash maps:

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

---

## 16. Error Handling

Handle recoverable errors using `Result`:

```rust
let file = File::open("hello.txt");
match file {
    Ok(f) => println!("File opened successfully"),
    Err(e) => println!("Failed to open file: {}", e),
}
```

---

## 17. Traits

Traits define shared behavior:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

---

## 18. Lifetimes

Lifetimes prevent dangling references and ensure memory safety:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
