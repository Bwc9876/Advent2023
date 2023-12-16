use std::collections::HashSet;

use crate::{day::Day, get_input_for_day};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

impl Tile {

    pub fn parse(c: char) -> Self {
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

type Pos = (usize, usize);
type Tiles = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Beam {
    pos: Pos,
    current_direction: Direction,
}

impl Beam {

    pub fn new(pos: Pos, direction: Direction) -> Self {
        Self {
            pos,
            current_direction: direction,
        }
    }

    pub fn split(self, splitter: &Splitter) -> Vec<Self> {
        match splitter {
            Splitter::Vertical => {
                match self.current_direction {
                    Direction::Up | Direction::Down => {
                        vec![self]
                    }
                    _ => {
                        vec![
                            Self::new(self.pos, Direction::Up),
                            Self::new(self.pos, Direction::Down),
                        ]
                    }
                }
            },
            Splitter::Horizontal => {
                match self.current_direction {
                    Direction::Left | Direction::Right => {
                        vec![self]
                    }
                    _ => {
                        vec![
                            Self::new(self.pos, Direction::Left),
                            Self::new(self.pos, Direction::Right),
                        ]
                    }
                }
            }
        }
    }

    pub fn reflect(self, mirror: &Mirror) -> Self {
        match (mirror, self.current_direction) {
            (Mirror::Forward, Direction::Up) => {
                Self::new(self.pos, Direction::Right)
            },
            (Mirror::Forward, Direction::Down) => {
                Self::new(self.pos, Direction::Left)
            },
            (Mirror::Forward, Direction::Left) => {
                Self::new(self.pos, Direction::Down)
            },
            (Mirror::Forward, Direction::Right) => {
                Self::new(self.pos, Direction::Up)
            },
            (Mirror::Backward, Direction::Up) => {
                Self::new(self.pos, Direction::Left)
            },
            (Mirror::Backward, Direction::Down) => {
                Self::new(self.pos, Direction::Right)
            },
            (Mirror::Backward, Direction::Left) => {
                Self::new(self.pos, Direction::Up)
            },
            (Mirror::Backward, Direction::Right) => {
                Self::new(self.pos, Direction::Down)
            },
        }
    }

    pub fn advance(mut self, tiles: &Tiles, no_inc: bool) -> Option<Vec<Self>> {
        let (x, y) = self.pos;
        let (width, height) = (tiles[0].len(), tiles.len());
        
        if !no_inc {
            self.pos = match self.current_direction {
                Direction::Up => {
                    if y == 0 {
                        return None;
                    }
                    (x, y - 1)
                },
                Direction::Down => {
                    if y == height - 1 {
                        return None;
                    }
                    (x, y + 1)
                },
                Direction::Left => {
                    if x == 0 {
                        return None;
                    }
                    (x - 1, y)
                },
                Direction::Right => {
                    if x == width - 1 {
                        return None;
                    }
                    (x + 1, y)
                },
            };
        }

        let new_tile = &tiles[self.pos.1][self.pos.0];

        match new_tile {
            Tile::Empty => {
                Some(vec![self])
            },
            Tile::Mirror(mirror) => {
                Some(vec![self.reflect(mirror)])
            },
            Tile::Splitter(splitter) => {
                Some(self.split(splitter))
            }
        }
    }


}

fn get_energized(starting_beam: Beam, tiles: &Tiles) -> i64 {
    let mut beams = starting_beam.advance(tiles, true).unwrap();

    let mut visited_poses = HashSet::new();
    let mut visited: HashSet<(Pos, Direction)> = HashSet::new();

    while !beams.is_empty() {
        let mut new_beams = vec![];
        for beam in beams {
            if visited.contains(&(beam.pos, beam.current_direction)) {
                continue;
            } else if !visited_poses.contains(&beam.pos) {
                visited_poses.insert(beam.pos);
            }
            visited.insert((beam.pos, beam.current_direction));
            if let Some(mut new_beam) = beam.advance(tiles, false) {
                new_beams.append(&mut new_beam);
            }
        }
        beams = new_beams;
    }

    visited_poses.len() as i64
}

pub struct Day16;

impl Day for Day16 {

    get_input_for_day!(16);

    fn part_1(&self, input: &str) -> i64 {
        let tiles = input.lines().map(|line| {
            line.chars().map(Tile::parse).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let starting_beam = Beam::new((0, 0), Direction::Right);

        get_energized(starting_beam, &tiles)
    }

    fn part_2(&self, input: &str) -> i64 {
        let tiles = input.lines().map(|line| {
            line.chars().map(Tile::parse).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let (width, height) = (tiles[0].len(), tiles.len());

        (0..width).map(|x| {

            let heights = if x == 0 || x == width - 1 { (0..height).collect() } else { vec![0,  height - 1] };

            heights.into_iter().map(|y| {
                let mut to_check = vec![];

                if x == 0 {
                    to_check.push(Direction::Right);
                } else if x == width - 1 {
                    to_check.push(Direction::Left);
                }

                if y == 0 {
                    to_check.push(Direction::Down);
                } else if y == height - 1 {
                    to_check.push(Direction::Up);
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