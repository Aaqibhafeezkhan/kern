# K.E.R.N.

**Keyboard-Engineered Runtime for Navigation**

[![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`kern` is a command-line code editor built with Rust, designed to bring code editing directly to your terminal.

## Features

- **Terminal-based** — Edit code directly from your command line.
- **Rust-powered** — Built with Rust for performance and reliability.
- **Lightweight** — A terminal-focused editing experience.
- **Open source** — Licensed under the MIT License.

> More features and details will be added as termcode evolves.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo, if building from source.

### Build from source

Clone the repository:

```bash
git clone https://github.com/dominikgobel/KERN
cd KERN
```

Build the project:

```bash
cargo build --release
```

Run the editor:

```bash
cargo run --release
```

Once the command-line interface is configured, you can also run the compiled binary directly from `target/release/`.

## Usage

Once you have installed `kern`, you can run it from your terminal:

```bash
kern path/to/file.rs
```

## Terminal Interface

`kern` uses [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal interaction.

Crossterm provides raw terminal mode, keyboard and resize events, cursor positioning, and screen management. This avoids implementing platform-specific terminal handling ourselves.

Crossterm is distributed under the MIT license.

The editor core remains independent of terminal input and rendering. Terminal events are translated into editor operations, and the renderer displays the resulting editor state.


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
cargo test
```

Check the code with Clippy:

```bash
cargo clippy --all-targets --all-features
```

Format the code:

```bash
cargo fmt
```

## Roadmap

Future goals for `kern` are

- to support more file types
- to be configurable
- to support more editing features and QoL improvements
- to support editing multiple files in a workspace simultaneously
- to have some kind of git integration

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

Made with Rust 🦀