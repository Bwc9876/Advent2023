use std::cmp::Ordering;

use crate::{day::Day, get_input_for_day, utils::Grid};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid tile")
        }
    }
}

type Tiles = Grid<Tile>;

struct TileGrid {
    tiles: Tiles
}

impl TileGrid {

    pub fn parse(input: &str) -> Self {
        Self {
            tiles: Tiles::parse(input)
        }
    }

    fn count_differences_in_row(row_1: &[&Tile], row_2: &[&Tile]) -> usize {
        row_1.iter().zip(row_2.iter()).filter(|(t1, t2)| t1 != t2).count()
    }

    fn find_reflections(rows: Vec<Vec<&Tile>>, target_diffs: usize) -> Option<usize> {
        let centers = rows.windows(2).enumerate().filter_map(|(i, t)| {
            if Self::count_differences_in_row(&t[0], &t[1]) == target_diffs {
                Some((i, true))
            } else if t[0] == t[1] {
                Some((i, false))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        centers.into_iter().find_map(|(center, used_fix)| {
            let rest_len = rows.len() - center - 1;

            let mut checks = match center.cmp(&rest_len) {
                Ordering::Greater => (rows[(center - rest_len + 1)..=center].to_vec(), rows[center+1..].to_vec()),
                Ordering::Less => (rows[..=center].to_vec(), rows[center+1..(center+1)*2].to_vec()),
                Ordering::Equal => (rows[center-(center-1)..=center].to_vec(), rows[center+1..].to_vec())
            };

            checks.1.reverse();

            let diffs = checks.0.into_iter().zip(checks.1).map(|(t1, t2)| Self::count_differences_in_row(&t1, &t2)).sum::<usize>();

            if (used_fix && diffs == 0) || diffs == target_diffs {
                Some(center)
            } else {
                None
            }
        })
    }

    pub fn find_reflections_rows(&self, target_diffs: usize) -> Option<usize> {
        Self::find_reflections(self.tiles.iter_rows().collect(), target_diffs)
    }

    pub fn find_reflections_columns(&self, target_diffs: usize) -> Option<usize>  {
        Self::find_reflections(self.tiles.iter_cols().collect(), target_diffs)
    }

    pub fn find_reflection_summaries(&self, target_diffs: usize) -> usize {
        let rows = self.find_reflections_rows(target_diffs);
        let columns = self.find_reflections_columns(target_diffs);
        
        let columns_stats = columns.map(|c| c + 1);
        let rows_stats = rows.map(|r| (r + 1) * 100);

        columns_stats.unwrap_or(rows_stats.unwrap_or(0))
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