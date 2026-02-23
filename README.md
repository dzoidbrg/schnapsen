# Dreierschnapsen

A Rust implementation of **Dreierschnapsen** (Three-player Schnapsen), a traditional Austrian trick-taking card game. Play against two AI opponents in a beautiful terminal UI.

## Overview

Dreierschnapsen is played with a 20-card deck (4 suits × 5 ranks) by 3 players. The **Rufer** (caller) selects the trump suit, players may announce special game types during bidding, and tricks are played with strict suit-following and trick-taking obligations (*Farb- und Stichzwang*). First player to reach 24 game points wins.

## Project Structure

This is a Cargo workspace with three crates:

```
crates/
├── schnapsen-model/    # Core data structures and game rules
├── schnapsen-engine/   # Game engine, move validation, AI
└── schnapsen-cli/      # Terminal UI (ratatui)
```

### schnapsen-model

Pure data structures representing all aspects of the game:

- **Cards**: 4 suits (Herz/Karo/Pik/Kreuz) × 5 ranks (Ass/Zehner/König/Ober/Unter)
- **16 Game Types**: Normal, Bettler, Schnapser, Gang, Bauernschnapser, Farbringerl, Herrenschnapser, and more
- **Rank Orderings**: Standard, Ace-low (Zehnergang), Ten-low (Königsgang), King-low (Damengang)
- **Spritzen (Doubling)**: Up to 8× multiplier through successive doubles
- **Scoring**: Bummerl, Schneider, Retourschneider
- **Full Game State**: Deal phases, trick resolution, marriage announcements

### schnapsen-engine

Game logic built on the model:

- **Move Validation**: Enforces Farb- und Stichzwang (must follow suit, must beat, must trump)
- **`GameEngine`**: Manages deal lifecycle — dealing, trump calling, bidding, talon exchange, trick play, scoring
- **`MoveSelector` trait**: Pluggable AI interface
- **`RandomSelector`**: Simple AI that picks random valid moves

### schnapsen-cli

Terminal user interface using [ratatui](https://ratatui.rs):

- Colored card display with suit symbols (♥ ♦ ♠ ♣)
- Trump selection, bidding, talon exchange, and trick-taking screens
- Valid moves highlighted, invalid cards dimmed
- Score tracking across multiple deals
- Two AI opponents using random strategy

## Quick Start

```bash
# Build everything
cargo build --release

# Run the TUI game
cargo run -p schnapsen-cli --release

# Run all tests
cargo test --workspace
```

## Controls

| Screen | Key | Action |
|--------|-----|--------|
| All | `q` | Quit |
| Trump Selection | `←` `→` | Select suit |
| Trump Selection | `Enter` | Confirm |
| Bidding | `↑` `↓` | Browse game types |
| Bidding | `Enter` | Announce game |
| Bidding | `p` | Pass |
| Talon Exchange | `t` | Pick up talon |
| Talon Exchange | `←` `→` | Navigate cards |
| Talon Exchange | `Space` | Mark card for discard |
| Talon Exchange | `Enter` | Confirm discard |
| Playing | `←` `→` | Select card |
| Playing | `Enter` | Play card |
| Results | `Enter` | Continue |

Vim-style keys (`h`/`j`/`k`/`l`) also work for navigation.

## Game Types

| Game | Points | Description |
|------|--------|-------------|
| Normal (Rufer) | 1-3 | Reach 66 card points |
| Bettler | 4 | Win zero tricks (no trump) |
| Assenbettler | 5 | Win zero tricks, Ace is lowest |
| Ass-Bettler | 5 | Win zero tricks, must hold an Ace |
| Schnapser | 6 | Reach 66 in ≤4 tricks (caller only) |
| Plauderer | 7 | Win all tricks |
| Damengang | 7 | Win all tricks, König is lowest |
| Königsgang | 8 | Win all tricks, Zehner is lowest |
| Gang | 9 | Win all tricks (no trump) |
| Zehnergang | 10 | Win all tricks, Ass is lowest |
| Bauernloch | 12 | Win all tricks, Ass low + trump |
| Bauernschnapser | 12 | Win all tricks with trump |
| Kontraschnapser | 12 | Opponents' Schnapser variant |
| Farbringerl | 18 | Win all tricks with one complete suit |
| Kontrabauernschnapser | 24 | Opponents win all tricks with trump |
| Herrenschnapser | 24 | Win all tricks with trump suit cards |

## Card Values

| Rank | German | Points |
|------|--------|--------|
| Ace | Ass | 11 |
| Ten | Zehner | 10 |
| King | König | 4 |
| Queen | Ober | 3 |
| Jack | Unter | 2 |

Total deck: 120 points (4 suits × 30 points per suit)

## Requirements

- Rust 1.82+ (tested with 1.93)
- Terminal with Unicode support (for card symbols)

## License

MIT
