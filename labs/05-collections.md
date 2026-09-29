[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 05 — Collections, iterators, and Option

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

[Previous lab](04-domain-models.md) · [Next lab](06-errors.md)
