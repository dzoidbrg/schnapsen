# Dreierschnapsen Workspace Implementation Plan

## Goal

Create a Rust Cargo workspace with three crates:

1. `schnapsen-model` - complete domain/data structures for representing Dreierschnapsen game states and rule variations.
2. `schnapsen-engine` - a minimal engine that computes valid moves and picks one random legal move.
3. `schnapsen-cli` - a terminal UI (TUI) using `ratatui` for playable interaction and clear card visualization.

## Architecture

### Workspace Layout

```text
/workspace
  Cargo.toml                      # workspace manifest
  README.md
  docs/
    IMPLEMENTATION_PLAN.md
  .cursor/
    rules/
      schnapsen-project.mdc
  crates/
    schnapsen-model/
    schnapsen-engine/
    schnapsen-cli/
```

### Dependency Graph

- `schnapsen-model`: no internal dependencies.
- `schnapsen-engine`: depends on `schnapsen-model` + `rand`.
- `schnapsen-cli`: depends on `schnapsen-model`, `schnapsen-engine`, `ratatui`, and terminal helper crates.

## Modeling Strategy (`schnapsen-model`)

Represent all relevant game possibilities as typed data:

1. **Card model**
   - `Suit`, `Rank`, `Card`.
   - Rank ordering systems to support standard and regional variants.

2. **Table actors and turn model**
   - `PlayerId` (3 players), seat order helpers, active player, trick leader.

3. **Game variants / declarations**
   - Normal game and all listed special games:
     - Bettler, Assenbettler, Ass-Bettler
     - Schnapser, Kontraschnapser
     - Gang, Zehnergang, Koenigsgang, Damengang
     - Bauernschnapser, Kontrabauernschnapser, Bauernloch
     - Farbringerl, Herrenschnapser
   - Include metadata (base points, trump usage, objective style).

4. **Rules & options**
   - Trump state (active / ignored).
   - Talon handling and draw/discard decisions.
   - Regional enable/disable toggles for optional game types.
   - Tie-breaking preferences where documented as regional differences.

5. **State containers**
   - `DealState`, `HandState`, `Trick`, `RoundState`, `MatchScore`.
   - Announcements (`Zwanziger`, `Vierziger`) and multiplier chain (`Spritzen` / retour levels).

6. **Scoring representation**
   - Base points per declaration.
   - Outcome (declarer won/lost).
   - Multiplier application.
   - Result distribution for 1v2 outcome semantics.

## Engine Strategy (`schnapsen-engine`)

1. Build move and legality primitives:
   - `Move` enum (play card, announce marriage, etc. minimal subset for now).
   - `valid_moves(state, player)` using color-follow and trick-taking constraints.

2. Random legal move selection:
   - `pick_random_valid_move` using RNG.
   - Return `None` if no move is legal.
   - Provide deterministic path in tests with seeded RNG.

3. Keep implementation intentionally small:
   - Focus on correctness of legal-card filtering and random selection.
   - Defer full game progression automation to future iterations.

## CLI Strategy (`schnapsen-cli`)

1. TUI with `ratatui`:
   - Header with round metadata (trump, mode, active player, score).
   - Hand view as readable card list.
   - Current trick panel.
   - Valid move panel with keyboard selection.

2. Controls:
   - Arrow keys / `j` `k` to navigate.
   - `Enter` to execute selected move.
   - `r` random move.
   - `q` quit.

3. Usability:
   - Keep card rendering clear and compact.
   - Show help/footer line with controls.

## Testing Plan

1. `schnapsen-model` tests:
   - Card ordering and deck invariants.
   - Variant metadata consistency.
   - Scoring multiplier correctness.

2. `schnapsen-engine` tests:
   - Valid move generation respects follow-suit/trump constraints.
   - Random-picked move is always from valid-move set.

3. `schnapsen-cli` smoke tests:
   - Minimal non-interactive construction tests for view/model helpers.

4. Workspace verification:
   - `cargo test --workspace`.

## Delivery Steps

1. Initialize workspace and crates.
2. Implement model crate with docs and tests.
3. Implement engine crate with random-valid-move logic and tests.
4. Implement CLI crate with playable TUI.
5. Add README and Cursor rule file.
6. Commit + push.
7. Run tests, fix issues, and commit + push any corrections.
