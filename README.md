# Dreierschnapsen

A complete implementation of **Dreierschnapsen** (three-player Schnapsen) in Rust — the Austrian card game for three players with 20 cards.

## Features

- **Full game representation**: All game types (Bettler, Schnapser, Gang, Bauernschnapser, etc.) with correct card orderings
- **Rule-aware engine**: Valid move generation (Farbzwang, Stichzwang) and state transitions
- **Playable TUI**: Terminal interface using [ratatui](https://ratatui.rs) — play against AI opponents

## Quick Start

```bash
cargo run -p schnapsen-cli
```

### Controls

| Key | Action |
|-----|--------|
| ← → | Select card |
| 1-6 | Jump to card by number |
| Enter | Play selected card |
| n | New round |
| q | Quit |

## Project Structure

```
├── schnapsen-model/   # Data structures (cards, game types, state)
├── schnapsen-engine/  # Game logic & random AI
├── schnapsen-cli/     # Terminal UI (ratatui)
└── docs/              # Implementation docs
```

## Game Rules (Summary)

- **20 cards**: 4 suits × 5 ranks (Ass, 10, König, Ober, Unter)
- **One vs two**: Declarer plays against both opponents
- **Goal**: 24 points to win (from trick points + game bonuses)
- **Trump**: Called by Rufer (player left of dealer) from first 3 cards
- **Normal game**: 66 points or last trick wins; 1-3 points depending on opponent score

See [docs/IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md) for detailed game type specifications.

## Development

```bash
# Build
cargo build

# Test
cargo test

# Run TUI
cargo run -p schnapsen-cli
```

## License

GPL-3.0-or-later — see [LICENSE](LICENSE).
