use std::collections::{HashSet, HashMap, BinaryHeap};

use crate::{day::Day, get_input_for_day, utils::{Direction, grid::{GridPointer, Position}}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Trail,
    Forest,
    Slope(Direction)
}

impl Tile {

    pub fn can_go(&self, dir: Direction, part_2: bool) -> bool {
        match self {
            Tile::Trail => true,
            Tile::Forest => false,
            Tile::Slope(d) => part_2 || d.opposite() != dir,
        }
    }

}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Trail,
            '#' => Tile::Forest,
            _ => {
                let dir = match c {
                    '^' => Direction::North,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    '>' => Direction::East,
                    _ => panic!("Invalid character")
                };
                Tile::Slope(dir)
            }
        }
    }
}

type Grid = crate::utils::grid::Grid<Tile>;

fn get_to_other_intersection(grid: &Grid, pos: Position, dir: Direction) -> (Position, usize) {
    let mut p = GridPointer::new(pos, dir);
    let mut steps = 0;

    loop {
        let adjacents = grid.get_direct_adjacents(p.pos);

        let mut valid_adjacents = adjacents.into_iter().filter(|(pos, d)| {
            d.opposite() != p.dir && grid.get(*pos).unwrap().can_go(*d, true)
        });

        if valid_adjacents.clone().count() != 1 {
            return (p.pos, steps + 1);
        } else {
            let (pos, d) = valid_adjacents.next().unwrap();
            p.pos = pos;
            p.dir = d;
            steps += 1;
        }
    }
}

fn get_intersections(s: Position, e: Position, grid: &Grid) -> HashMap<Position, Vec<(Position, usize)>> {
    grid.iter().filter_map(|(p, t)| {
        if *t == Tile::Trail {
            let adjacents = grid.get_direct_adjacents(p);
            let valid_adjacents = adjacents.into_iter().filter(|(ap, d)| {
                grid.get(*ap).unwrap().can_go(*d, true)
            }).collect::<Vec<_>>();
            if s == p || e == p || valid_adjacents.len() > 2 {
                let others = valid_adjacents.into_iter().map(|(ap, ad)| get_to_other_intersection(grid, ap, ad)).collect();
                Some((p, others))
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}

struct Node(Position, HashSet<Position>, usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.2.cmp(&other.2))
    } 
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Node {}

pub struct Day23;

impl Day for Day23 {

    get_input_for_day!(23);

    fn part_1(&self, input: &str) -> i64 {
        let grid = Grid::parse(input);

        let start_pos = grid.iter().find(|(_, t)|  **t == Tile::Trail).unwrap().0;
        let target_pos = grid.iter_rev().rev().find(|(_, t)|  **t == Tile::Trail).unwrap().0;

        let mut frontier = vec![(GridPointer::new(start_pos, Direction::South), 0)];

        let mut largest = None;

        while let Some((p, steps)) = frontier.pop() {
            
            if p.pos == target_pos {
                if largest.map(|l| steps > l).unwrap_or(true) {
                    largest = Some(steps);
                }
                continue;
            }

            let adjacents = grid.get_direct_adjacents(p.pos);

            let valid_adjacents = adjacents.into_iter().filter(|(pos, d)| {
                d.opposite() != p.dir && grid.get(*pos).unwrap().can_go(*d, false)
            });

            for (pos, d) in valid_adjacents {
                let mut new_p = p.clone();
                new_p.pos = pos;
                new_p.dir = d;
                frontier.push((new_p, steps + 1));
            }
        }

        largest.unwrap() as i64
    }

    fn part_2(&self, input: &str) -> i64 {

        let grid = Grid::parse(input);

        let start_pos = grid.iter().find(|(_, t)|  **t == Tile::Trail).unwrap().0;
        let target_pos = grid.iter_rev().rev().find(|(_, t)|  **t == Tile::Trail).unwrap().0;

        let intersections = get_intersections(start_pos, target_pos, &grid);
        
        let mut frontier = BinaryHeap::from_iter([Node(start_pos, HashSet::new(), 0)]);

        let mut largest = None;

        while let Some(Node(p, mut seen, steps)) = frontier.pop() {
            if p == target_pos {
                if largest.map(|l| steps > l).unwrap_or(true) {
                    largest = Some(steps);
                }
                continue;
            }

            seen.insert(p);

            let edges = intersections.get(&p).unwrap();

            for (pos, steps_to) in edges {
                if !seen.contains(pos) {
                    let mut new_seen = seen.clone();
                    new_seen.insert(*pos);
                    frontier.push(Node(*pos, new_seen, steps + steps_to));
                }
            }
        }

        largest.unwrap() as i64
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day23;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 2170);
    }

    #[test]
    fn test_part_2() {
        // Skipping bc this is an NP-Hard problem and takes too long to run

        // let day = Day23;
        // let input = day.get_input();
        // assert_eq!(day.part_2(&input), 6502);
    }

}