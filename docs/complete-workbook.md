# Rust: A Self-Paced Hands-On Learning Path

**From your first program to a tested Batch Order CLI**  
Language: English · Level: beginner Rust / existing programming experience  
Format: guided labs, checkpoints, challenges, and solutions  
Prepared: 29 September 2026

This is an original tutorial inspired by the learning flow of Microsoft-style hands-on training: understand a concept, complete an exercise, verify the result, and apply the idea independently. It is not an official Microsoft course.

## Your mission

You are building a small command-line tool for an operations team. It reads batch order quantities from a text file, validates them, and reports the number of orders and total volume. The scenario is deliberately small so that Rust concepts stay visible.

By the end, you will be able to:

- Create and run a Rust package using Cargo.
- Work with variables, functions, collections, structs, and enums.
- Explain ownership, moves, borrowing, and basic lifetimes.
- Handle missing values and recoverable errors explicitly.
- Organize logic into a library and a command-line application.
- Read a file, validate input, write tests, and create a release build.

**Prerequisites:** You can already use variables, conditions, loops, and functions in a language such as C#, TypeScript, or Python. No Rust experience is required. Set aside approximately **7–9 hours**, including independent challenges. Installation may take additional time.

**Scope:** This foundation course uses the Rust standard library only. Web APIs, async programming, unsafe code, FFI, and PLC communication belong in a later course. The batch examples are learning exercises, not production dosing or control logic.

## Learning map

| Lab | Deliverable | Approximate time |
|---|---|---:|
| 00 | Working Rust environment and first program | 30 min |
| 01 | Quantity calculation with variables and types | 30 min |
| 02 | Functions, decisions, and iteration | 35 min |
| 03 | Ownership and borrowing experiments | 50 min |
| 04 | Order model with structs and enums | 40 min |
| 05 | Collections, iterators, and optional values | 40 min |
| 06 | Validated input with `Result` | 45 min |
| 07 | Reusable module and a generic trait-bound function | 40 min |
| 08 | Unit tests and a quality workflow | 40 min |
| 09 | File-driven Batch Order CLI capstone | 75 min |

Suggested schedule: Labs 00–02 on day one; 03–05 on day two; 06–08 on day three; the capstone and independent practice on day four.

## How to use this workbook

1. Read the objective and concept before copying code.
2. Type the code when practical; predict the output before running it.
3. Complete the checkpoint. Do not move forward with an unexplained error.
4. Try the challenge before reading its solution in Appendix A.
5. Explain the review question aloud in your own words.

Labs 01–08 reuse one disposable package, `rust_lab`. Each lab explicitly replaces the relevant files. Keep your own notes or commit each completed lab to Git if you want to preserve earlier versions. Lab 09 uses a separate package, `batch_cli`.

Commands shown as `text` can be entered in PowerShell, Bash, or zsh unless an operating system is named. Do not type Markdown fences. Paths such as `src/main.rs` are relative to the package directory containing `Cargo.toml`. Use your editor to create files and folders; no Unix-only file creation commands are required.

**Validation note (initial authoring):** The examples were reviewed for consistency, but were not compiled or executed in the authoring environment because Rust was unavailable. The outputs below are expected outputs. Use each checkpoint on your own machine; exact compiler diagnostics and Cargo progress messages may vary by toolchain.

---

## Lab 00 — Prepare your environment

### Objective

Install the stable toolchain, create a package, and understand the edit–check–run cycle.

### Concept

`rustc` compiles Rust. Cargo creates packages, builds code, runs tests, and manages dependencies. `rustup` manages toolchains. A **package** contains a `Cargo.toml` manifest; a **crate** is a compilation unit, such as a library or executable.

### Exercise

1. Open the [official Rust installation page](https://www.rust-lang.org/tools/install).
2. Follow the instructions for your operating system:
   - **Windows:** Use the official `rustup-init.exe`. Follow its prompts for the Microsoft C++ build tools and Windows SDK required by the default MSVC toolchain. VS Code alone does not install the linker.
   - **macOS:** Install Xcode Command Line Tools with `xcode-select --install` if needed, then use the rustup instructions on the official page.
   - **Linux:** Use rustup and your distribution's C compiler/linker prerequisites. On Ubuntu/Debian, `sudo apt install build-essential` installs common build tools when permitted on your machine.
3. Close and reopen your terminal, then run:

```text
rustup show
rustc --version
cargo --version
rustup component add rustfmt clippy
```

Use stable Rust **1.85 or later** for the 2024 edition used in this workbook. To update an existing rustup-managed stable installation, run `rustup update stable`. Avoid changing a company's repository-specific toolchain just for this course; work in a separate learning folder.

4. Optionally install VS Code and the **rust-analyzer** extension. Open your learning folder in the editor.
5. In the terminal, create the practice package:

```text
mkdir rust-hands-on
cd rust-hands-on
cargo new rust_lab --edition 2024
cd rust_lab
cargo run
```

Expected application output:

```text
Hello, world!
```

6. Inspect `Cargo.toml` and `src/main.rs`. Replace `src/main.rs` with:

```rust
fn main() {
    println!("Hello, learner! Welcome to Rust.");
}
```

7. Run:

```text
cargo check
cargo run --quiet
```

`check` checks the code without producing the final executable. `run` builds and executes it. `--quiet` hides normal Cargo progress output, not compiler errors.

### Checkpoint

The program prints `Hello, learner! Welcome to Rust.` and exits successfully. You can identify the manifest and entry-point file.

### Challenge

Print a second line containing the name of the tool you will build: `Batch Order CLI`.

### Review

Why is Cargo useful even when a program has no external dependencies?

---

## Lab 01 — Variables, types, and expressions

### Objective

Calculate a batch target and distinguish reassignment from shadowing.

### Concept

Bindings are immutable by default. `mut` permits reassignment. A type annotation uses `name: Type`. `f64` represents a double-precision floating-point number; `u32` is an unsigned 32-bit integer. Rust does not silently convert an integer into a floating-point number for arithmetic.

### Exercise

Replace all of `src/main.rs` with:

```rust
fn main() {
    let order_id: u32 = 1001;
    let batch_volume_m3: f64 = 2.5;
    let cement_kg_per_m3: f64 = 320.0;
    let mut produced_m3: f64 = 0.0;

    let cement_target_kg = batch_volume_m3 * cement_kg_per_m3;
    produced_m3 += batch_volume_m3;

    println!("Order: {order_id}");
    println!("Cement target: {cement_target_kg:.1} kg");
    println!("Produced: {produced_m3:.1} m3");

    let label = "  Batch A  ";
    let label = label.trim();
    println!("Label: '{label}'");
}
```

Run `cargo run --quiet`.

```text
Order: 1001
Cement target: 800.0 kg
Produced: 2.5 m3
Label: 'Batch A'
```

Now remove `mut` from `produced_m3` and run `cargo check`. Read the error location and suggestion. Restore `mut` before continuing.

The second `let label` creates a new binding: this is **shadowing**. By contrast, `produced_m3 += ...` modifies an existing mutable binding. `:.1` controls displayed decimal places; it does not change the stored value.

### Checkpoint

Change the volume to `3.0`. The target becomes `960.0 kg`. Restore `2.5` after checking.

### Challenge

Introduce `let batch_count: u32 = 4;` and print the total planned volume, `10.0 m3`. Use `f64::from(batch_count)` for conversion.

### Review

What is the difference between immutable data bindings and a language that has no way to mutate data?

---

## Lab 02 — Functions and control flow

### Objective

Extract a calculation into a function and classify quantities using a loop.

### Concept

Functions declare parameter types and use `->` for the return type. A final expression without a semicolon becomes the return value. An `if` expression can produce a value, provided its branches have compatible types.

### Exercise

Replace `src/main.rs` with:

```rust
fn cement_target(volume_m3: f64, dosage_kg_per_m3: f64) -> f64 {
    volume_m3 * dosage_kg_per_m3
}

fn classify(volume_m3: f64) -> &'static str {
    if volume_m3 <= 0.0 {
        "invalid"
    } else if volume_m3 <= 3.0 {
        "small"
    } else {
        "large"
    }
}

fn main() {
    let volumes = [2.5, 4.0, 0.0];
    for volume in volumes {
        println!("{volume:.1} m3: {}", classify(volume));
    }
    println!("Target: {:.1} kg", cement_target(2.5, 320.0));
}
```

Run `cargo run --quiet`.

```text
2.5 m3: small
4.0 m3: large
0.0 m3: invalid
Target: 800.0 kg
```

The type `&'static str` is a borrowed string slice valid for the program's lifetime. These return values are string literals embedded in the program. You do not need to allocate a new `String` for each label.

Add a semicolon after `volume_m3 * dosage_kg_per_m3` and run `cargo check`. The function now implicitly returns `()`—the unit type—instead of `f64`. Remove the semicolon to fix it.

### Checkpoint

Explain why `3.0` is classified as small and `3.1` as large. This introductory classifier assumes finite numbers; Lab 06 adds explicit validation.

### Challenge

Add `fn is_valid_volume(volume: f64) -> bool` that accepts only finite values greater than zero. Call it with `2.5`, `0.0`, and `f64::NAN`.

### Review

How does an expression differ from a statement ending with a semicolon?

---

## Lab 03 — Ownership and borrowing

### Objective

Experience a move error, then fix it by borrowing instead of copying data.

### Concept

Each value has an owner. For a type such as `String`, assignment normally transfers ownership. When the owning value is dropped, its resources are released. References let functions access a value without owning it.

Think of `String` as owned UTF-8 text and `&str` as a borrowed view of UTF-8 text. In C#, assigning a reference type usually gives another reference to the same object. Do not transfer that assumption directly to Rust's owned `String`.

### Exercise A — Observe a move

Replace `src/main.rs` with this **intentionally non-compiling** program:

```rust
fn main() {
    let original = String::from("Order-1001");
    let moved = original;
    println!("{moved}");
    println!("{original}"); // Intentional use after move.
}
```

Run `cargo check`. Expect a diagnostic about borrowing a moved value, commonly E0382. `moved` now owns the string; `original` cannot be used.

### Exercise B — Borrow instead

Replace the entire file with:

```rust
fn print_order(name: &str) {
    println!("Order: {name}");
}

fn add_suffix(name: &mut String) {
    name.push_str("-READY");
}

fn main() {
    let mut order = String::from("Order-1001");
    print_order(&order);
    add_suffix(&mut order);
    print_order(&order);
    println!("Still owned by main: {order}");
}
```

Expected output:

```text
Order: Order-1001
Order: Order-1001-READY
Still owned by main: Order-1001-READY
```

`&order` borrows the string. It can be passed to an `&str` parameter through a standard coercion. `&mut order` grants temporary exclusive mutable access. For the same data, you may have multiple shared references or one exclusive mutable reference while those borrows overlap. Rust often ends a borrow at its last use, rather than at the closing brace.

### Exercise C — See overlapping borrows

Inside `main`, after the existing statements, add:

```rust
    let shared = &order;
    order.push_str("!");
    println!("{shared}");
```

Run `cargo check`: the mutation conflicts with a shared borrow that is used afterward. Move the `println!` before `push_str` and check again. The shared borrow no longer overlaps the mutation.

### Checkpoint

You have deliberately produced and repaired both an ownership error and a borrowing error. Keep the file compiling before proceeding.

### Challenge

Write `fn name_length(name: &str) -> usize` and print a name's length without consuming the owned string. For ASCII order identifiers, use `.len()`. Explain why this counts UTF-8 bytes, not necessarily human-readable characters.

### Review

When would `.clone()` be appropriate? When would it hide an unnecessarily ownership-taking function signature?

---

## Lab 04 — Model data with structs and enums

### Objective

Represent an order and a state explicitly rather than using unrelated variables and arbitrary strings.

### Concept

A struct groups named fields. An `impl` block defines associated functions and methods. An enum describes alternatives; `match` requires handling all alternatives. `&self` borrows an instance for reading, while `&mut self` permits changes.

### Exercise

Replace `src/main.rs` with:

```rust
#[derive(Debug)]
enum Status {
    Planned,
    Completed,
}

#[derive(Debug)]
struct Order {
    id: u32,
    volume_m3: f64,
    status: Status,
}

impl Order {
    fn new(id: u32, volume_m3: f64) -> Self {
        Self { id, volume_m3, status: Status::Planned }
    }

    fn complete(&mut self) {
        self.status = Status::Completed;
    }

    fn status_label(&self) -> &'static str {
        match &self.status {
            Status::Planned => "planned",
            Status::Completed => "completed",
        }
    }
}

fn main() {
    let mut order = Order::new(1001, 2.5);
    println!("{}: {:.1} m3, {}", order.id, order.volume_m3, order.status_label());
    order.complete();
    println!("{}: {}", order.id, order.status_label());
}
```

Run `cargo fmt` to format the code, then `cargo run --quiet`.

```text
1001: 2.5 m3, planned
1001: completed
```

`Self` means the type currently being implemented. `Order::new` is an associated function; `order.complete()` is an instance method. `#[derive(Debug)]` asks the compiler to generate debugging-format support. This simple constructor does not validate quantities yet.

### Checkpoint

Remove `mut` from `order`: `complete()` should fail to compile. Restore it.

### Challenge

Add `Cancelled` to `Status`. Compile before changing `status_label`; then add the missing match arm and print the cancelled status.

### Review

Why can an enum make state changes easier to maintain than strings such as `"done"`, `"Done"`, and `"completed"`?

---

## Lab 05 — Collections, iterators, and Option

### Objective

Process a growable collection without consuming it and handle an item that may not exist.

### Concept

`Vec<T>` is a growable sequence. `.iter()` visits borrowed elements; `.into_iter()` on an owned vector consumes it. Iterator adapters such as `filter` and `map` are lazy until a consumer such as `sum` or `collect` runs. `Option<T>` expresses presence (`Some`) or absence (`None`).

### Exercise

Replace `src/main.rs` with:

```rust
fn main() {
    let mut volumes = vec![2.5, 4.0, 1.5];
    volumes.push(3.0);

    let total: f64 = volumes.iter().sum();
    let large: Vec<f64> = volumes
        .iter()
        .copied()
        .filter(|volume| *volume > 3.0)
        .collect();

    println!("Count: {}", volumes.len());
    println!("Total: {total:.1} m3");
    println!("Large: {large:?}");

    match volumes.get(10) {
        Some(volume) => println!("Found: {volume}"),
        None => println!("No order at index 10"),
    }
}
```

Expected output:

```text
Count: 4
Total: 11.0 m3
Large: [4.0]
No order at index 10
```

`.copied()` converts borrowed `f64` items into copied values. The `filter` predicate still receives a reference to each item, hence `*volume`. This is a closure: a small callable expression. `:?` requests debug formatting.

### Checkpoint

Change `.get(10)` to `.get(0)`: it finds `2.5`. Prefer `.get()` when the index may be invalid; direct indexing panics if out of bounds.

### Challenge

Calculate the average safely for both a populated vector and an empty vector. Return `Option<f64>` from `fn average(values: &[f64])`. A slice borrows a sequence without requiring ownership of the vector.

### Review

Why is “no average” different from an average of zero?

---

## Lab 06 — Recoverable errors with Result

### Objective

Convert text into a valid positive quantity and explain failures without panicking.

### Concept

`Result<T, E>` contains `Ok(T)` on success or `Err(E)` on failure. `?` unwraps success or returns the error to the caller, with supported error conversion when necessary. It does not catch an exception. `.map_err()` changes an error's representation.

### Exercise

Replace `src/main.rs` with:

```rust
fn parse_volume(text: &str) -> Result<f64, String> {
    let volume = text
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Invalid number: {text}"))?;

    if !volume.is_finite() || volume <= 0.0 {
        return Err(String::from("Volume must be finite and greater than zero"));
    }
    Ok(volume)
}

fn main() {
    for input in ["2.5", "oops", "0", "NaN"] {
        match parse_volume(input) {
            Ok(volume) => println!("Accepted: {volume:.1} m3"),
            Err(error) => println!("Rejected: {error}"),
        }
    }
}
```

Run `cargo run --quiet`.

```text
Accepted: 2.5 m3
Rejected: Invalid number: oops
Rejected: Volume must be finite and greater than zero
Rejected: Volume must be finite and greater than zero
```

The `::<f64>` syntax explicitly chooses the parse result type. `NaN` and infinity can be floating-point values, so checking only `volume <= 0.0` is insufficient. The predicate explicitly rejects non-finite values.

For this small course, errors are strings. A larger reusable library often benefits from a dedicated error enum so callers can distinguish failures without parsing messages.

### Checkpoint

Test `" 3.0 "`, `"-1"`, and `"inf"`. The first is accepted; the others are rejected. Decimal input uses a dot: `2.5`, not `2,5`.

### Challenge

Reject values above `8.0` with `Volume exceeds capacity`. Accept exactly `8.0`. Keep the finite/positive check first.

### Review

Why is `.unwrap()` a poor default for parsing input supplied by a user?

---

## Lab 07 — Modules, libraries, and traits

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

## Lab 08 — Test behavior and build confidence

### Objective

Protect validation rules with focused tests and run a repeatable quality workflow.

### Concept

`#[test]` marks a test function. `#[cfg(test)]` includes a module when building tests. Tests should protect meaningful behavior, including failures and boundaries. Floating-point comparisons often require a tolerance.

### Exercise

Keep the files from Lab 07. Append this module to `src/volume.rs`, below `parse_volume`:

```rust
#[cfg(test)]
mod tests {
    use super::parse_volume;

    #[test]
    fn accepts_trimmed_positive_input() {
        let value = parse_volume(" 2.5 ").expect("valid fixture");
        assert!((value - 2.5).abs() < 1e-9);
    }

    #[test]
    fn rejects_non_numeric_input() {
        assert!(parse_volume("oops").is_err());
    }

    #[test]
    fn rejects_non_positive_and_non_finite_input() {
        for input in ["0", "-1", "NaN", "inf"] {
            assert!(parse_volume(input).is_err(), "input was {input}");
        }
    }
}
```

Run:

```text
cargo fmt
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Expected result: three library tests pass, formatting succeeds, and Clippy reports no warnings for the baseline code. Cargo may also show binary and documentation test sections with zero tests. Lint behavior can evolve across toolchain releases; read any new diagnostic rather than suppressing it automatically.

The `--` in the Clippy command separates Cargo options from arguments passed to Clippy/rustc. Here `-D warnings` treats warnings as errors.

### Checkpoint — Prove that the tests matter

Temporarily change `volume <= 0.0` to `volume < 0.0` in the parser. `cargo test` should fail because zero is now accepted. Restore the rule and rerun the tests.

### Challenge

Add a test for an empty string and a whitespace-only string. Both must fail to parse.

### Review

Why is a successful build alone insufficient evidence that validation rules are correct?

---

## Lab 09 — Capstone: build the Batch Order CLI

### Objective

Combine parsing, collections, modules, file I/O, error propagation, tests, and release builds into one usable tool.

### Acceptance criteria

- Accept exactly one input file path.
- Treat each nonblank line as one order quantity in m3.
- Accept only finite, positive numbers; ignore blank lines.
- Reject an empty or blank-only file.
- Report the physical line number for an invalid quantity.
- Reject totals that overflow to a non-finite value.
- Print count and total with two decimal places on success.
- Print an error and return a nonzero exit status on failure.

### Step 1 — Create a separate package

Starting in the `rust_lab` directory:

```text
cd ..
cargo new batch_cli --edition 2024
cd batch_cli
```

All following commands run from `batch_cli`. Leave the generated `Cargo.toml` unchanged; no dependencies are needed. The package name must remain `batch_cli` for the imports below.

### Step 2 — Implement the library

Create `src/lib.rs` with this complete code:

```rust
#[derive(Debug)]
pub struct Summary {
    pub count: usize,
    pub total_m3: f64,
}

pub fn parse_volume(text: &str) -> Result<f64, String> {
    let volume = text.trim().parse::<f64>()
        .map_err(|_| format!("Invalid number: {text}"))?;
    if !volume.is_finite() || volume <= 0.0 {
        return Err(String::from("Volume must be finite and greater than zero"));
    }
    Ok(volume)
}

pub fn parse_orders(contents: &str) -> Result<Vec<f64>, String> {
    let mut orders = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let volume = parse_volume(line)
            .map_err(|error| format!("Line {}: {error}", index + 1))?;
        orders.push(volume);
    }
    if orders.is_empty() {
        return Err(String::from("No orders found"));
    }
    Ok(orders)
}

pub fn summarize(orders: &[f64]) -> Result<Summary, String> {
    if orders.is_empty() {
        return Err(String::from("No orders found"));
    }
    let mut total_m3 = 0.0;
    for &volume in orders {
        if !volume.is_finite() || volume <= 0.0 {
            return Err(String::from("Invalid order volume"));
        }
        total_m3 += volume;
        if !total_m3.is_finite() {
            return Err(String::from("Total volume exceeds numeric range"));
        }
    }
    Ok(Summary { count: orders.len(), total_m3 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_valid_orders() {
        let orders = parse_orders("2.5\n4.0\n1.5\n").unwrap();
        let summary = summarize(&orders).unwrap();
        assert_eq!(summary.count, 3);
        assert!((summary.total_m3 - 8.0).abs() < 1e-9);
    }

    #[test]
    fn supports_crlf_and_blank_lines() {
        assert_eq!(parse_orders("2.5\r\n\r\n 1.5 \r\n").unwrap(), vec![2.5, 1.5]);
    }

    #[test]
    fn preserves_physical_error_line_number() {
        let error = parse_orders("2.5\n\noops\n").unwrap_err();
        assert_eq!(error, "Line 3: Invalid number: oops");
    }

    #[test]
    fn rejects_empty_input() {
        for input in ["", " \n\r\n"] {
            assert_eq!(parse_orders(input).unwrap_err(), "No orders found");
        }
    }

    #[test]
    fn rejects_invalid_quantities() {
        for input in ["0", "-2", "NaN", "inf", "oops"] {
            assert!(parse_orders(input).is_err());
        }
    }

    #[test]
    fn rejects_overflowing_total() {
        assert!(summarize(&[f64::MAX, f64::MAX]).is_err());
    }

    #[test]
    fn rejects_invalid_direct_summary_input() {
        assert!(summarize(&[]).is_err());
        assert!(summarize(&[0.0]).is_err());
        assert!(summarize(&[f64::NAN]).is_err());
    }
}
```

Read before continuing:

- `.enumerate()` yields a zero-based index and each line. Display `index + 1` for human-readable line numbers.
- `?` exits immediately on the first bad line, so a partial total is never reported as success.
- `&[f64]` lets `summarize` borrow a vector or array as a slice.
- `summarize` validates again because it is public and callers may bypass the text parser.
- `.unwrap()` is used only in tests where success is part of the fixture's assertion.

### Step 3 — Implement the command-line boundary

Replace `src/main.rs` with:

```rust
use batch_cli::{parse_orders, summarize};
use std::{env, fs, process::ExitCode};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let path = args.next()
        .ok_or_else(|| String::from("Usage: batch_cli <orders-file>"))?;
    if args.next().is_some() {
        return Err(String::from("Usage: batch_cli <orders-file>"));
    }

    let contents = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {:?}: {error}", path))?;
    let orders = parse_orders(&contents)?;
    let summary = summarize(&orders)?;

    println!("Orders: {}", summary.count);
    println!("Total volume: {:.2} m3", summary.total_m3);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
```

`args_os` supports platform-native file paths. `.skip(1)` skips the executable name. `ok_or_else` converts a missing argument from `Option` into a `Result` error. `Result<(), String>` means successful execution has no additional value to return. `read_to_string` reads UTF-8 text; invalid UTF-8 is reported as a file-reading error.

### Step 4 — Create an input fixture

Create a folder named `data` beside `Cargo.toml`. Create `data/orders.txt` as UTF-8 text with exactly:

```text
2.5
4.0
1.5
```

Save as UTF-8 **without a byte-order mark (BOM)**. The format is one decimal quantity per line: no header, commas, units, or comments.

Run:

```text
cargo run --quiet -- data/orders.txt
```

Expected application output:

```text
Orders: 3
Total volume: 8.00 m3
```

The first `--` passes subsequent arguments to your program, rather than to Cargo. Relative file paths are resolved from the terminal's current working directory, not from `src/main.rs`.

### Step 5 — Test failure behavior manually

Create `data/invalid.txt` with `2.5` on line 1 and `oops` on line 2. Then run the following cases individually:

| Command | Expected result |
|---|---|
| `cargo run --quiet -- data/invalid.txt` | `Error: Line 2: Invalid number: oops` |
| `cargo run --quiet` | `Error: Usage: batch_cli <orders-file>` |
| `cargo run --quiet -- missing.txt` | Error beginning with `Could not read`; OS details vary |
| `cargo run --quiet -- data/orders.txt extra` | Usage error |

After a failed run, immediately check the exit status:

- PowerShell: `$LASTEXITCODE`
- Bash/zsh: `echo $?`

The status should be nonzero. Do not run another command before checking it.

### Step 6 — Add an integration test

Create a `tests` folder beside `src`. Create `tests/cli.rs`:

```rust
use std::process::Command;

#[test]
fn cli_prints_summary_for_fixture() {
    let output = Command::new(env!("CARGO_BIN_EXE_batch_cli"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("data/orders.txt")
        .output()
        .expect("start CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let normalized = stdout.replace("\r\n", "\n");
    assert_eq!(normalized, "Orders: 3\nTotal volume: 8.00 m3\n");
}

#[test]
fn cli_fails_without_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_batch_cli"))
        .output()
        .expect("start CLI");
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("Usage:"));
}
```

Cargo supplies the executable location to integration tests. The success test depends on the unchanged fixture in `data/orders.txt`; keep the invalid case in a separate file.

### Step 7 — Run the quality gate and release build

```text
cargo fmt
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Expected: **7 unit tests and 2 integration tests pass**, with no failing checks. Run the built executable directly:

Windows PowerShell:

```powershell
.\target\release\batch_cli.exe data/orders.txt
```

macOS/Linux:

```bash
./target/release/batch_cli data/orders.txt
```

Expect the same two-line summary. A release build is optimized and is built for your current target platform; it does not automatically create executables for every operating system.

### Final checkpoint

You can demonstrate successful input, invalid input, missing arguments, file errors, and a passing test suite. You can explain why the library has no dependency on CLI arguments or file paths.

### Independent challenge

Add an average to the summary and print `Average volume: 2.67 m3` for the fixture. Add a unit-test assertion using a tolerance and update the integration test's expected output. See Appendix A only after trying it.

### Practical limitations

This CLI loads the whole file into memory and uses binary floating-point arithmetic. It is suitable for learning and modest text files. It does not model plant capacity, business-specific rounding, persistence, concurrency, or operational safety. A real system needs explicit units, numeric policies, domain validation, and bounded input handling.

---

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


For platform-specific setup, bonus scenarios, and runnable snapshots, use the [course homepage](../README.md).
