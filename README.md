# Sky Courier

Sky Courier is a game project using Rust 2024. The current executable is a placeholder that prints `Hello, world!`.

## Setup

Install Rust with rustup. The project selects the stable channel through `rust-toolchain.toml` and requires Rust 1.97.1 or newer. The toolchain file also declares rustfmt, Clippy, and the Rust standard library sources for IDE navigation.

Open the repository root in RustRover so it can load `Cargo.toml`. From the same directory in a terminal, confirm the toolchain and run the placeholder:

```sh
rustup show active-toolchain
rustc --version
cargo run --locked
```

If the installed stable compiler is older than 1.97.1, run `rustup update stable`. If RustRover reports an invalid toolchain, check that `cargo --version` and `rustc --version` work in a terminal. For a standard rustup installation, the RustRover toolchain directory is `~/.cargo/bin`.

## Local checks

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Run these checks locally before committing. There are currently zero tests, so `cargo test` verifies that the test target builds.

## Dependencies and project files

- `Cargo.toml` declares Bevy 0.20 as an optional dependency. Use `--features bevy` on Cargo commands when working on code that uses it. The default checks skip compiling the engine.
- `Cargo.lock` records the resolved dependency versions. Include it in version control so `--locked` builds use the same versions.
- `.editorconfig` defines editor defaults, and `.gitattributes` normalizes line endings. Cargo's `target/` directory and local RustRover settings in `.idea/` are ignored.
