# Completed solutions

Attempt the corresponding lab before opening its solution. These snapshots contain the baseline exercises, not every optional challenge.

From the repository root:

| Package | Lab | Run |
|---|---|---|
| `lab01` | 01 | `cargo run -p lab01` |
| `lab02` | 02 | `cargo run -p lab02` |
| `lab03` | 03, working borrowing example | `cargo run -p lab03` |
| `lab04` | 04 | `cargo run -p lab04` |
| `lab05` | 05 | `cargo run -p lab05` |
| `lab06` | 06 | `cargo run -p lab06` |
| `rust_lab` | 07–08 | `cargo run -p rust_lab` |
| `batch_cli` | 09 | `cargo run -p batch_cli -- solutions/batch-cli/data/orders.txt` |
| `sensor_validation` | 10 | `cargo run -p sensor_validation` |
| `log_summary` | 11 | `cargo run -p log_summary` |

Run all tests with `cargo test --workspace`. Run `cargo fmt --all` before checking formatting with `cargo fmt --all --check`. Lint with `cargo clippy --workspace --all-targets -- -D warnings`.

Files are standard Cargo projects. Since they belong to a workspace, builds normally go into the repository's shared `target` directory. This differs from the standalone learner packages, whose `target` directory belongs to each project.

The `batch_cli` fixture path is relative to your terminal, so the root command above includes `solutions/batch-cli/`. Inside `solutions/batch-cli`, use `cargo run --quiet -- data/orders.txt`. The integration test sets its own working directory explicitly.

Lab 00 has no snapshot because it uses Cargo's generated hello-world project. The compiler-error examples in Lab 03 are experiments to type separately, not broken solution packages.

[Return to course](../README.md)
