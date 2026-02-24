## Poob

Poob is a chess engine/library written in Rust. It provides a bitboard-based core for move generation, evaluation and search, plus a simple command‑line interface that understands a subset of the UCI protocol.

> Original project by `alphapitchzeta`. This README is an attempt to document the existing codebase and CLI usage.

### Features

- **Rust chess library**
  - Bitboard representation of positions
  - Legal move generation (including castling, en passant, promotions)
  - Basic evaluation and alpha–beta search with quiescence
- **CLI / UCI front‑end**
  - Interactive REPL over stdin/stdout
  - Supports common UCI commands: `uci`, `isready`, `ucinewgame`, `go`, `position`, etc.
- **Perft tooling**
  - Perft for debugging move generation
  - Perft suite runner for testing positions from a file

### Building

You’ll need a recent stable Rust toolchain.</br>
fork it</br>
git clone https://github.com/your-username/poob.git</br>
cd poob</br>
cargo build --release</br>
