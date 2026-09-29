# Challenge solutions

Try each challenge before reading its answer.

[Course home](../README.md)

## Appendix A — Challenge solutions and review answers

These are changes to the relevant lab, not complete programs unless stated otherwise.

### Lab 00

Add `println!("Batch Order CLI");` inside `main`. Cargo gives a consistent build, execution, and testing workflow even without third-party crates.

### Lab 01

Inside `main`:

```rust
let batch_count: u32 = 4;
let planned = batch_volume_m3 * f64::from(batch_count);
println!("Planned: {planned:.1} m3");
```

Rust supports mutation but requires the binding to explicitly permit it. This makes unintended changes easier to notice.

### Lab 02

Add outside `main`:

```rust
fn is_valid_volume(volume: f64) -> bool {
    volume.is_finite() && volume > 0.0
}
```

Inside `main`, print the results for `2.5`, `0.0`, and `f64::NAN`: expect `true`, `false`, and `false`. An expression produces a value; adding a semicolon discards that value in this function-tail example.

### Lab 03

Add outside `main`:

```rust
fn name_length(name: &str) -> usize {
    name.len()
}
```

Inside `main`, call `name_length(&order)` and then print `order` again to prove you still own it. `.len()` measures UTF-8 bytes. `.chars().count()` counts Unicode scalar values, which still need not equal user-perceived characters. Clone when you actually need an independent owned copy; use a borrow when the function only needs to inspect data.

### Lab 04

Add the enum variant `Cancelled` and the match arm `Status::Cancelled => "cancelled",`. After the existing prints, set `order.status = Status::Cancelled;` and print `order.status_label()`. Enums restrict values to declared alternatives and help the compiler find missing cases.

### Lab 05

Add outside `main`:

```rust
fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}
```

Call with `&volumes` and `&[]`. Expected results are `Some(2.75)` and `None`. This assumes reasonable, finite sample values; Lab 09 explicitly validates numeric inputs and totals. The `as f64` cast is practical for these small counts, although very large integers are not all exactly representable as `f64`. Missing data should not silently become a measured zero.

### Lab 06

Immediately before `Ok(volume)`, add:

```rust
if volume > 8.0 {
    return Err(String::from("Volume exceeds capacity"));
}
```

Check `8.0` succeeds and `8.1` fails. `unwrap()` panics on a parsing error; `Result` lets the application report and handle expected bad input.

### Lab 07

Inside `main`, add `print_value("Order", 1001_u32);`. It works because `u32` implements `Display`. `Vec<i32>` does not implement `Display`; it can be formatted with `Debug` using `{:?}` in a suitably bounded function instead. A module organizes code; an external dependency is another package your project uses, normally declared in `Cargo.toml`.

### Lab 08

Inside the existing `tests` module:

```rust
#[test]
fn rejects_empty_and_whitespace_input() {
    assert!(parse_volume("").is_err());
    assert!(parse_volume("   ").is_err());
}
```

Expect four tests after adding it. The compiler validates language rules and types; tests check the business behavior you intended.

### Lab 09

Add `pub average_m3: f64` to `Summary`. Replace the final construction in `summarize` with:

```rust
Ok(Summary {
    count: orders.len(),
    total_m3,
    average_m3: total_m3 / orders.len() as f64,
})
```

The earlier empty check prevents division by zero. Add to `run`, after the total print:

```rust
println!("Average volume: {:.2} m3", summary.average_m3);
```

Add to `summarizes_valid_orders`:

```rust
assert!((summary.average_m3 - 8.0 / 3.0).abs() < 1e-9);
```

Update the integration test's expected output to:

```text
Orders: 3
Total volume: 8.00 m3
Average volume: 2.67 m3
```

Keep the trailing newline in its Rust string literal. Rerun the complete quality gate.

## Appendix B — Troubleshooting

| Symptom | Likely cause | Action |
|---|---|---|
| `cargo` is not recognized | Terminal did not pick up PATH changes, or installation is incomplete | Reopen the terminal and check the rustup installation instructions |
| Linker or `link.exe` missing | Native build tools are missing | Install the platform prerequisites from Lab 00 |
| Could not find `Cargo.toml` | Wrong working directory | Move into `rust_lab` or `batch_cli` before running Cargo |
| Edition 2024 unsupported | Older toolchain | Check `rustc --version`; use stable 1.85 or later |
| Borrow of moved value | Ownership was transferred | Borrow if you only need to inspect the value; clone only when another owned copy is required |
| Cannot borrow as mutable | Binding is immutable or another live borrow conflicts | Check `mut` and the last use of competing references |
| Expected `f64`, found `()` | A final expression became a statement | Remove the accidental semicolon or use an explicit `return` |
| Module not found | Filename or declaration mismatch | Check `src/volume.rs` and `pub mod volume;` |
| First number rejected despite looking correct | BOM, header, comma, or unit in input | Save UTF-8 without BOM and use bare dot-decimal numbers |
| File cannot be read | Wrong path, access denied, or invalid UTF-8 | Verify the working directory, permissions, and encoding |
| Fixture integration test fails after edits | `data/orders.txt` no longer contains the baseline fixture | Restore `2.5`, `4.0`, and `1.5`; use another file for failure experiments |
| Formatting check fails | Code is not rustfmt-formatted | Run `cargo fmt`, then repeat the check |

Read compiler diagnostics from the first error downward. Fix one cause and check again; later messages may be consequences of the first problem.

## Appendix C — Rust through a C# developer's lens

These are learning comparisons, not exact equivalents.

| Familiar idea | Rust concept | Difference to remember |
|---|---|---|
| `dotnet build` / `dotnet run` | `cargo build` / `cargo run` | Cargo also manages Rust packages and targets |
| Project file | `Cargo.toml` | Dependencies and package metadata live here |
| `List<T>` | `Vec<T>` | Ownership and borrowing affect access and iteration |
| Nullable/missing value | `Option<T>` | Absence is an explicit enum variant |
| Recoverable exception path | `Result<T, E>` | Failure is part of the return type |
| Interface constraint | Trait bound | Traits have different implementation and dispatch rules |
| LINQ-style processing | Iterators and closures | Borrowed versus owned iteration matters |
| Managed object lifetime | Ownership and `Drop` | Rust ordinarily releases owned resources without a tracing garbage collector |

## Completion checklist

- [ ] I can create a Cargo project from an empty folder.
- [ ] I understand why `let`, `mut`, and shadowing are different.
- [ ] I can fix a move error by choosing the right ownership boundary.
- [ ] I can explain `String`, `&str`, `&T`, and `&mut T`.
- [ ] I can model alternatives using an enum and exhaustive matching.
- [ ] I can distinguish `Option` from `Result`.
- [ ] I can move logic into a library and call it from a binary.
- [ ] I can demonstrate a test that fails after deliberately breaking a rule.
- [ ] I can run the CLI successfully and demonstrate a nonzero failure status.
- [ ] I completed the average-volume challenge without copying the solution first.

## References and further study

The exercises and scenario are original. Official references support the language concepts and commands; they are not claims that these particular exercises were executed.

1. [Rust installation](https://www.rust-lang.org/tools/install) — current installers and operating-system prerequisites.
2. [The Rust Book: Installation](https://doc.rust-lang.org/book/ch01-01-installation.html) — toolchain setup.
3. [The Rust Book: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) — package creation and execution.
4. [The Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — moves, borrowing, and slices.
5. [The Rust Book: Recoverable Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) — `Result` and propagation.
6. [The Rust Book: Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) — test functions and assertions.
7. [The Cargo Book: Tests](https://doc.rust-lang.org/cargo/guide/tests.html) — running tests with Cargo.
8. [The Rust Programming Language](https://doc.rust-lang.org/book/) — continue with traits, lifetimes, smart pointers, and concurrency after this course.

For independent follow-up practice, extend the CLI with a maximum allowed volume, a filter threshold, and a structured error enum. Add one feature at a time, defining its acceptance criteria and tests first.
