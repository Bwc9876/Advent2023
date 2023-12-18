use std::collections::HashSet;

use crate::{day::Day, get_input_for_day, utils::{grid::{GridPointer, Position}, Direction, Grid}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mirror {
    Forward,
    Backward
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Splitter {
    Vertical,
    Horizontal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::Mirror(Mirror::Forward),
            '\\' => Self::Mirror(Mirror::Backward),
            '|' => Self::Splitter(Splitter::Vertical),
            '-' => Self::Splitter(Splitter::Horizontal),
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

type Tiles = Grid<Tile>;

#[derive(Debug)]
struct Beam(GridPointer);

impl Beam {

    pub fn new(pos: Position, direction: Direction) -> Self {
        Self(GridPointer::new(pos, direction))
    }

    pub fn split(&mut self, splitter: &Splitter) -> Option<Self> {
        match splitter {
            Splitter::Vertical => {
                match self.0.dir {
                    Direction::East | Direction::West => {
                        self.0.dir = Direction::North;
                        Some(Self::new(self.0.pos, Direction::South))
                    },
                    _ => {
                        None
                    }
                }
            },
            Splitter::Horizontal => {
                match self.0.dir {
                    Direction::North | Direction::South => {
                        self.0.dir = Direction::East;
                        Some(Self::new(self.0.pos, Direction::West))
                    },
                    _ => {
                        None
                    }
                }
            }
        }
    }

    pub fn reflect(&mut self, mirror: &Mirror) {
        self.0.dir = match (mirror, self.0.dir) {
            (Mirror::Forward, Direction::North) => Direction::East,
            (Mirror::Forward, Direction::South) => Direction::West,
            (Mirror::Forward, Direction::East) => Direction::North,
            (Mirror::Forward, Direction::West) => Direction::South,
            (Mirror::Backward, Direction::North) => Direction::West,
            (Mirror::Backward, Direction::South) => Direction::East,
            (Mirror::Backward, Direction::East) => Direction::South,
            (Mirror::Backward, Direction::West) => Direction::North,
        }
    }

    pub fn advance(&mut self, tiles: &Tiles, no_inc: bool) -> Option<Option<Self>> {
        
        if !no_inc && !self.0.move_to_next(tiles) {
            return None;
        }

        let new_tile = tiles.get(self.0.pos)?;

        match new_tile {
            Tile::Empty => {
                Some(None)
            },
            Tile::Mirror(mirror) => {
                self.reflect(mirror);
                Some(None)
            },
            Tile::Splitter(splitter) => {
                Some(self.split(splitter))
            }
        }
    }


}

fn get_energized(mut starting_beam: Beam, tiles: &Tiles) -> i64 {
    let other_starting = starting_beam.advance(tiles, true).unwrap();

    let mut beams = if let Some(other_starting) = other_starting {
        vec![starting_beam, other_starting]
    } else {
        vec![starting_beam]
    };

    let mut visited_poses = HashSet::new();
    let mut visited: HashSet<GridPointer> = HashSet::new();

    while !beams.is_empty() {
        beams = beams.into_iter().filter_map(|mut beam| {
            if visited.contains(&beam.0) {
                return None;
            } else if !visited_poses.contains(&beam.0.pos) {
                visited_poses.insert(beam.0.pos);
            }
            visited.insert(beam.0.clone());
            match beam.advance(tiles, false) {
                Some(Some(new_beam)) => {
                    Some(vec![beam, new_beam])
                },
                Some(None) => {
                    Some(vec![beam])
                },
                None => {
                    None
                }
            }
        }).flatten().collect();
    }

    visited_poses.len() as i64
}

pub struct Day16;

impl Day for Day16 {

    get_input_for_day!(16);

    fn part_1(&self, input: &str) -> i64 {
        let tiles = Grid::parse(input);
        let starting_beam = Beam(GridPointer::zero());
        get_energized(starting_beam, &tiles)
    }

    fn part_2(&self, input: &str) -> i64 {
        let tiles = Grid::parse(input);

        let (width, height) = tiles.size();

        (0..width).map(|x| {

            let heights = if x == 0 || x == width - 1 { (0..height).collect() } else { vec![0,  height - 1] };

            heights.into_iter().map(|y| {
                let mut to_check = vec![];

                if x == 0 {
                    to_check.push(Direction::East);
                } else if x == width - 1 {
                    to_check.push(Direction::West);
                }

                if y == 0 {
                    to_check.push(Direction::South);
                } else if y == height - 1 {
                    to_check.push(Direction::North);
                }

                to_check.into_iter().map(|dir| {
                    let starting_beam = Beam::new((x, y), dir);
                    get_energized(starting_beam, &tiles)
                }).max().unwrap()
            }).max().unwrap()
        }).max().unwrap()
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day16;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 7060);
    }

    #[test]
    fn test_part_2() {
        let day = Day16;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 7493);
    }

}