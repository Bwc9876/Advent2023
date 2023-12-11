use std::ops::Range;

use crate::{day::Day, get_input_for_day};

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<char>>,
    pub expansion_rows: Vec<usize>,
    pub expansion_columns: Vec<usize>,
}

impl Universe {

    const EMPTY: char = '.';
    const GALAXY: char = '#';

    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Self { grid, expansion_rows: vec![], expansion_columns: vec![] }
    }

    fn expand_rows(&mut self) {
        let expansion_rows = self.grid.iter().enumerate().filter_map(|(y, row)| {
            if row.iter().all(|c| *c == Self::EMPTY) {
                Some(y)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        self.expansion_rows = expansion_rows;
    }

    fn expand_columns(&mut self) {
        let expansion_columns = self.grid[0].iter().enumerate().filter_map(|(x, _)| {
                if self.grid.iter().map(|row| row[x]).all(|c| c == Self::EMPTY) {
                    Some(x)
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        self.expansion_columns = expansion_columns;
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_columns();
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(|(x, column)| {
                if *column == Self::GALAXY {
                    Some((x, y))
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }
}

fn construct_range(start: usize, end: usize) -> Range<usize> {
    if start < end {
        start..end
    } else {
        end..start
    }
}

fn distance_between_galaxies(g1: (usize, usize), g2: (usize, usize), expansion_amount: usize, uni: &Universe) -> usize {  
    if g1 == g2 {
        return 0;
    }

    let x_range = construct_range(g1.0, g2.0);
    let y_range = construct_range(g1.1, g2.1);

    let expansions_rows = uni.expansion_rows.iter().filter(|r| {
        y_range.contains(r)
    }).count() * (expansion_amount - 1);

    let expansions_columns = uni.expansion_columns.iter().filter(|c| {
        x_range.contains(c)
    }).count() * (expansion_amount - 1);

    x_range.count() + y_range.count() + expansions_rows + expansions_columns
}

pub struct Day11;

impl Day for Day11 {

    get_input_for_day!(11);

    fn part_1(&self, input: &str) -> i64 {
        let mut universe = Universe::parse(input);
        universe.expand();

        let galaxies = universe.get_galaxies();

        (galaxies.iter().flat_map(|g| {
            galaxies.iter().map(|g2| {
                distance_between_galaxies(*g, *g2, 2, &universe)
            }).collect::<Vec<_>>()
        }).sum::<usize>() / 2) as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let mut universe = Universe::parse(input);
        universe.expand();

        const ONE_MILLION: usize = 1000000;

        let galaxies = universe.get_galaxies();

        (galaxies.iter().flat_map(|g| {
            galaxies.iter().map(|g2| {
                distance_between_galaxies(*g, *g2, ONE_MILLION, &universe)
            }).collect::<Vec<_>>()
        }).sum::<usize>() / 2) as i64
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day11;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 9536038);
    }

    #[test]
    fn test_part_2() {
        let day = Day11;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 447744640566);
    }
    
}
