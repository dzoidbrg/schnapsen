use core::fmt;

/// The three fixed seats in Dreierschnapsen.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PlayerId {
    P0,
    P1,
    P2,
}

impl PlayerId {
    pub const ALL: [PlayerId; 3] = [PlayerId::P0, PlayerId::P1, PlayerId::P2];

    pub const fn index(self) -> usize {
        match self {
            PlayerId::P0 => 0,
            PlayerId::P1 => 1,
            PlayerId::P2 => 2,
        }
    }

    pub const fn next_cw(self) -> Self {
        match self {
            PlayerId::P0 => PlayerId::P1,
            PlayerId::P1 => PlayerId::P2,
            PlayerId::P2 => PlayerId::P0,
        }
    }

    pub const fn prev_cw(self) -> Self {
        match self {
            PlayerId::P0 => PlayerId::P2,
            PlayerId::P1 => PlayerId::P0,
            PlayerId::P2 => PlayerId::P1,
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerId::P0 => f.write_str("P0"),
            PlayerId::P1 => f.write_str("P1"),
            PlayerId::P2 => f.write_str("P2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clockwise_cycle_loops_back() {
        assert_eq!(PlayerId::P0.next_cw(), PlayerId::P1);
        assert_eq!(PlayerId::P1.next_cw(), PlayerId::P2);
        assert_eq!(PlayerId::P2.next_cw(), PlayerId::P0);
    }
}
