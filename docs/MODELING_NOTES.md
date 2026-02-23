# Modeling Notes: Dreierschnapsen Structures

## Purpose

This document explains how `schnapsen-model` represents game possibilities from the provided rule set.

## Card and Ordering Model

- `Suit`: Hearts, Diamonds, Spades, Clubs
- `Rank`: Ace, Ten, King, Ober, Unter
- `Card`: pair of suit + rank
- `RankOrder`:
  - `Standard` (A > 10 > K > O > U)
  - `AceLow` (10 > K > O > U > A)
  - `Koenigsgang`
  - `Damengang`

These orderings support normal and regional declarations with altered rank strength.

## Players and Seats

- `PlayerId`: `P0`, `P1`, `P2`
- Clockwise helpers (`next_cw`, `prev_cw`) support deterministic turn progression.

## Declaration Coverage

`GameDeclaration` contains:

- `Normal`
- `Bettler`
- `Assenbettler`
- `AssBettler`
- `Schnapser`
- `Plauderer`
- `Damengang`
- `Koenigsgang`
- `Gang`
- `Zehnergang`
- `Bauernloch`
- `Bauernschnapser`
- `Kontraschnapser`
- `Farbenringerl`
- `Kontrabauernschnapser`
- `Herrenschnapser`

Each declaration maps to:

- base point value
- objective category
- trump policy (`CalledTrump` / `NoTrump`)
- rank ordering
- declarer-role constraints
- regional-only marker

## Regional Rule Toggles

`RegionalOptions` models optional declaration availability and tie-break preference:

- allow flags for regional declarations
- `gang_tiebreak` for Gang vs Zehnergang differences

## Round and Trick State

`RoundState` tracks:

- caller, declarer, declaration
- phase and active player
- trump suit
- hands, talon
- current trick + completed tricks
- trick-point accumulation
- marriage announcements
- spritzen multiplier chain

`Trick` stores ordered played cards and computes current winner under trump and rank order.

## Move and Legality Representation

- `PlayerMove` currently includes `PlayCard(Card)`
- `legal_cards_for_player` enforces:
  - follow suit
  - attempt to beat when possible
  - trump obligation where applicable

This provides a safe foundation for random-valid move selection in the engine.

## Scoring Representation

`scoring` module includes:

- normal-game 1/2/3 point conversion
- declaration base points
- spritzen multiplier application
- 1-vs-2 award distribution
- match-level accumulation (`CountUpTo` and `CountDownFrom`)
- bummerl penalty markers (`Bummerl`, `Schneider`, `Retourschneider`)
