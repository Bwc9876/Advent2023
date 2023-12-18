pub trait Movement {
    fn get_kernel(&self) -> (isize, isize);

    fn add_to_pos(&self, pos: (usize, usize)) -> (isize, isize) {
        self.add_to_pos_times(pos, 1)
    }

    fn add_to_pos_times(&self, pos: (usize, usize), times: isize) -> (isize, isize) {
        let pos = (pos.0 as isize, pos.1 as isize);
        let kernel = self.get_kernel();
        (pos.0 + kernel.0 * times, pos.1 + kernel.1 * times)
    }
}

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
}

// impl From<ExpandedDirection> for Direction {
//     fn from(dir: ExpandedDirection) -> Self {
//         match dir {
//             ExpandedDirection::North => Self::North,
//             ExpandedDirection::South => Self::South,
//             ExpandedDirection::East => Self::East,
//             ExpandedDirection::West => Self::West,
//             _ => panic!("Cannot convert {:?} to Cardinal Direction", dir),
//         }
//     }
// }

impl Movement for Direction {
    fn get_kernel(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum ExpandedDirection {
//     North,
//     South,
//     East,
//     West,
//     NorthEast,
//     NorthWest,
//     SouthEast,
//     SouthWest,
// }

// pub const COMPASS: [ExpandedDirection; 8] = [
//     ExpandedDirection::North,
//     ExpandedDirection::South,
//     ExpandedDirection::East,
//     ExpandedDirection::West,
//     ExpandedDirection::NorthEast,
//     ExpandedDirection::NorthWest,
//     ExpandedDirection::SouthEast,
//     ExpandedDirection::SouthWest,
// ];

// impl ExpandedDirection {

//     pub fn from_cardinals(dirs: (Direction, Option<Direction>)) -> Self {
//         match dirs.1 {
//             Some(dir) => {
//                 match (dirs.0, dir) {
//                     (Direction::North, Direction::East) => Self::NorthEast,
//                     (Direction::North, Direction::West) => Self::NorthWest,
//                     (Direction::South, Direction::East) => Self::SouthEast,
//                     (Direction::South, Direction::West) => Self::SouthWest,
//                     (Direction::East, Direction::North) => Self::NorthEast,
//                     (Direction::East, Direction::South) => Self::SouthEast,
//                     (Direction::West, Direction::North) => Self::NorthWest,
//                     (Direction::West, Direction::South) => Self::SouthWest,
//                     _ => panic!("Invalid cardinal directions: {:?}, {:?}", dirs.0, dir),
//                 }
//             },
//             None => dirs.0.into()
//         }
//     }

//     pub fn opposite(&self) -> Self {
//         match self {
//             Self::North => Self::South,
//             Self::South => Self::North,
//             Self::East => Self::West,
//             Self::West => Self::East,
//             Self::NorthEast => Self::SouthWest,
//             Self::NorthWest => Self::SouthEast,
//             Self::SouthEast => Self::NorthWest,
//             Self::SouthWest => Self::NorthEast,
//         }
//     }
// }

// impl From<Direction> for ExpandedDirection {
//     fn from(dir: Direction) -> Self {
//         match dir {
//             Direction::North => Self::North,
//             Direction::South => Self::South,
//             Direction::East => Self::East,
//             Direction::West => Self::West,
//         }
//     }
// }

// impl Movement for ExpandedDirection {
//     fn get_kernel(&self) -> (isize, isize) {
//         match self {
//             Self::North => (0, -1),
//             Self::South => (0, 1),
//             Self::East => (1, 0),
//             Self::West => (-1, 0),
//             Self::NorthEast => (1, -1),
//             Self::NorthWest => (-1, -1),
//             Self::SouthEast => (1, 1),
//             Self::SouthWest => (-1, 1),
//         }
//     }
// }
