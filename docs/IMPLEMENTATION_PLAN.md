# Implementation Plan

This plan is intentionally concrete so each workspace layer can evolve independently.

## Phase 1: Workspace and crate boundaries

- Create a Cargo workspace with:
  - `schnapsen-model`
  - `schnapsen-engine`
  - `schnapsen-cli`
- Keep dependencies directional:
  - `model` has no workspace-internal dependencies
  - `engine` depends on `model`
  - `cli` depends on `engine` and `model`

## Phase 2: Domain model (all game possibilities)

- Represent:
  - cards, suits, ranks
  - players and turn order
  - game variants (including regional variants)
  - variant metadata:
    - base points
    - trump policy
    - rank order
    - declarer restrictions
    - objective type
  - doubling (Spritzen / Retour / Re) and multipliers
  - game state snapshots and move enums

## Phase 3: Baseline engine

- Implement strict legal-card generation for trick play:
  - follow suit
  - must overtake in suit when possible
  - otherwise trump when available
  - otherwise discard
- Implement state transition for `PlayCard`.
- Implement random valid-move selection for bot turns.

## Phase 4: Playable CLI

- Implement terminal application with `ratatui`.
- Render:
  - round status
  - player overview
  - current trick
  - legal move list
  - highlighted hand cards
- Add keyboard controls for selecting and playing legal cards.

## Phase 5: Testing and quality

- Unit tests for:
  - deck uniqueness
  - variant metadata correctness
  - trick winner logic
  - doubling progression
  - legal move constraints
  - random engine move validity
- Run workspace tests and keep code formatted.
