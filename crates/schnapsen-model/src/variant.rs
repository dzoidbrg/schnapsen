use crate::card::RankOrder;

/// Declared game types in Dreierschnapsen and common regional variants.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GameDeclaration {
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
    Farbenringerl,
    Kontrabauernschnapser,
    Herrenschnapser,
}

impl GameDeclaration {
    pub const ALL: [GameDeclaration; 16] = [
        GameDeclaration::Normal,
        GameDeclaration::Bettler,
        GameDeclaration::Assenbettler,
        GameDeclaration::AssBettler,
        GameDeclaration::Schnapser,
        GameDeclaration::Plauderer,
        GameDeclaration::Damengang,
        GameDeclaration::Koenigsgang,
        GameDeclaration::Gang,
        GameDeclaration::Zehnergang,
        GameDeclaration::Bauernloch,
        GameDeclaration::Bauernschnapser,
        GameDeclaration::Kontraschnapser,
        GameDeclaration::Farbenringerl,
        GameDeclaration::Kontrabauernschnapser,
        GameDeclaration::Herrenschnapser,
    ];

    pub fn info(self) -> DeclarationInfo {
        match self {
            GameDeclaration::Normal => DeclarationInfo {
                declaration: self,
                base_points: 1,
                objective: RoundObjective::Reach66OrLastTrick,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: false,
            },
            GameDeclaration::Bettler => DeclarationInfo {
                declaration: self,
                base_points: 4,
                objective: RoundObjective::LoseEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: false,
            },
            GameDeclaration::Assenbettler => DeclarationInfo {
                declaration: self,
                base_points: 5,
                objective: RoundObjective::LoseEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::AceLow,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::AssBettler => DeclarationInfo {
                declaration: self,
                base_points: 5,
                objective: RoundObjective::LoseEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::Schnapser => DeclarationInfo {
                declaration: self,
                base_points: 6,
                objective: RoundObjective::SchnapserObjective,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::CallerOnly,
                regional_only: false,
            },
            GameDeclaration::Plauderer => DeclarationInfo {
                declaration: self,
                base_points: 7,
                objective: RoundObjective::VariantSpecific,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::Damengang => DeclarationInfo {
                declaration: self,
                base_points: 7,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Damengang,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::Koenigsgang => DeclarationInfo {
                declaration: self,
                base_points: 8,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Koenigsgang,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::Gang => DeclarationInfo {
                declaration: self,
                base_points: 9,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: false,
            },
            GameDeclaration::Zehnergang => DeclarationInfo {
                declaration: self,
                base_points: 10,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::AceLow,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: true,
            },
            GameDeclaration::Bauernloch => DeclarationInfo {
                declaration: self,
                base_points: 12,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::AceLow,
                declarer_constraint: DeclarerConstraint::CallerOnly,
                regional_only: true,
            },
            GameDeclaration::Bauernschnapser => DeclarationInfo {
                declaration: self,
                base_points: 12,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::CallerOnly,
                regional_only: false,
            },
            GameDeclaration::Kontraschnapser => DeclarationInfo {
                declaration: self,
                base_points: 12,
                objective: RoundObjective::SchnapserObjective,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::DefenderOnly,
                regional_only: false,
            },
            GameDeclaration::Farbenringerl => DeclarationInfo {
                declaration: self,
                base_points: 18,
                objective: RoundObjective::SuitCollectionObjective,
                trump_rule: TrumpRule::NoTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::AnyPlayer,
                regional_only: false,
            },
            GameDeclaration::Kontrabauernschnapser => DeclarationInfo {
                declaration: self,
                base_points: 24,
                objective: RoundObjective::WinEveryTrick,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::DefenderOnly,
                regional_only: false,
            },
            GameDeclaration::Herrenschnapser => DeclarationInfo {
                declaration: self,
                base_points: 24,
                objective: RoundObjective::SuitCollectionObjective,
                trump_rule: TrumpRule::CalledTrump,
                rank_order: RankOrder::Standard,
                declarer_constraint: DeclarerConstraint::CallerOnly,
                regional_only: false,
            },
        }
    }

    pub fn base_points(self) -> u8 {
        self.info().base_points
    }

    pub fn objective(self) -> RoundObjective {
        self.info().objective
    }

    pub fn trump_rule(self) -> TrumpRule {
        self.info().trump_rule
    }

    pub fn rank_order(self) -> RankOrder {
        self.info().rank_order
    }
}

/// Role restrictions for who is allowed to declare a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeclarerConstraint {
    AnyPlayer,
    CallerOnly,
    DefenderOnly,
}

/// Trump handling in a declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TrumpRule {
    CalledTrump,
    NoTrump,
}

/// High-level objective category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoundObjective {
    Reach66OrLastTrick,
    LoseEveryTrick,
    WinEveryTrick,
    SchnapserObjective,
    SuitCollectionObjective,
    VariantSpecific,
}

/// Metadata for one declaration type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DeclarationInfo {
    pub declaration: GameDeclaration,
    pub base_points: u8,
    pub objective: RoundObjective,
    pub trump_rule: TrumpRule,
    pub rank_order: RankOrder,
    pub declarer_constraint: DeclarerConstraint,
    pub regional_only: bool,
}

/// Tie-break differences when Gang and Zehnergang collide.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GangTieBreak {
    GangWins,
    ZehnergangWins,
}

/// Switches for optional regional declarations and preferences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionalOptions {
    pub allow_assenbettler: bool,
    pub allow_ass_bettler: bool,
    pub allow_plauderer: bool,
    pub allow_zehnergang: bool,
    pub allow_koenigsgang: bool,
    pub allow_damengang: bool,
    pub allow_bauernloch: bool,
    pub gang_tiebreak: GangTieBreak,
}

impl Default for RegionalOptions {
    fn default() -> Self {
        Self {
            allow_assenbettler: false,
            allow_ass_bettler: false,
            allow_plauderer: false,
            allow_zehnergang: false,
            allow_koenigsgang: false,
            allow_damengang: false,
            allow_bauernloch: false,
            gang_tiebreak: GangTieBreak::GangWins,
        }
    }
}

impl RegionalOptions {
    pub fn is_allowed(&self, declaration: GameDeclaration) -> bool {
        match declaration {
            GameDeclaration::Assenbettler => self.allow_assenbettler,
            GameDeclaration::AssBettler => self.allow_ass_bettler,
            GameDeclaration::Plauderer => self.allow_plauderer,
            GameDeclaration::Zehnergang => self.allow_zehnergang,
            GameDeclaration::Koenigsgang => self.allow_koenigsgang,
            GameDeclaration::Damengang => self.allow_damengang,
            GameDeclaration::Bauernloch => self.allow_bauernloch,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarations_have_non_zero_points() {
        for declaration in GameDeclaration::ALL {
            if declaration == GameDeclaration::Normal {
                continue;
            }
            assert!(declaration.base_points() > 0);
        }
    }

    #[test]
    fn normal_game_uses_standard_order_and_trump() {
        let info = GameDeclaration::Normal.info();
        assert_eq!(info.rank_order, RankOrder::Standard);
        assert_eq!(info.trump_rule, TrumpRule::CalledTrump);
    }

    #[test]
    fn regional_options_gate_regional_declarations() {
        let options = RegionalOptions::default();
        assert!(!options.is_allowed(GameDeclaration::Assenbettler));
        assert!(options.is_allowed(GameDeclaration::Gang));
    }
}
