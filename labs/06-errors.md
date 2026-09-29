[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 06 — Recoverable errors with Result

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

[Previous lab](05-collections.md) · [Next lab](07-modules-traits.md)
