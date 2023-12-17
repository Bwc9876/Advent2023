#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub const CARDINALS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn ninety_deg(&self, clockwise: bool) -> Self {
        match self {
            Self::North => {
                if clockwise {
                    Self::East
                } else {
                    Self::West
                }
            }
            Self::South => {
                if clockwise {
                    Self::West
                } else {
                    Self::East
                }
            }
            Self::East => {
                if clockwise {
                    Self::South
                } else {
                    Self::North
                }
            }
            Self::West => {
                if clockwise {
                    Self::North
                } else {
                    Self::South
                }
            }
        }
    }

    pub fn add_to_pos(&self, pos: (usize, usize)) -> (i32, i32) {
        let pos = (pos.0 as i32, pos.1 as i32);
        match self {
            Self::North => (pos.0, pos.1 - 1),
            Self::South => (pos.0, pos.1 + 1),
            Self::East => (pos.0 + 1, pos.1),
            Self::West => (pos.0 - 1, pos.1),
        }
    }
}
