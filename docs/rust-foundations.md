# Rust foundations before your first lab

## What Rust is

Rust is a compiled, statically typed language. The compiler checks types and ownership rules before producing a native executable. Rust is useful when predictable resource management, memory safety, and control over performance matter. It can also be used for everyday command-line utilities; you do not need an embedded device to learn it.

Safe Rust prevents many memory errors through compile-time checks. That does not mean every Rust program is correct: incorrect business rules, panics, resource exhaustion, and deadlocks are still possible. This course uses safe Rust only.

## The build cycle

You edit `.rs` source files, check and compile them with Cargo, and execute the resulting program. An error reported by the compiler occurs before the application runs. A file-reading failure is a runtime condition and belongs in the application's error handling.

| Command | Purpose |
|---|---|
| `cargo new demo` | Create a binary package |
| `cargo check` | Check code quickly without generating a final executable |
| `cargo run` | Build and execute the default binary |
| `cargo test` | Run automated tests |
| `cargo fmt` | Format source code |
| `cargo clippy` | Run additional lint checks |
| `cargo build --release` | Build an optimized executable |
| `cargo doc --open` | Generate and open API documentation |

## The vocabulary you need

| Term | Meaning | Where you will see it |
|---|---|---|
| Binding | A name associated with a value | `let count = 3;` |
| Type inference | Compiler determines a type from usage | `let active = true;` |
| Primitive type | Basic type such as `bool`, `u32`, `f64`, or `char` | Lab 01 |
| Tuple / array | Fixed grouping / fixed-length same-type sequence | Arrays in Lab 02 |
| Struct | Named fields forming a domain value | Lab 04 |
| Enum | A value selected from declared variants | Status and error handling |
| Match | Exhaustive selection by value or pattern | Labs 04–06 |
| Ownership | The rules controlling who holds a value and when it is dropped | Lab 03 |
| Borrow | Temporary access through a reference | `&order` |
| Lifetime | The region where a reference remains valid | Usually inferred in these labs |
| Slice | Borrowed view of contiguous elements | `&[f64]`, `&str` |
| Trait | Shared behavior types can implement | `Display` in Lab 07 |
| Generic | Code parameterized by types | `Vec<T>`, `Result<T, E>` |
| Crate | A library or binary compilation unit | `src/lib.rs`, `src/main.rs` |
| Package | Manifest and one or more crate targets | `Cargo.toml` |
| Workspace | Related packages managed together | The solution collection |
| Macro | Syntax expanded into code | `println!`, `vec!`, `assert!` |

## Memory without a garbage-collector mental model

Local scalar values can be copied cheaply. An owned `String` manages a growable text allocation. Moving a string transfers ownership; borrowing it lets another part of the program inspect it without taking ownership. When its owner goes out of scope, the allocation is released. A reference cannot safely outlive the data it points to.

Start by asking three questions at every function boundary:

1. Does the function need to **own** the value? Pass `T`.
2. Does it need only to **read** it? Consider `&T` or a slice such as `&str`.
3. Does it need to **change** it? Consider `&mut T`.

The exact choice depends on the API. Do not respond to every compiler error by cloning data. In Lab 03 you will deliberately trigger errors to learn what they mean.

## Absence and failure are separate ideas

`Option<T>` answers “is there a value?” with `Some(value)` or `None`. `Result<T, E>` answers “did the operation succeed?” with `Ok(value)` or `Err(error)`. You must choose how to handle those alternatives. A missing list item is often an `Option`; malformed user input is often a `Result`.

## Where Rust fits in real projects

| Scenario | Why consider Rust? | Learning example |
|---|---|---|
| Batch order import utility | Typed validation and a portable native CLI | Labs 01–09 |
| Sensor sample validation | Explicit invalid states and predictable processing | Bonus Lab 10 |
| Service log summary tool | Simple text processing and reusable parsing functions | Bonus Lab 11 |
| Existing .NET business platform | Keep existing workflows; evaluate Rust for a bounded utility if justified | Architecture discussion, not an FFI exercise |

The examples are realistic **scenarios**, not claims of production adoption. A CLI is a good boundary for early experimentation: define an input format, output, and exit status, then measure whether it meets your needs before expanding its role.

## Before moving on

Explain the difference between a compiler error and a runtime error. Then locate `Cargo.toml`, `src/main.rs`, and `src/lib.rs` in a solution package. You do not need to understand all the source yet.

Further reading: [The Rust Book](https://doc.rust-lang.org/book/), [Cargo](https://doc.rust-lang.org/cargo/), and [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).

[Back to course](../README.md) · [Installation guide](setup-windows-macos.md)
