use std::{collections::HashSet, iter::once};

use crate::{day::Day, get_input_for_day, utils::{Grid, grid::Position}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    GardenPlot,
    Rock
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            '.' => Tile::GardenPlot,
            '#' => Tile::Rock,
            _ => panic!("Invalid tile")
        }
    }
}

type Garden = Grid<Tile>;

pub struct Day21;

impl Day for Day21 {

    get_input_for_day!(21);

    fn part_1(&self, input: &str) -> i64 {
        const STEPS: usize = 64;

        // 61051250 - Too low

        let grid = Garden::parse(input);
        let start_pos = grid.iter().find(|(_, t)| **t == Tile::Start).unwrap().0;

        let mut tracked = HashSet::<Position>::new();
        let mut seen_tiles = HashSet::<Position>::new();

        tracked.insert(start_pos);
        seen_tiles.insert(start_pos);

        for _ in 0..STEPS {
            let mut new_tracked = HashSet::<Position>::new();
            for p in tracked.iter() {
                for (adj, _) in grid.get_direct_adjacents(*p) {
                    let tile = grid.get(adj).unwrap();
                    if tile == &Tile::GardenPlot || tile == &Tile::Start {
                        if seen_tiles.contains(&adj) {
                            continue;
                        }
                        new_tracked.insert(adj);
                        seen_tiles.insert(adj);
                    }
                }
            }
            tracked = new_tracked;
        }
        
        seen_tiles.iter().filter(|p| {
            let dist = ((p.0 as i64) - (start_pos.0 as i64)) + ((p.1 as i64) - (start_pos.1 as i64));
            let dist = dist.abs();
            if STEPS % 2 == 0 {
                dist % 2 == 0
            } else {
                dist % 2 == 1
            }
        }).count() as i64
    }

    fn part_2(&self, input: &str) -> i64 {

        const DELTA: usize = 3;

        let grid = Garden::parse(input);
        let start_pos = grid.iter().find(|(_, t)| **t == Tile::Start).unwrap().0;

        let half_step = start_pos.0;
        let steps = grid.width * DELTA + half_step;

        let mut tracked = HashSet::<(isize, isize)>::new();
        let mut seen_tiles = HashSet::<(isize, isize)>::new();

        tracked.insert((start_pos.0 as isize, start_pos.1 as isize));
        seen_tiles.insert((start_pos.0 as isize, start_pos.1 as isize));

        let mut coeffs = vec![];

        for i in 1..=steps {
            let mut new_tracked = HashSet::<(isize, isize)>::new();
            for p in tracked.iter() {
                for (adj, _) in grid.get_direct_adjacents_wrapping(*p) {
                    let tile = grid.infinite_get(adj);
                    if tile == &Tile::GardenPlot || tile == &Tile::Start {
                        if seen_tiles.contains(&adj) {
                            continue;
                        }
                        new_tracked.insert(adj);
                        seen_tiles.insert(adj);
                    }
                }
            }
            if i % grid.width == half_step {
                let seen = seen_tiles.iter().filter(|p| {
                    let dist = ((p.0 as i64) - (start_pos.0 as i64)) + ((p.1 as i64) - (start_pos.1 as i64));
                    let dist = dist.abs();
                    if i % 2 == 0 {
                        dist % 2 == 0
                    } else {
                        dist % 2 == 1
                    }
                }).count() as i64;
                coeffs.push(seen);
            }
            tracked = new_tracked;
        }
        
        let diffs = once(coeffs[0]).chain(coeffs.windows(2).map(|w| w[1] - w[0])).collect::<Vec<_>>();

        let (a, b, c) = (diffs[0], diffs[1], diffs[2]);

        const ACTUAL_STEPS: usize = 26501365;

        let n = ((ACTUAL_STEPS - half_step) / grid.width) as i64;

        a + (b * n) + (n * (n - 1) / 2) * (c - b)
    }
    
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day21;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 3776);
    }

    #[test]
    fn test_part_2() {
        let day = Day21;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 625587097150084);
    }

}