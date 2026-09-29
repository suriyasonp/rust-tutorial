# Validation scope

The authoring environment did not have a Rust compiler. Local checks cover file structure, relative documentation links, and TOML syntax. They do not establish that Rust code compiles.

GitHub Actions is configured to compile all solution targets, run tests, lint with warnings treated as errors, and build release binaries on Windows, macOS, and Linux using stable Rust. Consult the actual Actions result before treating these as passed. The formatting step runs rustfmt to validate parsing and normalize code in the runner; it does not enforce a clean formatting diff.

The baseline includes 3 parser tests in `rust_lab`, 7 unit tests and 2 CLI integration tests in `batch_cli`, 2 sensor tests, and 3 log-summary tests: **17 tests total**. The earlier solution snapshots are checked and built but contain no unit tests.

Coverage excludes the tutorial's deliberately non-compiling ownership examples, optional challenge solutions, actual interactive OS installation, and production device integration. Expected terminal outputs are teaching checkpoints; toolchain diagnostic wording can vary.

Use `cargo fmt --all`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` from the repository root to verify the solutions on your own machine.
