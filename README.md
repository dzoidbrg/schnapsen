# Dreierschnapsen Workspace

Rust workspace for modeling and playing Dreierschnapsen from terminal.

## Crates

- `schnapsen-model`
  - Core data structures for cards, players, declarations, round state, trick logic, and scoring metadata.
  - Includes representations for normal game and listed special/regional variants.
- `schnapsen-engine`
  - Minimal engine utilities:
    - legal move extraction
    - random valid move selection
    - state move application wrapper
- `schnapsen-cli`
  - Ratatui-based TUI that allows interactive terminal play.
  - Human controls one seat while bots use random valid moves.

## Project Structure

```text
.
├── Cargo.toml
├── README.md
├── docs
│   ├── IMPLEMENTATION_PLAN.md
│   └── MODELING_NOTES.md
└── crates
    ├── schnapsen-model
    ├── schnapsen-engine
    └── schnapsen-cli
```

## Quick Start

### Run tests

```bash
cargo test --workspace
```

### Run the terminal UI

```bash
cargo run -p schnapsen-cli
```

## TUI Controls

- `j` / `Down`: next legal move
- `k` / `Up`: previous legal move
- `Enter`: play selected move
- `r`: play a random legal move (human seat)
- `q`: quit

## Current Status

- Comprehensive representation layer for declarations and rule options is implemented.
- Engine currently supports card-play legality and random move choice.
- CLI is intentionally simple but fully usable for turn-by-turn terminal play.

## Notes

- This implementation emphasizes clear structure and testability over full rule-complete automation.
- Full declaration/bidding flow and advanced victory checking are suitable next steps.
