# Dreierschnapsen

A Rust implementation of **Dreierschnapsen** (Three-player Schnapsen), a traditional Austrian card game.

## Overview

This project is organized as a Cargo workspace with three crates:

| Crate | Description |
|-------|-------------|
| **schnapsen-model** | Data structures for cards, game types, and game state |
| **schnapsen-engine** | Game logic, move validation, and AI (random move selection) |
| **schnapsen-cli** | Terminal UI using [ratatui](https://ratatui.rs/) for playing the game |

## Quick Start

```bash
# Build everything
cargo build

# Run the CLI
cargo run -p schnapsen-cli

# Run tests
cargo test
```

## CLI Controls

| Key | Action |
|-----|--------|
| `←` / `→` | Select card |
| `Enter` | Play selected card |
| `Space` | AI makes a move (when it's the AI's turn) |
| `q` / `Esc` | Quit |

## Game Rules

Dreierschnapsen is played with a 20-card deck (4 suits × 5 ranks). Three players participate; one plays against the other two. The goal is to reach 24 points first.

- **Suits**: Herz ♥, Karo ♦, Pik ♠, Kreuz ♣  
- **Ranks**: Ass (11 pts), Zehner (10), König (4), Ober (3), Unter (2)

See [PLAN.md](PLAN.md) for a detailed implementation plan and game-type overview.

## License

GPL-3.0-or-later — see [LICENSE](LICENSE).
