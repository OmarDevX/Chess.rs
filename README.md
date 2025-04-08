# 🧠♟️ Rust Chess Game with egui

A simple and elegant chess game built using [Rust](https://www.rust-lang.org/) and [egui](https://github.com/emilk/egui). This app supports two game modes:

- 🔁 **Local Multiplayer**: Two players can play on the same PC (White vs Black).
- 🤖 **Play vs Stockfish**: Challenge the powerful [Stockfish](https://stockfishchess.org/) engine.

---

## 🚀 Features

- Clean and intuitive GUI powered by egui.
- Two game modes: Local PvP and vs Stockfish AI.
- Fully functional chess rules and piece movement.
- Turn-based gameplay with color switching.

---

## 🛠️ Requirements

- Rust (latest stable recommended) → [Install Rust](https://www.rust-lang.org/tools/install)
- Stockfish binary (if you want to play against the engine)
  - Download from [https://stockfishchess.org/download](https://stockfishchess.org/download)
  - Make sure it's executable and in your system `PATH`.

---

## 📦 Running the Game

Clone the repository and run it using Cargo:

```bash
git clone https://github.com/OmarDevX/Chess.rs.
cd rust-chess-egui
cargo run
