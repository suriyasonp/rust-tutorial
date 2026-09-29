[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 08 — Test behavior and build confidence

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

[Previous lab](07-modules-traits.md) · [Next lab](09-batch-cli.md)
