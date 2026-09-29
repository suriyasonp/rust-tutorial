[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 02 — Functions and control flow

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

[Previous lab](01-variables.md) · [Next lab](03-ownership.md)
