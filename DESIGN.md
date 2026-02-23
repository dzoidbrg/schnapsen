# Dreierschnapsen - Design Document

## Overview

This project implements **Dreierschnapsen** (Three-player Schnapsen), an Austrian
trick-taking card game played with 20 cards. One player (the "caller"/Rufer) plays
against the other two.

## Architecture

The project is organized as a Cargo workspace with three crates:

```
schnapsen-model/    — Core data structures and game rules
schnapsen-engine/   — Game engine with move generation and AI
schnapsen-cli/      — Terminal UI using ratatui
```

## Game Domain Model

### Cards

- **Suits (4):** Hearts (Herz), Diamonds (Karo), Spades (Pik), Clubs (Kreuz)
- **Ranks (5 per suit):** Ace/Sau (11), Ten (10), King (4), Queen/Ober (3), Jack/Unter (2)
- **Deck:** 20 cards total

### Card Ordering

Standard ordering (highest to lowest): Ace > Ten > King > Queen > Jack

Some game types alter the ordering:
- **Assenbettler / Zehnergang:** Ace becomes lowest → Ten > King > Queen > Jack > Ace
- **Königsgang:** Ten becomes lowest → King > Queen > Jack > Ace > Ten
- **Damengang:** King becomes lowest → Queen > Jack > Ace > Ten > King

### Players & Roles

| Role       | Description                                             |
|------------|---------------------------------------------------------|
| **Geber**  | Dealer; shuffles and deals                              |
| **Rufer**  | Caller; sits left of dealer; determines trump           |
| **Third**  | Third player; sits left of caller                       |

After each hand, the Rufer becomes the Geber (roles rotate).

### Game Phases

1. **Dealing** — Each player gets 3 cards, Rufer picks trump, 2 cards go to talon, then 3 more cards each
2. **Bidding** — Players announce game types; highest bid wins
3. **Talon Exchange** — Winner picks up talon, discards 2 cards
4. **Trick Play** — Standard trick-taking with Farb- und Stichzwang
5. **Scoring** — Points awarded based on game type and outcome

### Game Types (ordered by value)

| Game Type              | Points | Trump? | Who Can Call   | Goal                           |
|------------------------|--------|--------|----------------|--------------------------------|
| Normal                 | 1/2/3  | Yes    | Rufer (default)| Reach 66 points                |
| Bettler                | 4      | No     | Anyone         | Take no tricks                 |
| Assenbettler           | 5      | No     | Anyone         | Take no tricks (Ace lowest)    |
| Ass-Bettler            | 5      | No     | Anyone         | Take no tricks (must hold Ace) |
| Schnapser              | 6      | Yes    | Rufer only     | Reach 66 in ≤4 tricks         |
| Plauderer              | 7      | Yes    | Anyone         | Special                        |
| Damengang              | 7      | No     | Anyone         | All tricks (King lowest)       |
| Königsgang             | 8      | No     | Anyone         | All tricks (Ten lowest)        |
| Gang                   | 9      | No     | Anyone         | All tricks                     |
| Zehnergang             | 10     | No     | Anyone         | All tricks (Ace lowest)        |
| Bauernloch             | 12     | Yes    | Rufer only     | All tricks (Ace lowest + trump)|
| Bauernschnapser        | 12     | Yes    | Rufer only     | All tricks                     |
| Kontraschnapser        | 12     | Yes    | Non-Rufer      | Reach 66 in ≤4 tricks         |
| Farbringerl            | 18     | No     | Anyone         | Hold all 5 of one suit         |
| Kontrabauernschnapser  | 24     | Yes    | Non-Rufer      | All tricks                     |
| Herrenschnapser        | 24     | Yes    | Rufer only     | Hold all 5 trump cards         |

### Spritzen (Doubling)

Any game can be doubled ("gespritzt"):
- Spritzen: points × 2
- Retour/Kontra: points × 4
- Re: points × 8

### Trick-Taking Rules (Farb- und Stichzwang)

When a card is led, the following player must:
1. Play a higher card of the same suit (if possible), else
2. Play a lower card of the same suit (if possible), else
3. Play a trump card to win the trick (if possible), else
4. Play any card

### Marriages (Zwanziger/Vierziger)

When a player holds King + Queen of the same suit and it's their turn to lead:
- Non-trump marriage: "Zwanziger" = 20 bonus points
- Trump marriage: "Vierziger" = 40 bonus points
- One of the two cards must be played
- If the bonus brings total to ≥66, the game ends immediately

### Scoring (Bummerl System)

- Game goal: reach 24 points total
- Loser gets a "Bummerl"
- Loser at 0 points: "Schneider" = 2 Bummerl
- Two players at 23, third at 0 and third wins: "Retourschneider" = 4 Bummerl

## Crate Responsibilities

### `schnapsen-model`
- Card, Suit, Rank enums
- Deck construction and shuffling
- Player, Hand, Trick structures
- GameType enum with all variants
- GameState machine (dealing → bidding → playing → scoring)
- Move/Action representation
- Valid move generation
- Marriage detection
- Point calculation

### `schnapsen-engine`
- Trait `Player` for pluggable AI
- `RandomPlayer` — picks a random valid move
- Game loop orchestration
- Move validation delegation to model

### `schnapsen-cli`
- ratatui-based terminal UI
- Card rendering with suit symbols (♥ ♦ ♠ ♣)
- Hand display, trick area, score board
- Bidding interface
- Human input handling
