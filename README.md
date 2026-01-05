# Coop-Sweeper

A minesweeper game built in Rust using the [Iced](https://iced.rs/) GUI framework.
> This is my first attempt at making a GUI game in Rust, so ignore any bugs or design decisions :P

## Download

**[⬇️ Latest Release from GitHub Actions](https://github.com/CoopTRUE/coop-sweeper/actions/workflows/build.yml)**

| Platform | File |
|----------|------|
| Linux x86_64 | `coop-sweeper-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `coop-sweeper-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Apple Silicon | `coop-sweeper-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `coop-sweeper-x86_64-pc-windows-msvc.zip` |

## Controls

| Action | Input |
| ------ | ----- |
| Reveal cell | Left click |
| Flag cell | Right click |
| Chord (reveal neighbors) | Left click (on already revealed cells) |

## Features

- Customizable grid size (5–50 rows/columns)
- Adjustable mine count
- Cascade reveal for empty cells
- Chording support for faster gameplay
- Game over overlay with mine reveal

## Building from Source

```bash
cargo build --release
```

The binary will be at `target/release/coop-sweeper`.
