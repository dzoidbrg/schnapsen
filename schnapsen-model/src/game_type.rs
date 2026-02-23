//! Game types (Spiele) and their metadata for Dreierschnapsen.

use std::fmt;

/// Card ordering variant for special games.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardOrdering {
    /// Normal: Ass > Zehner > König > Ober > Unter
    Normal,
    /// Ass lowest: Zehner > König > Ober > Unter > Ass
    AssLowest,
    /// Zehner lowest: König > Ober > Unter > Ass > Zehner
    ZehnerLowest,
    /// König lowest: Ober > Unter > Ass > Zehner > König
    KoenigLowest,
}

/// All declarable game variants in Dreierschnapsen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    /// Normales Spiel (1–3 points depending on opponent score)
    NormalesSpiel,
    /// Bettler / Fechter – no tricks, no trump (4 pts)
    Bettler,
    /// Assenbettler / Zehnerloch – no tricks, Ass lowest (5 pts)
    Assenbettler,
    /// Ass-Bettler / Ass-Fechter – no tricks, must have Ass (5 pts)
    AssBettler,
    /// Schnapser / Drei-Stich – Rufer only (6 pts)
    Schnapser,
    /// Plauderer (7 pts)
    Plauderer,
    /// Damengang – all tricks, König lowest (7 pts)
    Damengang,
    /// Königsgang – all tricks, Zehner lowest (8 pts)
    Koenigsgang,
    /// Gang / Ring / Land – all tricks (9 pts)
    Gang,
    /// Zehnergang – all tricks, Ass lowest (10 pts)
    Zehnergang,
    /// Bauernloch – all tricks, trump, Ass lowest (12 pts)
    Bauernloch,
    /// Bauernschnapser – all tricks, trump, Rufer only (12 pts)
    Bauernschnapser,
    /// Kontraschnapser – Ansager wins with special rules (12 pts)
    Kontraschnapser,
    /// Farbringerl / Farbenjodler – all 5 of one suit (18 pts)
    Farbringerl,
    /// Kontrabauernschnapser – opponents, all tricks (24 pts)
    Kontrabauernschnapser,
    /// Herrenschnapser – all trump cards (24 pts)
    Herrenschnapser,
}

/// Metadata for a game type.
#[derive(Debug, Clone)]
pub struct GameTypeInfo {
    /// Base points for winning this game.
    pub points: u8,
    /// Whether trump applies.
    pub uses_trump: bool,
    /// Whether only the Rufer can declare.
    pub rufer_only: bool,
    /// Whether only opponents of the Rufer can declare.
    pub opponents_only: bool,
    /// Card ordering variant (for games like Assenbettler).
    pub card_ordering: CardOrdering,
    /// Goal: no tricks, all tricks, or 66 points.
    pub goal: GameGoal,
}

/// What the declarer must achieve to win.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameGoal {
    /// Reach 66 points (or last trick); normal scoring (1–3 pts).
    SixtySix,
    /// Make no tricks (Bettler variants).
    NoTricks,
    /// Make all tricks.
    AllTricks,
    /// Schnapser/Kontraschnapser: 2 stich+Vierziger or 3 stich+Zwanziger or 4 stich.
    SchnapserVariants,
}

impl GameType {
    /// Returns metadata for this game type.
    pub fn info(self) -> GameTypeInfo {
        use GameGoal::*;

        match self {
            GameType::NormalesSpiel => GameTypeInfo {
                points: 1, // 1–3 depending on opponent
                uses_trump: true,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: SixtySix,
            },
            GameType::Bettler => GameTypeInfo {
                points: 4,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: NoTricks,
            },
            GameType::Assenbettler => GameTypeInfo {
                points: 5,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::AssLowest,
                goal: NoTricks,
            },
            GameType::AssBettler => GameTypeInfo {
                points: 5,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: NoTricks,
            },
            GameType::Schnapser => GameTypeInfo {
                points: 6,
                uses_trump: true,
                rufer_only: true,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: SchnapserVariants,
            },
            GameType::Plauderer => GameTypeInfo {
                points: 7,
                uses_trump: true,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: SixtySix,
            },
            GameType::Damengang => GameTypeInfo {
                points: 7,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::KoenigLowest,
                goal: AllTricks,
            },
            GameType::Koenigsgang => GameTypeInfo {
                points: 8,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::ZehnerLowest,
                goal: AllTricks,
            },
            GameType::Gang => GameTypeInfo {
                points: 9,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: AllTricks,
            },
            GameType::Zehnergang => GameTypeInfo {
                points: 10,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::AssLowest,
                goal: AllTricks,
            },
            GameType::Bauernloch => GameTypeInfo {
                points: 12,
                uses_trump: true,
                rufer_only: true,
                opponents_only: false,
                card_ordering: CardOrdering::AssLowest,
                goal: AllTricks,
            },
            GameType::Bauernschnapser => GameTypeInfo {
                points: 12,
                uses_trump: true,
                rufer_only: true,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: AllTricks,
            },
            GameType::Kontraschnapser => GameTypeInfo {
                points: 12,
                uses_trump: true,
                rufer_only: false,
                opponents_only: true,
                card_ordering: CardOrdering::Normal,
                goal: SchnapserVariants,
            },
            GameType::Farbringerl => GameTypeInfo {
                points: 18,
                uses_trump: false,
                rufer_only: false,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: AllTricks,
            },
            GameType::Kontrabauernschnapser => GameTypeInfo {
                points: 24,
                uses_trump: true,
                rufer_only: false,
                opponents_only: true,
                card_ordering: CardOrdering::Normal,
                goal: AllTricks,
            },
            GameType::Herrenschnapser => GameTypeInfo {
                points: 24,
                uses_trump: true,
                rufer_only: true,
                opponents_only: false,
                card_ordering: CardOrdering::Normal,
                goal: AllTricks,
            },
        }
    }

    /// Base point value (before Flecken multiplier).
    pub fn base_points(self) -> u8 {
        self.info().points
    }

    /// All standard game types in approximate order of value.
    pub fn all_standard() -> Vec<GameType> {
        vec![
            GameType::NormalesSpiel,
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
        ]
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
        };
        write!(f, "{}", s)
    }
}
