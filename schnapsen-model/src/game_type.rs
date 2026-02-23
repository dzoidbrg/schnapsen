//! Game types (Spielarten) in Dreierschnapsen.
//!
//! Each game type has different rules, point values, and card orderings.

use serde::{Deserialize, Serialize};
use std::fmt;

/// All possible game types in Dreierschnapsen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    /// Normal game (Rufer) - 1-3 points depending on opponent score
    NormalesSpiel,
    /// Bettler/Fechter - no trump, make no tricks. 4 points.
    Bettler,
    /// Assenbettler/Zehnerloch - no trump, Ass lowest. 5 points.
    Assenbettler,
    /// Ass-Bettler/Ass-Fechter - no trump, must have Ace. 5 points.
    AssBettler,
    /// Schnapser/Drei-Stich - Rufer only, trump applies. 6 points.
    Schnapser,
    /// Plauderer - 7 points.
    Plauderer,
    /// Damengang - no trump, König lowest. 7 points.
    Damengang,
    /// Königsgang - no trump, Zehner lowest. 8 points.
    Koenigsgang,
    /// Gang/Land/Ring - no trump, all tricks. 9 points.
    Gang,
    /// Zehnergang - no trump, Ass lowest, all tricks. 10 points.
    Zehnergang,
    /// Bauernloch - trump, Ass lowest, all tricks. 12 points.
    Bauernloch,
    /// Bauernschnapser - trump, all tricks, Rufer only. 12 points.
    Bauernschnapser,
    /// Kontraschnapser - trump, opponent plays. 12 points.
    Kontraschnapser,
    /// Farbringerl - no trump, 5 cards of one suit. 18 points.
    Farbringerl,
    /// Kontrabauernschnapser - trump, all tricks, opponent. 24 points.
    Kontrabauernschnapser,
    /// Herrenschnapser - all trump cards. 24 points.
    Herrenschnapser,
    /// Flecken/Spritzen - multiplier (×2, ×4, ×8) on any game.
    Flecken,
}

impl GameType {
    /// Base points for this game type (before Flecken multiplier).
    pub const fn base_points(&self) -> GameTypePoints {
        match self {
            GameType::NormalesSpiel => GameTypePoints::Variable(1, 3),
            GameType::Bettler => GameTypePoints::Fixed(4),
            GameType::Assenbettler => GameTypePoints::Fixed(5),
            GameType::AssBettler => GameTypePoints::Fixed(5),
            GameType::Schnapser => GameTypePoints::Fixed(6),
            GameType::Plauderer => GameTypePoints::Fixed(7),
            GameType::Damengang => GameTypePoints::Fixed(7),
            GameType::Koenigsgang => GameTypePoints::Fixed(8),
            GameType::Gang => GameTypePoints::Fixed(9),
            GameType::Zehnergang => GameTypePoints::Fixed(10),
            GameType::Bauernloch => GameTypePoints::Fixed(12),
            GameType::Bauernschnapser => GameTypePoints::Fixed(12),
            GameType::Kontraschnapser => GameTypePoints::Fixed(12),
            GameType::Farbringerl => GameTypePoints::Fixed(18),
            GameType::Kontrabauernschnapser => GameTypePoints::Fixed(24),
            GameType::Herrenschnapser => GameTypePoints::Fixed(24),
            GameType::Flecken => GameTypePoints::Multiplier,
        }
    }

    /// Whether this game uses trump (the called suit).
    pub const fn uses_trump(&self) -> bool {
        matches!(
            self,
            GameType::NormalesSpiel
                | GameType::Schnapser
                | GameType::Kontraschnapser
                | GameType::Bauernschnapser
                | GameType::Kontrabauernschnapser
                | GameType::Herrenschnapser
                | GameType::Bauernloch
        )
    }

    /// Whether all colors are equal (no trump) in this game.
    pub const fn no_trump(&self) -> bool {
        !self.uses_trump()
    }

    /// Whether only the Rufer (dealer's left) can announce this game.
    pub const fn rufer_only(&self) -> bool {
        matches!(
            self,
            GameType::Schnapser | GameType::Bauernschnapser | GameType::Herrenschnapser | GameType::Bauernloch
        )
    }

    /// Whether only opponents of the Rufer can announce this game.
    pub const fn opponents_only(&self) -> bool {
        matches!(
            self,
            GameType::Kontraschnapser | GameType::Kontrabauernschnapser
        )
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GameType::NormalesSpiel => "Normales Spiel",
            GameType::Bettler => "Bettler",
            GameType::Assenbettler => "Assenbettler",
            GameType::AssBettler => "Ass-Bettler",
            GameType::Schnapser => "Schnapser",
            GameType::Plauderer => "Plauderer",
            GameType::Damengang => "Damengang",
            GameType::Koenigsgang => "Königsgang",
            GameType::Gang => "Gang",
            GameType::Zehnergang => "Zehnergang",
            GameType::Bauernloch => "Bauernloch",
            GameType::Bauernschnapser => "Bauernschnapser",
            GameType::Kontraschnapser => "Kontraschnapser",
            GameType::Farbringerl => "Farbringerl",
            GameType::Kontrabauernschnapser => "Kontrabauernschnapser",
            GameType::Herrenschnapser => "Herrenschnapser",
            GameType::Flecken => "Flecken",
        };
        write!(f, "{}", s)
    }
}

/// Point value for a game type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTypePoints {
    /// Fixed points (e.g., Bettler = 4)
    Fixed(u8),
    /// Variable points (min, max) - e.g., Normales Spiel 1-3
    Variable(u8, u8),
    /// Flecken is a multiplier, not a base game
    Multiplier,
}

/// Flecken (Spritzen) multiplier level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FleckenLevel {
    /// No Flecken
    #[default]
    None,
    /// Gespritzt - ×2
    Spritzen,
    /// Zurückgespritzt - ×4
    Retour,
    /// Nochmal zurückgespritzt - ×8
    Re,
}

impl FleckenLevel {
    pub const fn multiplier(&self) -> u8 {
        match self {
            FleckenLevel::None => 1,
            FleckenLevel::Spritzen => 2,
            FleckenLevel::Retour => 4,
            FleckenLevel::Re => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trump_games() {
        assert!(GameType::Bauernschnapser.uses_trump());
        assert!(GameType::NormalesSpiel.uses_trump());
        assert!(!GameType::Gang.uses_trump());
        assert!(!GameType::Bettler.uses_trump());
    }

    #[test]
    fn flecken_multiplier() {
        assert_eq!(FleckenLevel::None.multiplier(), 1);
        assert_eq!(FleckenLevel::Spritzen.multiplier(), 2);
        assert_eq!(FleckenLevel::Retour.multiplier(), 4);
        assert_eq!(FleckenLevel::Re.multiplier(), 8);
    }
}
