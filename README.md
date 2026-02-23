# Dreierschnapsen

A Rust implementation of **Dreierschnapsen** (Three-player Schnapsen), a traditional
Austrian trick-taking card game played with 20 cards.

```
 ┌─────────────────────────────────────────────────────────────────────┐
 │ Dreierschnapsen │ Normales Spiel │ Trump: ♥ Herz │ Scores: 0|3|0  │
 ├──────────────────────────┬──────────────────────────────────────────┤
 │ Bot B: 🂠🂠🂠🂠           │ Bot C: 🂠🂠🂠🂠🂠                     │
 ├──────────────────────────┼──────────────────────────────────────────┤
 │  Current trick:          │  Log                                    │
 │    You    A♥             │  You: Trump: ♥ Herz                     │
 │    Bot B  10♠            │  Player 2: Pass                         │
 │                          │  Player 3: Pass                         │
 ├──────────────────────────┴──────────────────────────────────────────┤
 │ Your Hand:  J♠   Q♦   K♣   10♥   A♥   Q♠                         │
 ├────────────────────────────────────────────────────────────────────┤
 │ Play a card: ▸ J♠   Q♦   K♣   10♥   A♥   Q♠                     │
 └────────────────────────────────────────────────────────────────────┘
```

## Game Overview

Dreierschnapsen is a three-player variant of the Austrian card game Bauernschnapsen.
One player (the **Rufer**/caller) plays against the other two. The deck consists of
20 cards: 4 suits × 5 ranks.

### Card Values

| Rank  | Points |
|-------|--------|
| Ace   | 11     |
| Ten   | 10     |
| King  | 4      |
| Queen | 3      |
| Jack  | 2      |

### Game Types

The game supports 16 different game types, from the basic Normal game (1-3 points)
up to Herrenschnapser and Kontrabauernschnapser (24 points each), including:

- **Bettler** — Take no tricks (4 pts)
- **Schnapser** — Reach 66 points in ≤4 tricks (6 pts)
- **Gang** — Win all tricks, no trump (9 pts)
- **Bauernschnapser** — Win all tricks with trump (12 pts)
- **Farbringerl** — Hold all 5 cards of one suit (18 pts)
- And many more (see [DESIGN.md](DESIGN.md) for the full list)

## Project Structure

This is a Cargo workspace with three crates:

```
crates/
├── schnapsen-model/    Core data structures and game rules
│   ├── card.rs         Card, Suit, Rank types
│   ├── deck.rs         Deck with shuffle/draw
│   ├── types.rs        GameType, Actions, Tricks, Scoring
│   └── game.rs         GameState machine (deal → bid → play → score)
│
├── schnapsen-engine/   Game engine and AI
│   ├── player.rs       Player trait + RandomPlayer
│   └── engine.rs       GameRunner for match orchestration
│
└── schnapsen-cli/      Terminal interface
    ├── main.rs          Entry point with crossterm event loop
    ├── app.rs           Application state and logic
    └── ui.rs            ratatui rendering (cards, tricks, scores)
```

## Building

```bash
cargo build --workspace
```

## Running the TUI

```bash
cargo run -p schnapsen-cli
```

### Controls

| Key           | Action                        |
|---------------|-------------------------------|
| ←/→ or h/l   | Navigate options              |
| Enter / Space | Confirm selection             |
| n             | New game (after match ends)   |
| q / Esc       | Quit                          |

## Running Tests

```bash
cargo test --workspace
```

Currently includes 42 unit tests covering:
- Card points and rank orderings (standard, ace-low, ten-low, king-low)
- Deck operations (shuffle, draw, uniqueness)
- Trick resolution with trump and suit-following
- Valid move generation (Farb- und Stichzwang)
- Marriage detection (Zwanziger/Vierziger)
- Game state transitions (dealing, bidding, playing, scoring)
- Full game simulation (random AI plays complete matches)

## Game Rules

The game follows standard Austrian Dreierschnapsen rules:

1. **Dealing**: Each player gets 6 cards (3 + 3), 2 go to the talon
2. **Trump**: The Rufer chooses the trump suit from their first 3 cards
3. **Bidding**: Players announce game types; highest bid wins
4. **Talon**: The declarer may pick up the talon and discard 2 cards
5. **Play**: Trick-taking with strict suit-following and trump obligations
6. **Scoring**: First to 24 match points wins; loser gets a Bummerl

### Trick-Taking Rules (Farb- und Stichzwang)

When a card is led, the next player must:
1. Play a **higher** card of the same suit, if possible
2. Otherwise play a **lower** card of the same suit
3. Otherwise play a **trump** card
4. Otherwise play **any** card

## License

MIT
