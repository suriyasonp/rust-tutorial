[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

# Lab 09 — Capstone: build the Batch Order CLI

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

[Previous lab](08-testing.md) · [Next: sensor scenario](10-sensor-validation.md)
