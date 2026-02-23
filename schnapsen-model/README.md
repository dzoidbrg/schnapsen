# schnapsen-model

Data structures for representing the **Dreierschnapsen** (Three-player Schnapsen) card game.

## Contents

- **Card types**: `Suit`, `Rank`, `Card` with full 20-card deck
- **Game types**: All 16+ Spiel variants (Normales Spiel, Bettler, Gang, Bauernschnapser, etc.)
- **Game state**: `GamePhase`, `RoundState`, `GameState`, `Hand`, `Trick`, `Bid`
- **Configuration**: `GameConfig` for regional rule variants

## Usage

```rust
use schnapsen_model::{Card, Suit, Rank, GameType, GameState, GameConfig};

// Full deck
let deck = Card::full_deck();
assert_eq!(deck.len(), 20);

// Game types
let info = GameType::Gang.info();
assert_eq!(info.points, 9);
assert!(info.uses_trump == false);
```
