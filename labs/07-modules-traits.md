[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 07 — Modules, libraries, and traits

### Objective

Separate domain logic from the application and use a trait bound in one small generic function.

### Concept

`src/lib.rs` is the entry point for a library crate. `src/main.rs` is the entry point for a binary crate. A module groups related code. Items are private unless made public with `pub`. A trait describes behavior that a type provides; a generic bound restricts a type parameter to types with that behavior.

### Exercise

1. Create `src/volume.rs`. Copy **only the `parse_volume` function from Lab 06**, using the baseline version without the optional capacity challenge. Add `pub` before `fn`:

```text
pub fn parse_volume(text: &str) -> Result<f64, String>
```

Keep its function body unchanged. Do not paste the declaration above as a complete function.

2. Create `src/lib.rs` with:

```rust
pub mod volume;
```

3. Replace `src/main.rs` with:

```rust
use rust_lab::volume::parse_volume;
use std::fmt::Display;

fn print_value<T: Display>(label: &str, value: T) {
    println!("{label}: {value}");
}

fn main() {
    match parse_volume("2.5") {
        Ok(volume) => print_value("Volume", volume),
        Err(error) => eprintln!("Error: {error}"),
    }
    print_value("Status", "ready");
}
```

4. Run `cargo check` and `cargo run --quiet`.

```text
Volume: 2.5
Status: ready
```

`Display` enables `{}` formatting. `f64` and `&str` both implement it. The function accepts either type without using a universal object type. `eprintln!` writes to standard error.

### Checkpoint

Temporarily remove `pub` from `parse_volume` and check the visibility error. Restore it. The library and binary in the same package are still separate crates.

### Challenge

Call `print_value` with an order ID of type `u32`. Then try a `Vec<i32>` and explain the compile error. Remove the failing call afterward.

### Review

What is the difference between a module and an external dependency?

---

[Previous lab](06-errors.md) · [Next lab](08-testing.md)
