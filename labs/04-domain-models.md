[Course home](../README.md) · [Setup](../docs/setup-windows-macos.md) · [Solutions](../docs/challenge-solutions.md)

**Working directory:** Your own `rust-hands-on/rust_lab` package created in Lab 00. Complete earlier labs first.

# Lab 04 — Model data with structs and enums

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

[Previous lab](03-ownership.md) · [Next lab](05-collections.md)
