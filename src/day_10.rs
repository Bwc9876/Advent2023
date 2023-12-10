use std::collections::HashMap;

use crate::{day::Day, get_input_for_day};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Vertical,
    Horizontal,
    NeBend,
    NwBend,
    SeBend,
    SwBend,
    Start,
    Empty
}

impl MapTile {

    pub fn parse(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NeBend,
            'J' => Self::NwBend,
            '7' => Self::SwBend,
            'F' => Self::SeBend,
            'S' => Self::Start,
            _ => Self::Empty
        }
    }

    pub fn determine_from_directions(a: &Direction, b: &Direction) -> Self {
        match a {
            Direction::North => match b {
                Direction::South => Self::Vertical,
                Direction::East => Self::NeBend,
                Direction::West => Self::NwBend,
                _ => panic!("Invalid directions")
            },
            Direction::South => match b {
                Direction::North => Self::Vertical,
                Direction::East => Self::SeBend,
                Direction::West => Self::SwBend,
                _ => panic!("Invalid directions")
            },
            Direction::East => match b {
                Direction::North => Self::NeBend,
                Direction::South => Self::SeBend,
                Direction::West => Self::Horizontal,
                _ => panic!("Invalid directions")
            },
            Direction::West => match b {
                Direction::North => Self::NwBend,
                Direction::South => Self::SwBend,
                Direction::East => Self::Horizontal,
                _ => panic!("Invalid directions")
            }
        }
    }

    pub fn has_direction(&self, direction: &Direction) -> bool {
        match self {
            Self::Vertical => matches!(direction, Direction::North | Direction::South),
            Self::Horizontal => matches!(direction, Direction::East | Direction::West),
            Self::NeBend => matches!(direction, Direction::North | Direction::East),
            Self::NwBend => matches!(direction, Direction::North | Direction::West),
            Self::SeBend => matches!(direction, Direction::South | Direction::East),
            Self::SwBend => matches!(direction, Direction::South | Direction::West),
            Self::Start => false,
            Self::Empty => false
        }
    }

    pub fn follow(&self, coming_from: Direction) -> Direction {
        match self {
            Self::Vertical => match coming_from {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                _ => panic!("Can't follow vertical tile from non-vertical direction")
            },
            Self::Horizontal => match coming_from {
                Direction::East => Direction::West,
                Direction::West => Direction::East,
                _ => panic!("Can't follow horizontal tile from non-horizontal direction")
            },
            Self::NeBend => match coming_from {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                _ => panic!("Can't follow NE bend tile from non-NE direction")
            },
            Self::NwBend => match coming_from {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                _ => panic!("Can't follow NW bend tile from non-NW direction")
            },
            Self::SeBend => match coming_from {
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                _ => panic!("Can't follow SE bend tile from non-SE direction")
            },
            Self::SwBend => match coming_from {
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                _ => panic!("Can't follow SW bend tile from non-SW direction")
            },
            Self::Start => coming_from,
            Self::Empty => panic!("Can't follow empty tile")
        }
    }
}

fn find_start(map: &[Vec<MapTile>]) -> (usize, usize) {
    map.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, tile)| if matches!(*tile, MapTile::Start) { Some((x, y)) } else { None })).unwrap()
}

fn get_adjacent(pos: (usize, usize), map: &Vec<Vec<MapTile>>) -> Vec<((usize, usize), Direction)> {
    let mut adjacent = vec![];

    if pos.1 > 0 {
        adjacent.push(((pos.0, pos.1 - 1), Direction::North));
    }
    if pos.0 < map[0].len() - 1 {
        adjacent.push(((pos.0 + 1, pos.1), Direction::East));
    }
    if pos.1 < map.len() - 1 {
        adjacent.push(((pos.0, pos.1 + 1), Direction::South));
    }
    if pos.0 > 0 {
        adjacent.push(((pos.0 - 1, pos.1), Direction::West));
    }

    adjacent
}

pub struct Day10;

impl Day for Day10 {

    get_input_for_day!(10);

    fn part_1(&self, input: &str) -> i64 {
        let map = input.lines().map(|line| line.chars().map(MapTile::parse).collect::<Vec<_>>()).collect::<Vec<_>>();

        let start_pos = find_start(&map);

        let adjacent_to_start = get_adjacent(start_pos, &map);

        let connectors = adjacent_to_start.into_iter().filter(|(pos, dir)| map.get(pos.1).and_then(|r| r.get(pos.0)).map(|t| t.has_direction(&dir.opposite())).unwrap_or(false)).collect::<Vec<_>>();

        let mut count = 1;
        let mut current = connectors.into_iter().next().unwrap();

        while current.0 != start_pos {
            let current_tile = &map[current.0.1][current.0.0];

            let direction = current_tile.follow(current.1.opposite());

            let next_pos = (current.0.0 as i32 + match direction {
                Direction::East => 1,
                Direction::West => -1,
                _ => 0
            }, current.0.1 as i32 + match direction {
                Direction::South => 1,
                Direction::North => -1,
                _ => 0
            });

            current = ((next_pos.0 as usize, next_pos.1 as usize), direction);
            count += 1;
        }

        count / 2
    }

    fn part_2(&self, input: &str) -> i64 {
        let map = input.lines().map(|line| line.chars().map(MapTile::parse).collect::<Vec<_>>()).collect::<Vec<_>>();

        let start_pos = find_start(&map);

        let adjacent_to_start = get_adjacent(start_pos, &map);

        let connectors = adjacent_to_start.into_iter().filter(|(pos, dir)| map.get(pos.1).and_then(|r| r.get(pos.0)).map(|t| t.has_direction(&dir.opposite())).unwrap_or(false)).collect::<Vec<_>>();

        let start_tile = MapTile::determine_from_directions(&connectors[0].1, &connectors[1].1);

        let mut current = connectors.into_iter().next().unwrap();

        let tile = map[current.0.1][current.0.0];

        let mut loop_tiles = [(start_pos, start_tile), (current.0, tile)].iter().cloned().collect::<HashMap<_, _>>();

        while current.0 != start_pos {

            let current_tile = &map[current.0.1][current.0.0];

            let direction = current_tile.follow(current.1.opposite());

            let next_pos = (current.0.0 as i32 + match direction {
                Direction::East => 1,
                Direction::West => -1,
                _ => 0
            }, current.0.1 as i32 + match direction {
                Direction::South => 1,
                Direction::North => -1,
                _ => 0
            });

            loop_tiles.insert(current.0, *current_tile);

            current = ((next_pos.0 as usize, next_pos.1 as usize), direction);
        }

        let mut count = 0;

        for (y, row) in map.iter().enumerate() {
            let mut toggle = false;
            for (x, _) in row.iter().enumerate() {
                if let Some(tile) = loop_tiles.get(&(x, y)) {
                    if tile.has_direction(&Direction::South) {
                        toggle = !toggle;
                    }
                } else if toggle {
                    count += 1;
                }
            }
        }

       count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day10;
        let input = day.get_input();
        assert_eq!(day.part_1(&input), 6815);
    }

    #[test]
    fn test_part_2() {
        let day = Day10;
        let input = day.get_input();
        assert_eq!(day.part_2(&input), 269);
    }
}