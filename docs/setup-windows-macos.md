# Set up Rust on Windows and macOS

Allow 30–60 minutes. Complete only the section for your operating system, then perform the common verification. Administrator access may be needed for native build tools.

## Tool checklist

| Tool | Purpose | Required? |
|---|---|---|
| Rust stable through rustup | Compiler, standard library, Cargo | Yes |
| Native C/C++ build tools | Link executables for your operating system | Yes |
| VS Code | Editor and integrated terminal | Recommended; another editor works |
| rust-analyzer VS Code extension | Completion, navigation, inline diagnostics | Recommended |
| rustfmt and Clippy | Formatting and lint checks | Yes for the quality exercises |
| Git | Clone this repository and save progress | Recommended; ZIP download is an alternative |

You do not need Docker, Node.js, Python, a database, a cloud account, or a paid IDE for the Rust exercises.

## Windows: PowerShell and native MSVC

1. Open the [official Rust installer page](https://www.rust-lang.org/tools/install) and download the Windows installer matching your machine. Prefer the default native MSVC toolchain for this path.
2. Run `rustup-init.exe`. If prerequisites are missing, follow its Visual Studio installation prompt. Alternatively, install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) and select **Desktop development with C++**, including the MSVC tools for your architecture and a Windows SDK. The full Visual Studio IDE is not required.
3. Complete the default stable Rust installation. Reopen PowerShell so that its PATH includes the installed tools.
4. Install [VS Code](https://code.visualstudio.com/) and [Git for Windows](https://git-scm.com/downloads/win) if you will clone the repository.
5. In VS Code, press `Ctrl+Shift+X`, search for **rust-analyzer**, and install the extension published by **rust-lang**. Open a project folder rather than an isolated `.rs` file.
6. Open **Terminal > New Terminal**, using PowerShell, and complete common verification below.

Do not mix Windows Cargo and WSL Cargo during the course. WSL is an alternative Linux environment, not a prerequisite for this Windows path. If `link.exe` cannot be found, recheck the C++ workload and SDK, not just the Rust extension.

## macOS: Terminal and native Apple tools

1. Open Terminal and install Apple's Command Line Tools if absent:

```bash
xcode-select --install
```

Complete the system dialog. If already installed, the command may say so. Verify with `xcode-select -p` and `clang --version`.

2. Visit the [official Rust installation page](https://www.rust-lang.org/tools/install). Its standard Unix installer command is:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default stable installation. This command downloads and executes the official installation script; use the linked official instructions if your organization's installation policy requires another method.

3. Reopen Terminal, or load the environment into the current shell:

```bash
source "$HOME/.cargo/env"
```

4. Install [VS Code](https://code.visualstudio.com/). Press `Cmd+Shift+X` and install **rust-analyzer** by **rust-lang**. Open your project folder.
5. Run `git --version`; the Command Line Tools normally provide Git. If it is unavailable, use the [official Git options](https://git-scm.com/downloads/mac).
6. On Apple Silicon, prefer native Terminal and a native toolchain. `rustup show` normally reports `aarch64-apple-darwin`; Intel Macs normally report `x86_64-apple-darwin`. Do not force an Intel toolchain on an M-series Mac just to follow these labs.

## Common verification

In a newly opened terminal:

```text
rustup show
rustc --version
cargo --version
git --version
rustup component add rustfmt clippy
```

The course uses Rust edition 2024, requiring Rust 1.85 or later. A Rust **edition** is a language compatibility choice in `Cargo.toml`; it is not the same as the compiler release number. The repository asks rustup for stable through `rust-toolchain.toml`.

Create a disposable smoke-test project outside the cloned repository:

```text
cargo new rust_setup_check --edition 2024
cd rust_setup_check
cargo run
cargo test
```

**Pass:** `Hello, world!` appears and Cargo finishes successfully. Zero tests in a fresh binary project is expected. If installation fails, resolve that before starting the labs.

## Get the course

From a folder where you keep learning projects:

```text
git clone https://github.com/suriyasonp/rust-tutorial.git
cd rust-tutorial
```

If studying a pull request before merge, check out that PR's branch after cloning. Alternatively use GitHub's **Code > Download ZIP** on the appropriate branch and extract the archive. Open the extracted repository folder in VS Code.

**Choose one working mode:**

- **Build it yourself (recommended):** Follow Lab 00 and create `rust-hands-on` alongside the cloned repository. Work in that separate learning folder through the labs. This avoids accidentally nesting new packages inside the solutions workspace.
- **Inspect a completed solution:** From the cloned repository root run `cargo run -p lab01` or `cargo test --workspace`. See [solution instructions](../solutions/README.md). Return to your own learning folder for the exercises.

The `code .` command is optional. On macOS you can enable it using VS Code's Command Palette: **Shell Command: Install 'code' command in PATH**. Opening the folder from the editor menu also works.

## Troubleshooting checkpoint

- `cargo` missing: restart the terminal and confirm rustup installed successfully. Rust's tools normally live in `%USERPROFILE%\.cargo\bin` on Windows or `~/.cargo/bin` on macOS.
- Linker missing: revisit platform prerequisites above.
- Proxy or certificate error: follow your organization's approved network configuration; do not disable TLS verification.
- Editor shows unresolved imports: open the folder containing `Cargo.toml`, then run `cargo check` in its terminal.

Sources: [Rust installation](https://doc.rust-lang.org/book/ch01-01-installation.html), [rustup](https://rust-lang.github.io/rustup/installation/), [Rust 2024 release](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/), and [Rust in VS Code](https://code.visualstudio.com/docs/languages/rust).

[Back to course](../README.md) · [Start Lab 00](../labs/00-environment.md)
