use crate::player::PlayerId;
use crate::variant::GameDeclaration;

/// Match scoring progression mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchScoringMode {
    /// Typical mode: first player to target points wins.
    CountUpTo(i32),
    /// Regional mode: all players start at a value and count down to zero.
    CountDownFrom(i32),
}

/// Penalties tracked at match level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BummerlMark {
    Bummerl,
    Schneider,
    Retourschneider,
}

impl BummerlMark {
    pub const fn bummerl_units(self) -> u8 {
        match self {
            BummerlMark::Bummerl => 1,
            BummerlMark::Schneider => 2,
            BummerlMark::Retourschneider => 4,
        }
    }
}

/// Multiplier chain from Spritzen/Retour/Re.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct Spritzen {
    doublings: u8,
}

impl Spritzen {
    pub const fn new() -> Self {
        Self { doublings: 0 }
    }

    pub const fn level(self) -> u8 {
        self.doublings
    }

    pub const fn multiplier(self) -> u32 {
        1u32 << self.doublings
    }

    /// Raises the multiplier one level (x2 each step).
    pub fn raise(self) -> Self {
        Self {
            doublings: self.doublings.saturating_add(1),
        }
    }

    pub const fn apply(self, base_points: u8) -> u32 {
        self.multiplier() * u32::from(base_points)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpritzenAction {
    Spritzen,
    Retour,
    Re,
}

/// Winning side in a 1-vs-2 round.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoundWinnerSide {
    Declarer,
    Defenders,
}

/// Input model for scoring a round.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundScoringInput {
    pub declarer: PlayerId,
    pub declaration: GameDeclaration,
    pub winner_side: RoundWinnerSide,
    /// Required for a normal game if fine-grained 1/2/3 scoring is needed.
    pub normal_game_defender_points: Option<u8>,
    /// Required for a normal game to detect "no trick".
    pub normal_game_defender_tricks: Option<u8>,
    pub spritzen: Spritzen,
}

/// Awarded points to a single player.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointAward {
    pub player: PlayerId,
    pub points: u32,
}

/// Point conversion for normal game outcomes.
pub fn normal_game_base_points(defender_card_points: u8, defender_tricks: u8) -> u8 {
    if defender_tricks == 0 {
        3
    } else if defender_card_points < 33 {
        2
    } else {
        1
    }
}

/// Computes 1-vs-2 awards.
pub fn score_round(input: RoundScoringInput) -> Vec<PointAward> {
    let base_points = if input.declaration == GameDeclaration::Normal {
        let defender_points = input.normal_game_defender_points.unwrap_or(33);
        let defender_tricks = input.normal_game_defender_tricks.unwrap_or(1);
        normal_game_base_points(defender_points, defender_tricks)
    } else {
        input.declaration.base_points()
    };
    let total = input.spritzen.apply(base_points);

    match input.winner_side {
        RoundWinnerSide::Declarer => vec![PointAward {
            player: input.declarer,
            points: total,
        }],
        RoundWinnerSide::Defenders => PlayerId::ALL
            .iter()
            .copied()
            .filter(|player| *player != input.declarer)
            .map(|player| PointAward {
                player,
                points: total,
            })
            .collect(),
    }
}

/// Score accumulator for a whole match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchScore {
    pub mode: MatchScoringMode,
    pub points: [i32; 3],
}

impl MatchScore {
    pub fn new(mode: MatchScoringMode) -> Self {
        let points = match mode {
            MatchScoringMode::CountUpTo(_) => [0, 0, 0],
            MatchScoringMode::CountDownFrom(start) => [start, start, start],
        };
        Self { mode, points }
    }

    pub fn apply_awards(&mut self, awards: &[PointAward]) {
        for award in awards {
            let idx = award.player.index();
            match self.mode {
                MatchScoringMode::CountUpTo(_) => {
                    self.points[idx] += award.points as i32;
                }
                MatchScoringMode::CountDownFrom(_) => {
                    self.points[idx] -= award.points as i32;
                }
            }
        }
    }

    pub fn winner(&self) -> Option<PlayerId> {
        match self.mode {
            MatchScoringMode::CountUpTo(target) => PlayerId::ALL
                .into_iter()
                .find(|p| self.points[p.index()] >= target),
            MatchScoringMode::CountDownFrom(_) => PlayerId::ALL
                .into_iter()
                .find(|p| self.points[p.index()] <= 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variant::GameDeclaration;

    #[test]
    fn normal_game_points_respect_thresholds() {
        assert_eq!(normal_game_base_points(40, 1), 1);
        assert_eq!(normal_game_base_points(32, 1), 2);
        assert_eq!(normal_game_base_points(0, 0), 3);
    }

    #[test]
    fn spritzen_doubles_points() {
        let doubled = Spritzen::new().raise();
        assert_eq!(doubled.multiplier(), 2);
        assert_eq!(doubled.apply(9), 18);
    }

    #[test]
    fn defenders_get_both_awards_when_declarer_loses() {
        let awards = score_round(RoundScoringInput {
            declarer: PlayerId::P0,
            declaration: GameDeclaration::Gang,
            winner_side: RoundWinnerSide::Defenders,
            normal_game_defender_points: None,
            normal_game_defender_tricks: None,
            spritzen: Spritzen::new(),
        });
        assert_eq!(awards.len(), 2);
        assert!(awards.iter().all(|award| award.points == 9));
    }
}
