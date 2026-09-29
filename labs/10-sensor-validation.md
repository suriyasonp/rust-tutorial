# Bonus Lab 10 — Validate sensor samples

**Time:** 30–40 minutes. **Prerequisites:** Labs 00–08. **Scenario:** A monitoring tool receives simulated percentage readings from a sensor; it must distinguish high readings from invalid readings.

## Objective and rules

Build a classifier with three outcomes: `Normal`, `High`, and `Invalid`. Valid measurements are finite values in the inclusive range 0–100. A reading of 80 or higher is high. These are exercise-specific thresholds, not a hardware specification.

## Concept

Validation must precede business classification. An out-of-range reading such as 150 should be invalid rather than simply high. `PartialEq` enables equality assertions for the enum. The `contains` method on an inclusive range helps describe both bounds.

## Guided exercise

1. In your separate learning folder, create a new package with `cargo new sensor_validation --edition 2024`, then `cd sensor_validation`.
2. Replace `src/main.rs` with the starter below. `todo!` compiles but panics when executed; implement it before running.

```rust
#[derive(Debug, PartialEq)]
enum Reading {
    Normal,
    High,
    Invalid,
}

fn classify(value: f64) -> Reading {
    todo!("validate the range, then check the high threshold")
}

fn main() {
    for value in [42.0, 80.0, -1.0, f64::NAN] {
        println!("{value}: {:?}", classify(value));
    }
}
```

3. First return `Reading::Invalid` if `!value.is_finite()` or `!(0.0..=100.0).contains(&value)`.
4. Otherwise return `Reading::High` if `value >= 80.0`; return `Reading::Normal` for the remaining values.
5. Run `cargo run --quiet` and compare:

```text
42: Normal
80: High
-1: Invalid
NaN: Invalid
```

6. Add unit tests for `0.0`, `79.9`, `80.0`, `100.0`, `100.1`, and `f64::INFINITY`. Use `assert_eq!` on the returned enum.
7. Run `cargo fmt`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`.

## Checkpoint

Exactly 80 is high, exactly 100 is high, and 100.1 is invalid. You can explain why checking the high threshold before the valid range would be a bug.

## Challenge

Count the normal, high, and invalid samples in the input array. Expected counts: 1 normal, 1 high, and 2 invalid. Use a `match` inside the loop.

## Solution and review

The [complete baseline and tests](../solutions/sensor-validation/src/main.rs) are available after your attempt. For the challenge, initialize three counters to zero, match `classify(value)`, and increment the corresponding counter. Do not discard invalid samples without tracking their frequency; a monitoring system may need to surface that as a separate signal.

Review: Why is an invalid sample different from a valid high sample? The first describes unreliable/out-of-contract input, while the second is a valid measurement meeting an alarm condition.

[Previous](09-batch-cli.md) · [Next](11-log-summary.md) · [Course home](../README.md)
