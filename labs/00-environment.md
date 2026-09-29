[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

# Lab 00 — Prepare your environment

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
    println!("Hello, Fore! Welcome to Rust.");
}
```

7. Run:

```text
cargo check
cargo run --quiet
```

`check` checks the code without producing the final executable. `run` builds and executes it. `--quiet` hides normal Cargo progress output, not compiler errors.

### Checkpoint

The program prints `Hello, Fore! Welcome to Rust.` and exits successfully. You can identify the manifest and entry-point file.

### Challenge

Print a second line containing the name of the tool you will build: `Batch Order CLI`.

### Review

Why is Cargo useful even when a program has no external dependencies?

---

[Next lab](01-variables.md)
