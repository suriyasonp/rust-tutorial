# Rust Hands-On Tutorial

**Read the [course website](https://suriyasonp.github.io/rust-tutorial/) for a navigable version of every lesson.**

Learn Rust by building small, practical tools. This English, self-paced course follows an original Microsoft Learn-inspired structure: **objective → concept → guided exercise → expected result → challenge → review**. It is not an official Microsoft course.

**Audience:** Developers new to Rust; basic experience with functions, conditions, and loops is helpful. C#/.NET and TypeScript developers will find familiar comparisons.  
**Duration:** Approximately 7–9 hours for the core path, plus 60–90 minutes for bonus scenarios.  
**Platforms:** Windows and macOS, with Linux CI coverage.  
**Requirements:** Stable Rust 1.85+ / edition 2024. The exercises have no third-party Rust dependencies.

## Start here

1. Read [Rust foundations](docs/rust-foundations.md).
2. Complete [Windows or macOS setup](docs/setup-windows-macos.md).
3. Follow the labs in order in your own learning folder **outside this repository**.
4. Compare with the [runnable solutions](solutions/README.md) only after attempting an exercise.

Every core lab has an objective, explanation, copyable code, expected output, a checkpoint, and an independent challenge. Deliberately broken code is clearly marked. The [challenge answers](docs/challenge-solutions.md) also explain the review questions.

## Core learning path

| Lab | Topic | What you build |
|---|---|---|
| [00: Environment](labs/00-environment.md) | Environment | First executable |
| [01: Variables and types](labs/01-variables.md) | Variables and types | Quantity calculator |
| [02: Functions and control flow](labs/02-functions.md) | Functions and control flow | Batch classifier |
| [03: Ownership and borrowing](labs/03-ownership.md) | Ownership and borrowing | Borrowing experiments |
| [04: Structs and enums](labs/04-domain-models.md) | Structs and enums | Order state model |
| [05: Collections and Option](labs/05-collections.md) | Collections and Option | Collection summary |
| [06: Error handling with Result](labs/06-errors.md) | Error handling with Result | Validated quantity parser |
| [07: Modules and traits](labs/07-modules-traits.md) | Modules and traits | Reusable library |
| [08: Testing and quality](labs/08-testing.md) | Testing and quality | Validation test suite |
| [09: Batch Order CLI](labs/09-batch-cli.md) | Batch Order CLI | File-driven order summary |

## Real-world practice

| Scenario | Skill transfer | Exercise |
|---|---|---|
| Batch order import | File boundaries, validation, meaningful errors and exit codes | [Capstone](labs/09-batch-cli.md) |
| Sensor validation | Invalid numeric readings and alarm thresholds | [Bonus Lab 10](labs/10-sensor-validation.md) |
| Service log summary | Parsing text, counting events, rejecting malformed records | [Bonus Lab 11](labs/11-log-summary.md) |

These are educational tools with explicitly bounded formats, not production plant control software.

## Run a completed example

After cloning and entering the repository root:

```text
cargo run -p lab01
cargo run -p batch_cli -- solutions/batch-cli/data/orders.txt
cargo run -p sensor_validation
cargo run -p log_summary
cargo test --workspace
```

Expected capstone result: `Orders: 3` and `Total volume: 8.00 m3`.

The workspace contains finished snapshots. For the step-by-step learning path, create your own packages as instructed in Lab 00; do not overwrite solution files unless deliberately experimenting. Labs 07 and 08 share a completed snapshot named `rust_lab`.

## Reference material

- [Full core workbook in one file](docs/complete-workbook.md)
- [Solutions, troubleshooting, C# comparisons, and completion checklist](docs/challenge-solutions.md)
- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Official Cargo Book](https://doc.rust-lang.org/cargo/)
- [Validation scope](docs/validation.md)

## Verification

The GitHub Actions workflow checks the finished solutions on Windows, macOS, and Linux. It compiles, runs tests, lints, and builds release binaries. See the repository's Actions tab for the current result; the presence of a workflow alone is not evidence that it passed.

The tutorial's intentional compiler-error experiments are excluded from the runnable solutions. The initial writing environment did not contain Rust; verification status and coverage are documented separately.

## License

See the existing [MIT license](LICENSE).
