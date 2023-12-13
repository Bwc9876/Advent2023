use std::cmp::Ordering;

use crate::{day::Day, get_input_for_day};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid tile")
        }
    }
}

type Grid = Vec<Vec<Tile>>;
type R = Vec<usize>;

struct TileGrid {
    tiles: Grid
}

impl TileGrid {

    pub fn parse(input: &str) -> Self {
        let tiles = input.lines().map(|line| {
            line.chars().map(Tile::from_char).collect()
        }).collect();
        Self { tiles }
    }

    pub fn rotate(&self) -> Grid {
        let mut tiles = vec![vec![Tile::Ash; self.tiles.len()]; self.tiles[0].len()];
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tiles[j][i] = *tile;
            }
        }
        tiles
    }

    fn count_differences_in_row(row_1: Vec<Tile>, row_2: Vec<Tile>) -> usize {
        row_1.iter().zip(row_2.iter()).filter(|(t1, t2)| t1 != t2).count()
    }

    fn find_reflections(tiles: &Grid, target_diffs: usize) -> R {
        let centers = tiles.windows(2).enumerate().filter_map(|(i, t)| {
            if Self::count_differences_in_row(t[0].clone(), t[1].clone()) == target_diffs {
                Some((i, true))
            } else if t[0] == t[1] {
                Some((i, false))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        centers.into_iter().filter_map(|(center, used_fix)| {
            let rest_len = tiles.len() - center - 1;

            let mut checks = match center.cmp(&rest_len) {
                Ordering::Greater => (tiles[(center - rest_len + 1)..=center].to_vec(), tiles[center+1..].to_vec()),
                Ordering::Less => (tiles[..=center].to_vec(), tiles[center+1..(center+1)*2].to_vec()),
                Ordering::Equal => (tiles[center-(center-1)..=center].to_vec(), tiles[center+1..].to_vec())
            };

            checks.1.reverse();

            let diffs = checks.0.into_iter().zip(checks.1).map(|(t1, t2)| Self::count_differences_in_row(t1, t2)).sum::<usize>();

            if (used_fix && diffs == 0) || diffs == target_diffs {
                Some(center)
            } else {
                None
            }
        }).collect()
    }

    pub fn find_reflections_rows(&self, target_diffs: usize) -> R {
        Self::find_reflections(&self.tiles, target_diffs)
    }

    pub fn find_reflections_columns(&self, target_diffs: usize) -> R  {
        let rotated = self.rotate();
        Self::find_reflections(&rotated, target_diffs)
    }

    pub fn find_reflection_summaries(&self, target_diffs: usize) -> usize {
        let rows = self.find_reflections_rows(target_diffs);
        let columns = self.find_reflections_columns(target_diffs);

        assert!(rows.len() + columns.len() == 1, "Uh oh");
        
        let columns_stats = columns.iter().map(|c| c + 1).collect::<Vec<_>>();
        let rows_stats = rows.iter().map(|c| (c + 1) * 100).collect::<Vec<_>>();

        columns_stats.iter().sum::<usize>() + rows_stats.iter().sum::<usize>()
    }

}

pub struct Day13;

impl Day for Day13 {

    get_input_for_day!(13);

    fn part_1(&self, input: &str) -> i64 {
        let grids = input.split("\n\n").map(TileGrid::parse).collect::<Vec<_>>();

        grids.iter().map(|grid| grid.find_reflection_summaries(0)).sum::<usize>() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let grids = input.split("\n\n").map(TileGrid::parse).collect::<Vec<_>>();

        grids.iter().map(|grid| grid.find_reflection_summaries(1)).sum::<usize>() as i64
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day13;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 34993);
    }

    #[test]
    fn test_part_2() {
        let day = Day13;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 29341);
    }

}