[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 03 — Ownership and borrowing

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

[Previous lab](02-functions.md) · [Next lab](04-domain-models.md)
