# Architecture Notes

## 1) `schnapsen-model`

Purpose: represent Dreierschnapsen as data.

Key components:

- **Card primitives**: `Suit`, `Rank`, `Card`, `schnapsen_deck()`
- **Match entities**: `PlayerId`, turn helpers
- **Variant layer**:
  - `GameVariant` enum contains all listed variants
  - `GameVariantSpec` exposes metadata for each variant
  - `Ruleset` supports regional toggles and declaration comparisons
- **Round state layer**:
  - `GameState`, `PlayerState`, `CurrentTrick`, `ResolvedTrick`
  - `GameMove` enum for bid/talon/play/doubling actions
- **Scoring helpers**:
  - marriage values
  - doubling multiplier
  - trick winner with variant-specific rank order

The model avoids hard-coding one gameplay flow so engine implementations can vary.

## 2) `schnapsen-engine`

Purpose: execute transitions over `GameState`.

Current baseline:

- `setup_demo_normal_round` creates a playable normal-round setup.
- `legal_cards_for_player` enforces strict play obligations.
- `apply_move` currently supports `PlayCard`.
- `RandomMoveEngine` picks random valid moves.
- `round_summary` provides end-of-round aggregation for UI.

This layer is intentionally small but test-covered and extensible.

## 3) `schnapsen-cli`

Purpose: human-playable terminal experience.

Current baseline:

- `ratatui` dashboard with:
  - status header
  - player table
  - trick panel
  - legal-move panel
  - highlighted hand panel
- Human controls for legal card choice.
- Bots use random valid-move engine.

The CLI is kept separate from rules logic to simplify future frontends.
