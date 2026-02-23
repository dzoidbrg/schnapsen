# Dreierschnapsen Implementation Plan

## Overview

Dreierschnapsen (Three-player Schnapsen) is an Austrian trick-taking card game for 3 players
using a 20-card deck. This project implements the game as a Rust workspace with three crates.

## Game Domain Summary

### Cards
- **4 Suits**: Herz (Hearts), Karo (Diamonds), Pik (Spades), Kreuz (Clubs)
- **5 Ranks** per suit with point values:
  - Ass (Ace): 11 points
  - Zehner (Ten): 10 points
  - König (King): 4 points
  - Ober (Queen): 3 points
  - Unter (Jack): 2 points
- **Total**: 20 cards, 120 points in the deck

### Players & Roles
- **3 players** seated in clockwise order
- **Geber** (Dealer): shuffles and deals
- **Rufer** (Caller): sits left of dealer, calls trump suit
- **Third player**: sits left of Rufer

### Deal Sequence
1. Each player receives 3 cards
2. Rufer selects trump suit from first 3 cards
3. 2 cards form the **Talon** (face-down)
4. Each player receives 3 more cards (total 6 per player)
5. If Rufer cannot pick trump from first 3, they flip one of the second 3 cards

### Game Types (ordered by point value)

| Game Type | Points | Who Can Play | Trump? | Goal |
|-----------|--------|-------------|--------|------|
| Normal (Rufer) | 1/2/3 | Rufer (default) | Yes | Reach 66 points |
| Bettler | 4 | Anyone | No | Win zero tricks |
| Assenbettler | 5 | Anyone | No | Win zero tricks (Ace is lowest) |
| Ass-Bettler | 5 | Anyone (must hold Ace) | No | Win zero tricks |
| Schnapser | 6 | Rufer only | Yes | Reach 66 in specific ways |
| Plauderer | 7 | Anyone | Yes | (Regional) |
| Damengang | 7 | Anyone | No | Win all tricks (König lowest) |
| Königsgang | 8 | Anyone | No | Win all tricks (Zehner lowest) |
| Gang | 9 | Anyone | No | Win all tricks |
| Zehnergang | 10 | Anyone | No | Win all tricks (Ass lowest) |
| Bauernloch | 12 | Rufer only | Yes | Win all tricks (Ass lowest) |
| Bauernschnapser | 12 | Rufer only | Yes | Win all tricks |
| Kontraschnapser | 12 | Opponents only | Yes | Reach 66 in specific ways |
| Farbringerl | 18 | Anyone (all 5 of a suit) | No | Win all tricks with one suit |
| Kontrabauernschnapser | 24 | Opponents only | Yes | Win all tricks |
| Herrenschnapser | 24 | Rufer only (all 5 trump) | Yes | Win all tricks with trump |

### Spritzen (Doubling)
- Gespritzt: points x 2
- Zurückgespritzt: points x 4
- Nochmal zurückgespritzt: points x 8

### Card Ordering Variants
- **Normal**: Ass > Zehner > König > Ober > Unter
- **Assenbettler/Zehnergang/Bauernloch**: Zehner > König > Ober > Unter > Ass
- **Königsgang**: König > Ober > Unter > Ass > Zehner
- **Damengang**: Ober > Unter > Ass > Zehner > König

### Trick Rules (Farb- und Stichzwang)
1. Must play higher card of same suit if possible
2. Otherwise, play lower card of same suit
3. Otherwise, trump if possible
4. Otherwise, play any card

### Scoring
- Goal: reach 24 game points
- Normal game: 1 pt (opponent >= 33), 2 pts (opponent < 33), 3 pts (opponent no tricks)
- Bummerl for the loser of a full game
- Schneider (0 points at loss): 2 Bummerl
- Retourschneider: 4 Bummerl

### Announcements
- **Zwanziger** (20 points): König + Ober of same non-trump suit
- **Vierziger** (40 points): König + Ober of trump suit

## Crate Architecture

```
dreierschnapsen/
├── Cargo.toml              (workspace root)
├── README.md
├── docs/
│   └── PLAN.md
├── crates/
│   ├── schnapsen-model/    (data structures & game rules)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── card.rs     (Card, Suit, Rank)
│   │   │   ├── game_type.rs (GameType enum, all variants)
│   │   │   ├── player.rs   (Player, Role, Hand)
│   │   │   ├── state.rs    (GameState, phases, tricks)
│   │   │   ├── action.rs   (PlayerAction enum)
│   │   │   ├── scoring.rs  (point calculations)
│   │   │   └── deck.rs     (Deck, shuffle, deal)
│   │   └── Cargo.toml
│   ├── schnapsen-engine/   (game engine with move validation)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── engine.rs   (Engine trait)
│   │   │   ├── random.rs   (RandomEngine)
│   │   │   └── validator.rs (move legality checking)
│   │   └── Cargo.toml
│   └── schnapsen-cli/      (ratatui TUI client)
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs      (application state)
│       │   ├── ui.rs       (rendering)
│       │   └── input.rs    (keyboard handling)
│       └── Cargo.toml
└── .cursor/
    └── rules/
        └── dreierschnapsen.mdc
```

## Implementation Order
1. `schnapsen-model` - Foundation types and game rules
2. `schnapsen-engine` - Game logic, validation, random AI
3. `schnapsen-cli` - Terminal UI using ratatui
4. Tests for model and engine
5. Integration testing via CLI
