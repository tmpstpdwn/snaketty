# Snaketty

A classic Snake game implementation in Rust that runs directly in your terminal!

## Dependencies
- Rust (1.65+).
- Cargo.
- `termion` (terminal handling).
- `k_board` (keyboard input).
- `rand` (random number generation).

## Installation
Ensure Rust and Cargo are installed:
```bash
git clone https://github.com/tmpstpdwn/snaketty.git
cd snaketty
cargo run --release
```

## Controls

- Arrow keys: Change direction.
- Space: Start/Restart game.
- Escape: Quit game.

## How to Play

- Start the game with Space.
- Collect food to grow and increase score.
- Avoid colliding with yourself.
- The snake wraps around screen edges.
- Press Escape to exit anytime.

## LICENSE

This project is licensed under GPL3 [LICENSE](LICENSE).
