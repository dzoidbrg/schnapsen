# Dreierschnapsen Workspace

Rust workspace for modeling and playing **Dreierschnapsen** (3-player Schnapsen).

## Workspace structure

- `schnapsen-model`  
  Core domain model for cards, variants, scoring metadata, rulesets, game state, and move types.
- `schnapsen-engine`  
  Baseline engine with legal move generation and a simple random valid-move picker.
- `schnapsen-cli`  
  Terminal UI (TUI) based on `ratatui`, wired to the engine for an interactive playable round.

## Current implementation scope

This iteration focuses on:

1. Representing game possibilities as strong Rust data structures.
2. Providing an initial playable engine loop (normal-round baseline).
3. Offering a usable terminal UX for card visualization and interaction.

Full strategic bidding and all variant-specific win conditions are intentionally staged for future iterations, while the model already contains variant metadata for all listed games.

## Quickstart

```bash
cargo test --workspace
cargo run -p schnapsen-cli
```

## TUI controls

- `Left` / `Right` (or `h` / `l`): select legal card
- `Enter` (or `Space`): play selected card
- `r`: restart round
- `q`: quit

## Design goals

- Keep model types explicit and composable.
- Keep engine rules deterministic and testable.
- Keep CLI ergonomic and easy to evolve.

See `docs/IMPLEMENTATION_PLAN.md` and `docs/ARCHITECTURE.md` for details.
