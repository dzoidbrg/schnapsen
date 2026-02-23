//! Player representation in the three-player game.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The three players in Dreierschnapsen.
/// One player (the declarer) plays against the other two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Player {
    /// The Rufer (caller) - sits left of the dealer, calls trump, plays first in normal game
    Rufer,
    /// The player to the left of the Rufer (clockwise)
    LeftOpponent,
    /// The player to the right of the Rufer (the dealer)
    RightOpponent,
}

impl Player {
    /// All players in turn order (Rufer plays first in normal game).
    pub const fn all() -> [Player; 3] {
        [Player::Rufer, Player::LeftOpponent, Player::RightOpponent]
    }

    /// The next player in turn order (clockwise).
    pub const fn next(&self) -> Player {
        match self {
            Player::Rufer => Player::LeftOpponent,
            Player::LeftOpponent => Player::RightOpponent,
            Player::RightOpponent => Player::Rufer,
        }
    }

    /// The previous player in turn order (counter-clockwise).
    pub const fn prev(&self) -> Player {
        match self {
            Player::Rufer => Player::RightOpponent,
            Player::LeftOpponent => Player::Rufer,
            Player::RightOpponent => Player::LeftOpponent,
        }
    }

    /// Index for array access (0, 1, 2).
    pub const fn index(&self) -> usize {
        match self {
            Player::Rufer => 0,
            Player::LeftOpponent => 1,
            Player::RightOpponent => 2,
        }
    }

    /// Whether this player is an opponent of the declarer.
    pub fn is_opponent_of(&self, declarer: Player) -> bool {
        *self != declarer
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Player::Rufer => "Rufer",
            Player::LeftOpponent => "Linker Gegner",
            Player::RightOpponent => "Rechter Gegner",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_order() {
        assert_eq!(Player::Rufer.next(), Player::LeftOpponent);
        assert_eq!(Player::LeftOpponent.next(), Player::RightOpponent);
        assert_eq!(Player::RightOpponent.next(), Player::Rufer);
    }

    #[test]
    fn indices() {
        assert_eq!(Player::Rufer.index(), 0);
        assert_eq!(Player::LeftOpponent.index(), 1);
        assert_eq!(Player::RightOpponent.index(), 2);
    }
}
