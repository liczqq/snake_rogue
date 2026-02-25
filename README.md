# Snake Rogue 🐍

A fun and addictive snake game with power-ups, built with Rust and eframe.

![Game Screenshot](screenshot.png)

## Features

- 🎮 Classic snake gameplay with rogue-like elements
- ⚡ Power-up system (30% spawn chance):
  - 🟨 Speed - Move faster
  - 🟦 Slow - Slow down for precision
  - 🟩 Double Points - 2x score per food
  - 🟧 Invincible - Crash into walls without dying
- 📈 Level system - Level up every 100 points
- 💀 High score tracking
- 🎨 Beautiful UI with animations

## Controls

- ↑↓←→ or WASD - Move
- SPACE - Pause/Restart
- R - Reset game

## Build

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/snake_rogue
```

## Requirements

- Rust 1.70+
- Linux/macOS/Windows with graphics support

## License

MIT
