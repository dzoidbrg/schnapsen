# schnapsen-engine

Game engine for Dreierschnapsen: move validation, state updates, and AI.

## API

```rust
use schnapsen_engine::{get_valid_moves, apply_move, pick_random_move};

let moves = get_valid_moves(&state);
apply_move(&mut state, my_move)?;
let ai_move = pick_random_move(&state);
```

## AI

Currently implements a simple random valid-move strategy.
