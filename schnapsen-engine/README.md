# schnapsen-engine

Game engine for Dreierschnapsen: move validation, application, and AI.

## Features

- `valid_moves(round)` — returns all legal moves for the current player
- `GameEngine` trait — pluggable AI
- `RandomEngine` — picks a random valid move (default implementation)

## Usage

```rust
use schnapsen_engine::{GameEngine, RandomEngine, valid_moves};
use schnapsen_model::GameState;

let engine = RandomEngine::new();
let state = GameState::with_round(0, Default::default());
if let Some(mv) = engine.pick_move(&state) {
    // Apply the move...
}
```
