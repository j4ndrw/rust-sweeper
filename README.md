# Rust-Sweeper

A minimal one-handed CLI minesweeper game.

# Installation
1. Clone the repo:
```console
git clone https://github.com/j4ndrw rust-sweeper
```
2. Build the binary with `cargo` (find installation guide [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)):
```console
cargo build --release
```

# Usage
```console
USAGE:
    rust-sweeper --difficulty <DIFFICULTY>
# where DIFFICULTY can be 0, 1 or 2.
```

# Difficulties
- 0 is the easiest (9x9 board with 10 bombs)
- 1 is medium (16x16 board with 40 bombs)
- 2 is the hardest (30x16 board with 99 bombs)

# Controls
- `WSAD` for moving the cursor in a single step
- `Alt+[W|S|A|D]` for moving the cursor in a double step
- `f` to toggle flags
- `Space` or `e` to reveal a tile
- `q` or `Ctrl+C` to quit