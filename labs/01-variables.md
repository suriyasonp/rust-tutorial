[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 01 — Variables, types, and expressions

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

[Previous lab](00-environment.md) · [Next lab](02-functions.md)
