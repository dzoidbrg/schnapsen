# Dreierschnapsen Implementation Plan

## Overview

This document outlines the implementation plan for a Dreierschnapsen (Three-player Schnapsen) game in Rust, organized as a Cargo workspace with three crates.

## Game Summary (from Wikipedia rules)

- **Players**: 3 (one vs two)
- **Deck**: 20 cards (4 suits × 5 values)
- **Cards per player**: 6 (3+2 talon+3 dealing)
- **Talon**: 2 cards
- **Goal**: First to 24 points wins

---

## Crate Architecture

```
schnapsen-workspace/
├── Cargo.toml              # Workspace manifest
├── schnapsen-model/        # Data structures, no game logic
├── schnapsen-engine/       # Game logic, move validation, AI
└── schnapsen-cli/          # Terminal UI (ratatui)
```

---

## Phase 1: schnapsen-model

### 1.1 Cards & Suits

| Type | Variants |
|------|----------|
| **Suit** | Herz, Karo, Pik, Kreuz |
| **Rank** | Ass (11), Zehner (10), König (4), Ober (3), Unter (2) |
| **Card** | Suit + Rank, Display, Points |

### 1.2 Game Types (Spiele)

| Game | Points | Who can declare | Trump | Special rules |
|------|--------|-----------------|-------|---------------|
| Normales Spiel | 1-3 | Default | Yes | 66 pts or last trick |
| Bettler | 4 | Anyone | No | No tricks |
| Assenbettler | 5 | Anyone | No | Ass lowest |
| Ass-Bettler | 5 | Anyone | No | Must have Ass |
| Schnapser | 6 | Rufer only | Yes | 2/3 stich + meld variants |
| Plauderer | 7 | - | - | - |
| Damengang | 7 | Anyone | No | König lowest |
| Königsgang | 8 | Anyone | No | Zehner lowest |
| Gang | 9 | Anyone | No | All tricks |
| Zehnergang | 10 | Anyone | No | Ass lowest |
| Bauernloch | 12 | Rufer | Yes | All tricks, Ass lowest |
| Bauernschnapser | 12 | Rufer | Yes | All tricks |
| Kontraschnapser | 12 | Opponents | Yes | Ansager wins |
| Farbringerl | 18 | Anyone | No | All 5 of one suit |
| Kontrabauernschnapser | 24 | Opponents | Yes | All tricks |
| Herrenschnapser | 24 | Rufer | Yes | All trump cards |
| Flecken | ×2/×4/×8 | Doubling | - | Multiplier |

### 1.3 Card Ordering Variants

- **Normal**: Ass > Zehner > König > Ober > Unter
- **Ass lowest** (Assenbettler, Zehnergang, Bauernloch): Zehner > König > Ober > Unter > Ass
- **Zehner lowest** (Königsgang): König > Ober > Unter > Ass > Zehner
- **König lowest** (Damengang): Ober > Unter > Ass > Zehner > König

### 1.4 Game State Model

- `PlayerId` (0, 1, 2)
- `Hand` (Vec<Card>)
- `Trick` (played cards in order)
- `GamePhase`: Dealing, TrumpCalling, Bidding, Playing, Finished
- `Bid` (GameType + PlayerId)
- `RoundState` (current trick, scores, etc.)
- `GameConfig` (which regional variants allowed)

---

## Phase 2: schnapsen-engine

- `GameEngine`: orchestrates game flow
- `valid_moves(game_state) -> Vec<Move>`
- `apply_move(state, move) -> Result<GameState>`
- `RandomEngine`: picks random valid move (for now)
- Win/loss detection per game type
- Points calculation

---

## Phase 3: schnapsen-cli

- Ratatui-based TUI
- Card display (ASCII/Unicode)
- Player hand view
- Trick display
- Game state feedback
- Key bindings for play
- crossterm for terminal handling

---

## Implementation Order

1. Create workspace + empty crates
2. schnapsen-model: Card, Suit, Rank, Deck
3. schnapsen-model: GameType, GamePhase, PlayerId
4. schnapsen-model: Hand, Trick, RoundState, GameState
5. schnapsen-engine: Move validation (basic)
6. schnapsen-engine: Random move selection
7. schnapsen-cli: Basic layout, card rendering
8. schnapsen-cli: Game loop integration
9. Unit tests throughout
10. Cursor rule + README
