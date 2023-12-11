use std::collections::HashMap;

use crate::{day::Day, get_input_for_day};

#[derive(Debug, Clone, Copy)]
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

    pub fn ninety_deg(&self, clockwise: bool) -> Self {
        match self {
            Self::North => if clockwise { Self::East } else { Self::West },
            Self::South => if clockwise { Self::West } else { Self::East },
            Self::East => if clockwise { Self::South } else { Self::North },
            Self::West => if clockwise { Self::North } else { Self::South }
        }
    }

    pub fn add_to_pos(&self, pos: (usize, usize)) -> (i32, i32) {
        let pos = (pos.0 as i32, pos.1 as i32);
        match self {
            Self::North => (pos.0, pos.1 - 1),
            Self::South => (pos.0, pos.1 + 1),
            Self::East => (pos.0 + 1, pos.1),
            Self::West => (pos.0 - 1, pos.1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
        match (a, b) {
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => Self::Vertical,
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => Self::Horizontal,
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => Self::NeBend,
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => Self::NwBend,
            (Direction::South, Direction::East) | (Direction::East, Direction::South) => Self::SeBend,
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => Self::SwBend,
            _ => panic!("Can't determine tile from directions")
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
            Self::Vertical | Self::Horizontal => coming_from.opposite(),
            Self::NeBend => coming_from.ninety_deg(matches!(coming_from, Direction::North)),
            Self::NwBend => coming_from.ninety_deg(matches!(coming_from, Direction::West)),
            Self::SeBend => coming_from.ninety_deg(matches!(coming_from, Direction::East)),
            Self::SwBend => coming_from.ninety_deg(matches!(coming_from, Direction::South)),
            Self::Start => panic!("Can't follow start tile as it's unknown"),
            Self::Empty => panic!("Can't follow empty tile")
        }
    }
}

fn find_start(map: &[Vec<MapTile>]) -> (usize, usize) {
    map.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, tile)| if matches!(*tile, MapTile::Start) { Some((x, y)) } else { None })).unwrap()
}

fn get_adjacents(pos: (usize, usize), map: &Vec<Vec<MapTile>>) -> Vec<((usize, usize), Direction)> {
    vec![Direction::North, Direction::South, Direction::East, Direction::West].into_iter().filter_map(|dir| {
        let pos = dir.add_to_pos(pos);

        if pos.0 > 0 && pos.1 > 0 && pos.0 < map[0].len() as i32 && pos.1 < map.len() as i32 {
            Some(((pos.0 as usize, pos.1 as usize), dir))
        } else {
            None
        }
    }).collect()
}

fn get_connectors_of_tile(pos: (usize, usize), map: &Vec<Vec<MapTile>>) -> Vec<((usize, usize), Direction)> {
    let adjacents = get_adjacents(pos, map);
    adjacents.into_iter().filter(|adjacent| map[adjacent.0.1][adjacent.0.0].has_direction(&adjacent.1.opposite())).collect::<Vec<_>>()
}

fn follow_loop(map: &Vec<Vec<MapTile>>) -> HashMap<(usize, usize), MapTile> {
    
    let start_pos = find_start(map);
    let connectors = get_connectors_of_tile(start_pos, map);
    let start_tile = MapTile::determine_from_directions(&connectors[0].1, &connectors[1].1);

    let mut current = connectors[0];
    let tile = map[current.0.1][current.0.0];

    let mut loop_tiles = [(start_pos, start_tile), (current.0, tile)].iter().copied().collect::<HashMap<_, _>>();

    while current.0 != start_pos {
        let current_tile = &map[current.0.1][current.0.0];
        let direction = current_tile.follow(current.1.opposite());
        let next_pos = direction.add_to_pos(current.0);
        loop_tiles.insert(current.0, *current_tile);
        current = ((next_pos.0 as usize, next_pos.1 as usize), direction);
    }

    loop_tiles
}

pub struct Day10;

impl Day for Day10 {

    get_input_for_day!(10);

    fn part_1(&self, input: &str) -> i64 {
        let map = input.lines().map(|line| line.chars().map(MapTile::parse).collect::<Vec<_>>()).collect::<Vec<_>>();

        let tiles = follow_loop(&map);

        (tiles.len() / 2) as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let map = input.lines().map(|line| line.chars().map(MapTile::parse).collect::<Vec<_>>()).collect::<Vec<_>>();

        let loop_tiles = follow_loop(&map);

        let count = map.iter().enumerate().fold(0, |count, (y, row)| {
            let row_count = row.iter().enumerate().fold((0, false), |(count, toggle), (x, _)| {
                if let Some(tile) = loop_tiles.get(&(x, y)) {
                    if tile.has_direction(&Direction::South) {
                        return (count, !toggle);
                    }
                } else if toggle {
                    return (count + 1, toggle);
                }
                (count, toggle)
            }).0;
            count + row_count
        });

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
        assert_eq!(day.part_1(input), 6815);
    }

    #[test]
    fn test_part_2() {
        let day = Day10;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 269);
    }
}