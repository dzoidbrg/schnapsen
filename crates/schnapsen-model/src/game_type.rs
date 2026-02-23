use crate::card::RankOrdering;
use std::fmt;

/// All possible game types in Dreierschnapsen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    Normal,
    Bettler,
    Assenbettler,
    AssBettler,
    Schnapser,
    Plauderer,
    Damengang,
    Koenigsgang,
    Gang,
    Zehnergang,
    Bauernloch,
    Bauernschnapser,
    Kontraschnapser,
    Farbringerl,
    Kontrabauernschnapser,
    Herrenschnapser,
}

/// Who is allowed to announce a given game type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    /// Only the Rufer (caller) can play this
    CallerOnly,
    /// Only opponents of the Rufer can play this
    OpponentsOnly,
    /// Any player can play this
    Anyone,
}

/// The goal the announcing player must achieve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameGoal {
    /// Reach 66 card points (normal game)
    Reach66,
    /// Win zero tricks
    WinNoTricks,
    /// Win all 5 tricks
    WinAllTricks,
    /// Reach 66 in specific Schnapser ways (2 tricks + Vierziger, 3 tricks + Zwanziger, or 4 tricks)
    SchnapserReach66,
    /// Win all tricks using only cards of one non-trump suit
    WinAllTricksOneSuit,
    /// Win all tricks using only trump cards
    WinAllTricksTrump,
}

impl GameType {
    pub fn base_points(&self) -> u32 {
        match self {
            GameType::Normal => 1, // 1, 2, or 3 depending on outcome
            GameType::Bettler => 4,
            GameType::Assenbettler => 5,
            GameType::AssBettler => 5,
            GameType::Schnapser => 6,
            GameType::Plauderer => 7,
            GameType::Damengang => 7,
            GameType::Koenigsgang => 8,
            GameType::Gang => 9,
            GameType::Zehnergang => 10,
            GameType::Bauernloch => 12,
            GameType::Bauernschnapser => 12,
            GameType::Kontraschnapser => 12,
            GameType::Farbringerl => 18,
            GameType::Kontrabauernschnapser => 24,
            GameType::Herrenschnapser => 24,
        }
    }

    pub fn eligibility(&self) -> Eligibility {
        match self {
            GameType::Schnapser
            | GameType::Bauernschnapser
            | GameType::Bauernloch
            | GameType::Herrenschnapser => Eligibility::CallerOnly,

            GameType::Kontraschnapser | GameType::Kontrabauernschnapser => {
                Eligibility::OpponentsOnly
            }

            _ => Eligibility::Anyone,
        }
    }

    pub fn uses_trump(&self) -> bool {
        match self {
            GameType::Normal
            | GameType::Schnapser
            | GameType::Bauernschnapser
            | GameType::Bauernloch
            | GameType::Kontraschnapser
            | GameType::Kontrabauernschnapser
            | GameType::Herrenschnapser
            | GameType::Plauderer => true,

            GameType::Bettler
            | GameType::Assenbettler
            | GameType::AssBettler
            | GameType::Gang
            | GameType::Zehnergang
            | GameType::Koenigsgang
            | GameType::Damengang
            | GameType::Farbringerl => false,
        }
    }

    pub fn goal(&self) -> GameGoal {
        match self {
            GameType::Normal => GameGoal::Reach66,
            GameType::Bettler | GameType::Assenbettler | GameType::AssBettler => {
                GameGoal::WinNoTricks
            }
            GameType::Schnapser | GameType::Kontraschnapser => GameGoal::SchnapserReach66,
            GameType::Gang
            | GameType::Zehnergang
            | GameType::Koenigsgang
            | GameType::Damengang
            | GameType::Bauernschnapser
            | GameType::Kontrabauernschnapser
            | GameType::Bauernloch
            | GameType::Plauderer => GameGoal::WinAllTricks,
            GameType::Farbringerl => GameGoal::WinAllTricksOneSuit,
            GameType::Herrenschnapser => GameGoal::WinAllTricksTrump,
        }
    }

    pub fn rank_ordering(&self) -> RankOrdering {
        match self {
            GameType::Assenbettler | GameType::Zehnergang | GameType::Bauernloch => {
                RankOrdering::AceLow
            }
            GameType::Koenigsgang => RankOrdering::TenLow,
            GameType::Damengang => RankOrdering::KingLow,
            _ => RankOrdering::Standard,
        }
    }

    /// Whether the caller leads the first trick (as opposed to the announcer).
    pub fn caller_leads(&self) -> bool {
        match self {
            GameType::Normal | GameType::Schnapser | GameType::Kontraschnapser
            | GameType::Kontrabauernschnapser => true,
            _ => false,
        }
    }

    pub fn german_name(&self) -> &'static str {
        match self {
            GameType::Normal => "Normales Spiel (Rufer)",
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
        }
    }

    /// Returns all game types sorted by ascending point value, for bidding resolution.
    pub fn all_by_value() -> Vec<GameType> {
        let mut types = vec![
            GameType::Normal,
            GameType::Bettler,
            GameType::Assenbettler,
            GameType::AssBettler,
            GameType::Schnapser,
            GameType::Plauderer,
            GameType::Damengang,
            GameType::Koenigsgang,
            GameType::Gang,
            GameType::Zehnergang,
            GameType::Bauernloch,
            GameType::Bauernschnapser,
            GameType::Kontraschnapser,
            GameType::Farbringerl,
            GameType::Kontrabauernschnapser,
            GameType::Herrenschnapser,
        ];
        types.sort_by_key(|g| g.base_points());
        types
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}pts)", self.german_name(), self.base_points())
    }
}

/// Tracks the current doubling level (Spritzen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoublingLevel {
    None,
    Gespritzt,
    Zurueckgespritzt,
    NochmalZurueckgespritzt,
}

impl DoublingLevel {
    pub fn multiplier(&self) -> u32 {
        match self {
            DoublingLevel::None => 1,
            DoublingLevel::Gespritzt => 2,
            DoublingLevel::Zurueckgespritzt => 4,
            DoublingLevel::NochmalZurueckgespritzt => 8,
        }
    }

    pub fn can_raise(&self) -> bool {
        !matches!(self, DoublingLevel::NochmalZurueckgespritzt)
    }

    pub fn raise(self) -> Option<DoublingLevel> {
        match self {
            DoublingLevel::None => Some(DoublingLevel::Gespritzt),
            DoublingLevel::Gespritzt => Some(DoublingLevel::Zurueckgespritzt),
            DoublingLevel::Zurueckgespritzt => Some(DoublingLevel::NochmalZurueckgespritzt),
            DoublingLevel::NochmalZurueckgespritzt => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_type_points() {
        assert_eq!(GameType::Normal.base_points(), 1);
        assert_eq!(GameType::Bettler.base_points(), 4);
        assert_eq!(GameType::Gang.base_points(), 9);
        assert_eq!(GameType::Bauernschnapser.base_points(), 12);
        assert_eq!(GameType::Herrenschnapser.base_points(), 24);
    }

    #[test]
    fn doubling_multipliers() {
        assert_eq!(DoublingLevel::None.multiplier(), 1);
        assert_eq!(DoublingLevel::Gespritzt.multiplier(), 2);
        assert_eq!(DoublingLevel::Zurueckgespritzt.multiplier(), 4);
        assert_eq!(DoublingLevel::NochmalZurueckgespritzt.multiplier(), 8);
    }

    #[test]
    fn doubling_raises() {
        assert_eq!(DoublingLevel::None.raise(), Some(DoublingLevel::Gespritzt));
        assert_eq!(
            DoublingLevel::Gespritzt.raise(),
            Some(DoublingLevel::Zurueckgespritzt)
        );
        assert_eq!(
            DoublingLevel::Zurueckgespritzt.raise(),
            Some(DoublingLevel::NochmalZurueckgespritzt)
        );
        assert_eq!(DoublingLevel::NochmalZurueckgespritzt.raise(), None);
    }

    #[test]
    fn eligibility_rules() {
        assert_eq!(GameType::Schnapser.eligibility(), Eligibility::CallerOnly);
        assert_eq!(
            GameType::Kontraschnapser.eligibility(),
            Eligibility::OpponentsOnly
        );
        assert_eq!(GameType::Gang.eligibility(), Eligibility::Anyone);
    }

    #[test]
    fn trump_usage() {
        assert!(GameType::Normal.uses_trump());
        assert!(GameType::Bauernschnapser.uses_trump());
        assert!(!GameType::Bettler.uses_trump());
        assert!(!GameType::Gang.uses_trump());
        assert!(!GameType::Farbringerl.uses_trump());
    }

    #[test]
    fn all_by_value_is_sorted() {
        let types = GameType::all_by_value();
        for window in types.windows(2) {
            assert!(window[0].base_points() <= window[1].base_points());
        }
    }
}
