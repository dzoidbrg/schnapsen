# Dreierschnapsen Implementation Plan

## Overview

This document outlines the implementation of a complete Dreierschnapsen (Three-player Schnapsen) card game in Rust. The game is based on the rules from German Wikipedia.

## Architecture

```
schnapsen-workspace/
├── Cargo.toml          # Workspace manifest
├── schnapsen-model/    # Data structures & game representation
├── schnapsen-engine/   # Game logic & AI
├── schnapsen-cli/      # Terminal UI (ratatui)
└── docs/               # Documentation
```

## 1. schnapsen-model Crate

### 1.1 Card Representation

| Structure | Description |
|-----------|-------------|
| `Suit` | Herz, Karo, Pik, Kreuz (Hearts, Diamonds, Spades, Clubs) |
| `Rank` | Ass (11pts), Zehner (10pts), König (4pts), Ober (3pts), Unter (2pts) |
| `Card` | Combination of Suit + Rank |

### 1.2 Game Types (Spielarten)

| Game Type | Points | Who Can Announce | Special Rules |
|-----------|--------|------------------|---------------|
| Normales Spiel | 1-3 | Default | Trump applies, 66 pts or last trick |
| Bettler | 4 | Anyone | No trump, no tricks |
| Assenbettler | 5 | Anyone | No trump, Ass=lowest |
| Ass-Bettler | 5 | Anyone | No trump, must have Ace |
| Schnapser | 6 | Rufer only | Trump, specific trick/point combos |
| Plauderer | 7 | - | - |
| Damengang | 7 | Anyone | No trump, König=lowest |
| Königsgang | 8 | Anyone | No trump, Zehner=lowest |
| Gang | 9 | Anyone | No trump, all tricks |
| Zehnergang | 10 | Anyone | No trump, Ass=lowest, all tricks |
| Bauernloch | 12 | Rufer only | Trump, Ass=lowest, all tricks |
| Bauernschnapser | 12 | Rufer only | Trump, all tricks |
| Kontraschnapser | 12 | Opponents | Trump, specific combos |
| Farbringerl | 18 | Anyone | No trump, 5 cards of same suit |
| Kontrabauernschnapser | 24 | Opponents | Trump, all tricks |
| Herrenschnapser | 24 | Rufer only | All trump cards |
| Flecken | multiplier | Any | ×2, ×4, ×8 on any game |

### 1.3 Game State Structures

- `GamePhase`: Dealing, TrumpCalling, Bidding, Playing, Scoring
- `Player`: Enum with positions (Rufer, LeftOpponent, RightOpponent)
- `Trick`: Cards played in one round
- `Hand`: Player's cards
- `GameState`: Full game state at any moment
- `GameConfig`: Regional variants (which games allowed, etc.)

### 1.4 Card Order Variations

Different games use different rank orderings:
- **Normal**: Ass > Zehner > König > Ober > Unter
- **Assenbettler/Zehnergang**: Zehner > König > Ober > Unter > Ass
- **Königsgang**: König > Ober > Unter > Ass > Zehner
- **Damengang**: Ober > Unter > Ass > Zehner > König

## 2. schnapsen-engine Crate

### 2.1 Core Responsibilities

- Validate moves against game rules (Farbzwang, Stichzwang)
- Apply moves to game state
- Determine winner of each trick
- Calculate game scores
- Check win/loss conditions

### 2.2 Initial AI Strategy

Simple random valid move selection:
- Get all valid moves for current player
- Pick one uniformly at random

### 2.3 Public API

```rust
pub fn get_valid_moves(state: &GameState) -> Vec<Move>;
pub fn apply_move(state: &mut GameState, mv: Move) -> Result<(), GameError>;
pub fn pick_random_move(state: &GameState) -> Option<Move>;
```

## 3. schnapsen-cli Crate

### 3.1 Technology

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal backend

### 3.2 UI Layout

```
┌─────────────────────────────────────────────────┐
│  Dreierschnapsen - [Game Type]                   │
├─────────────────────────────────────────────────┤
│  Opponents' tricks: [trick display]             │
│  Current trick: [cards]                          │
├─────────────────────────────────────────────────┤
│  Your hand: [card display - selectable]          │
│  Talon: [?] [?]  Trump: ♠                        │
├─────────────────────────────────────────────────┤
│  Scores: Rufer 12 | Opp1 8 | Opp2 4             │
│  [q]uit  [h]elp                                 │
└─────────────────────────────────────────────────┘
```

### 3.3 Card Display

- ASCII/Unicode card representation
- Suit symbols: ♥ ♦ ♠ ♣
- Rank symbols: A, 10, K, O, U (or regional)
- Highlight selectable cards
- Show current trick clearly

### 3.4 Interaction

- Arrow keys / number keys to select card
- Enter to play selected card
- AI opponents make automatic moves (random for now)

## 4. Implementation Order

1. Create Cargo workspace
2. Implement schnapsen-model (all structures)
3. Implement schnapsen-engine (validation + random AI)
4. Implement schnapsen-cli (basic TUI)
5. Unit tests throughout
6. Integration testing
7. Create Cursor rule

## 5. Testing Strategy

- **schnapsen-model**: Card ordering, game type logic, state transitions
- **schnapsen-engine**: Move validation, trick resolution, scoring
- **schnapsen-cli**: (Limited - TUI tested manually)

## 6. Regional Variants

Configurable options:
- Which special games are allowed (Bettler, Zehnergang, etc.)
- Gang vs Zehnergang priority when both announced
- Rufer bidding order (before/after others)
