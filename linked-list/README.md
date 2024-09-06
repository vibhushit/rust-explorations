# Linked List Implementations in Rust

This folder is dedicated to implementing various types of linked lists in Rust. The goal is to explore basic and advanced Rust programming concepts by building different types of linked lists. This project is a deep dive into Rust, with concepts like pointers, ownership, borrowing, and many other key aspects of the language.

## Overview

Linked lists might seem simple, but they are deceptively complex in Rust due to the language’s strict ownership and borrowing rules.

### Concepts

Hands-on experience with concepts:

- **Pointer Types**: `&`, `&mut`, `Box`, `Rc`, `Arc`, `*const`, `*mut`, and `NonNull`
- **Ownership and Borrowing**: Concepts such as inherited mutability, interior mutability, and `Copy`
- **Rust Keywords**: Including `struct`, `enum`, `fn`, `pub`, `impl`, `use`, and more
- **Pattern Matching and Generics**: Using Rust's powerful type system to handle different cases and data types
- **Testing**: Writing unit tests, using tools like `miri`, and installing new toolchains
- **Unsafe Rust**: Working with raw pointers, understanding aliasing, stacked borrows, `UnsafeCell`, and variance

### Implementations

Here’s what you’ll find in this folder:

1. **A Bad Singly-Linked Stack**
   - A basic implementation to introduce the concept of linked lists.
   
2. **An Ok Singly-Linked Stack**
   - An improved version of the singly-linked stack with better safety and performance.
   
3. **A Persistent Singly-Linked Stack**
   - A functional approach to a singly-linked stack, with persistence features.
   
4. **A Bad But Safe Doubly-Linked Deque**
   - A doubly-linked list implementation, focusing on making it safe despite its inherent complexity.
   
5. **An Unsafe Singly-Linked Queue**
   - An exploration into unsafe Rust with a singly-linked queue that uses raw pointers.
   
<!-- 6. **TODO: An Ok Unsafe Doubly-Linked Deque**
   - A safer, yet still unsafe, implementation of a doubly-linked deque.
   
7. **Bonus: A Bunch of Silly Lists**
   - Various playful and experimental list implementations, pushing the boundaries of Rust. -->

## Getting Started

To get started with any of the linked list implementations, navigate to the corresponding file and dive into the code. Each list has its own set of challenges, so take your time to understand the concepts.

## Contributing

Contributions are welcome! If you have improvements or additional linked list implementations you'd like to share, feel free to fork the repository and create a pull request.

## License

This project is licensed under the MIT License. You’re free to use, modify, and distribute the code, but please give credit where it’s due.
