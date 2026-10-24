# K.E.R.N. v0.1.1

**Keyboard-Engineered Runtime for Navigation**

[![Rust CI](https://github.com/dominikgobel/kern/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/dominikgobel/kern/actions/workflows/ci.yml?query=branch%3Amain)
[![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/dominikgobel/kern)](https://github.com/dominikgobel/kern/releases/latest)

K.E.R.N. (`kern`) is a command-line code editor written in Rust. It brings code editing directly to your terminal, with a lightweight, keyboard-focused interface.

> **Status:** K.E.R.N. is under active development. This is an early release, and features, commands, and workflows may change.

## Features

- **Terminal-based** — Edit code directly in your terminal.
- **Rust-powered** — Built with Rust for performance and reliability.
- **Lightweight** — A terminal-focused editing experience.
- **Cross-platform** — Designed for terminal use across supported operating systems.
- **Open source** — Released under the MIT License.

## Installation

### Option 1: Download a prebuilt release

Prebuilt binaries are available on the [GitHub Releases page](https://github.com/dominikgobel/kern/releases).

Choose the binary that matches your operating system and CPU architecture.

| Platform | Release asset |
|---|---|
| Linux x86_64 | `kern-linux-x86_64` |
| Linux ARM64 | `kern-linux-aarch64` |
| Windows x86_64 | `kern-windows-x86_64.exe` |
| macOS Intel | `kern-macos-x86_64` |
| macOS Apple Silicon | `kern-macos-aarch64` |

These filenames assume the release workflow uses `kern` as its binary name.

#### Linux

Download the appropriate Linux binary from the [latest release](https://github.com/dominikgobel/kern/releases/latest).

For example, on x86_64:

```bash
curl -fL \
  https://github.com/dominikgobel/kern/releases/latest/download/kern-linux-x86_64 \
  -o kern

chmod +x kern
./kern
```

To make `kern` available from anywhere in your terminal:

```bash
mkdir -p "$HOME/.local/bin"
mv kern "$HOME/.local/bin/kern"
```

Make sure `$HOME/.local/bin` is included in your `PATH`. You can then launch the editor with:

```bash
kern
```

#### Windows

1. Open the [latest release](https://github.com/dominikgobel/kern/releases/latest).
2. Download `kern-windows-x86_64.exe`.
3. Open PowerShell in the directory containing the downloaded file.
4. Run the executable:

```powershell
.\kern-windows-x86_64.exe
```

To open a file, pass its path as an argument if file-opening support is available in your version:

```powershell
.\kern-windows-x86_64.exe .\example.rs
```

#### macOS

Download the binary matching your Mac from the [latest release](https://github.com/dominikgobel/kern/releases/latest).

For Apple Silicon:

```bash
curl -fL \
  https://github.com/dominikgobel/kern/releases/latest/download/kern-macos-aarch64 \
  -o kern

chmod +x kern
./kern
```

For an Intel Mac, replace `kern-macos-aarch64` with `kern-macos-x86_64`.

If macOS blocks the executable because it is not signed or notarized, the release may require additional steps before it can run. Do not disable system security protections globally.

### Option 2: Build from source

#### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- Git

Clone the repository:

```bash
git clone https://github.com/dominikgobel/kern.git
cd kern
```

Build the optimized executable:

```bash
cargo build --release
```

Run the project:

```bash
cargo run --release
```

The compiled executable is located in `target/release/`.

## Usage

Start K.E.R.N. from your terminal:

```bash
kern
```

If your version supports opening a file through a command-line argument, you can use:

```bash
kern path/to/file.rs
```

The available commands and editing features depend on the current release.

## Terminal Interface

K.E.R.N. uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal interaction.

Crossterm provides terminal functionality such as raw mode, keyboard and resize events, cursor positioning, and screen management. This avoids implementing platform-specific terminal handling directly.

Crossterm is distributed under the MIT license.

The editor core is designed to remain independent of terminal input and rendering. Terminal events are translated into editor operations, and the renderer displays the resulting editor state.

## Development

Build the project in debug mode:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

Run tests:

```bash
cargo test --locked
```

Check the code with Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Check formatting:

```bash
cargo fmt --all -- --check
```

## Roadmap

Planned areas of development include:

- Support for additional file types
- User configuration and customization
- More editing features and quality-of-life improvements
- Editing multiple files within a workspace
- Git integration

These goals may change as development progresses.

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

Before opening a pull request, please read [CONTRIBUTING.md](CONTRIBUTING.md) and follow the project's contribution guidelines.

For bugs and feature requests, use the repository's [Issues](https://github.com/dominikgobel/kern/issues) page.

## License

K.E.R.N. is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Made with Rust 🦀
