# schnapsen-model

Data structures representing Dreierschnapsen (three-player Schnapsen).

## Types

- **Card**, **Suit**, **Rank** — Card representation (20 cards, 4×5)
- **GameState**, **GamePhase** — Full game state
- **GameType** — All 17 game variants (Bettler, Schnapser, Gang, etc.)
- **Player** — Rufer, LeftOpponent, RightOpponent
- **Trick** — Cards played in one round

## Usage

```rust
use schnapsen_model::{create_deck, Card, Suit, Rank};

let deck = create_deck();
```
