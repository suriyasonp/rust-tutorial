# Bonus Lab 11 — Summarize service logs

**Time:** 30–50 minutes. **Prerequisites:** Labs 00–09. **Scenario:** An engineer wants a quick count of informational, warning, and error messages from a service export.

## Objective and input contract

Read records in the format `LEVEL|message`. Supported levels are uppercase `INFO`, `WARN`, and `ERROR`. Ignore blank lines, trim the level, and require a nonblank message. Keep additional `|` characters inside the message. Reject the first malformed record with a physical line number. An empty log is valid and produces zero counts.

## Concept

`split_once('|')` separates the first delimiter only. `Default` generates zero values for numeric counter fields. `include_str!` embeds a UTF-8 fixture at compile time, relative to the source file; unlike Lab 09's runtime file reading, changes to that fixture require rebuilding.

## Guided exercise

1. In your separate learning folder run `cargo new log_summary --edition 2024`, then `cd log_summary`.
2. Create `data/service.txt` next to `src`, with:

```text
INFO|Service started
WARN|Connection retry
ERROR|Import failed
INFO|Service recovered
```

3. Define `#[derive(Debug, Default, PartialEq)] struct Counts` with `info`, `warn`, and `error` fields, each `usize`.
4. Write `fn summarize(text: &str) -> Result<Counts, String>`. Start with `Counts::default()` and iterate using `text.lines().enumerate()`.
5. Ignore whitespace-only lines. For each remaining line, call `split_once('|')` and convert `None` into a missing-separator error with `ok_or_else`. Display `index + 1`.
6. Reject an empty trimmed message. Match `level.trim()` to update the appropriate field or return an unknown-level error. Return `Ok(counts)` after the loop.
7. Use this `main` function:

```rust
fn main() {
    let input = include_str!("../data/service.txt");
    match summarize(input) {
        Ok(counts) => println!("INFO={} WARN={} ERROR={}", counts.info, counts.warn, counts.error),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}
```

8. Run `cargo run --quiet`. Expected: `INFO=2 WARN=1 ERROR=1`.
9. Add tests for the fixture; malformed records `INFO`, `INFO| `, and `DEBUG|message`; an empty log; and `INFO|left|right`. Then run `cargo fmt`, `cargo test`, and Clippy as in Lab 08.

## Checkpoint

Insert a blank line before a malformed record and confirm the reported line number still matches the physical line in the file. A malformed log must return an error, not a success containing partial counts.

## Challenge

Use a file path argument at runtime instead of `include_str!`. Follow Lab 09's `args_os`, `read_to_string`, `Result`, and `ExitCode` structure. Accept exactly one path; a missing path and an unreadable file must produce a nonzero status.

## Solution and review

Compare your baseline with the [complete implementation and tests](../solutions/log-summary/src/main.rs). For the runtime challenge, copy Lab 09's `run`/`main` boundary, replace `parse_orders` and its summary call with `summarize(&contents)?`, and print the three counters. Keep the parser independent of files and arguments. Run with `cargo run -- data/service.txt`.

Review: Why can the empty log be valid while an empty batch-order import is rejected? Input validity depends on the domain contract; there is no universal rule that every empty collection is an error.

[Previous](10-sensor-validation.md) · [Course home](../README.md)
