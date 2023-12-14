use std::collections::HashMap;

use crate::{day::Day, get_input_for_day};

fn make_grid(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    grid
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (usize, usize);

fn get_matches(grid: &[Vec<char>], char: char) -> Vec<Position> {
    grid.iter().enumerate().flat_map(|(y, r)| {
        r.iter().enumerate().filter_map(|(x, c)| {
            if *c == char {
                Some((x, y))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn get_new_pos_after_tilt(direction: &Direction, round: &Position, rounds_in_dir: &[Position], squares_in_dir: &[Position], size: &Position) -> Position {
    let closest_dir_square = &squares_in_dir.iter().filter(|(x, y)| {
        match direction {
            Direction::North => *y < round.1,
            Direction::East => *x > round.0,
            Direction::South => *y > round.1,
            Direction::West => *x < round.0,
        }
    }).min_by_key(|(x, y)| {
        match direction {
            Direction::North => round.1 - *y,
            Direction::East => *x - round.0,
            Direction::South => *y - round.1,
            Direction::West => round.0 - *x,
        }
    });

    let in_betweens = rounds_in_dir.iter().filter(|(x, y)| {
        match direction {
            Direction::North => *y < round.1 && closest_dir_square.map(|v| *y > v.1).unwrap_or(true),
            Direction::East => *x > round.0 && closest_dir_square.map(|v| *x < v.0).unwrap_or(true),
            Direction::South => *y > round.1 && closest_dir_square.map(|v| *y < v.1).unwrap_or(true),
            Direction::West => *x < round.0 && closest_dir_square.map(|v| *x > v.0).unwrap_or(true),
        }
    }).count();

    match direction {
        Direction::North => (round.0, closest_dir_square.map(|v| v.1 + 1).unwrap_or(0) + in_betweens),
        Direction::East => (closest_dir_square.map(|v| v.0 - 1).unwrap_or(size.0 - 1) - in_betweens, round.1),
        Direction::South => (round.0, closest_dir_square.map(|v| v.1 - 1).unwrap_or(size.1 - 1) - in_betweens),
        Direction::West => (closest_dir_square.map(|v| v.0 + 1).unwrap_or(0) + in_betweens, round.1),
    }
}

fn tilt_rounds(direction: Direction, rounds: &[Position], squares: &(Vec<Vec<Position>>, Vec<Vec<Position>>), size: Position) -> Vec<Position> {
    let rounds_per_line = match direction {
        Direction::North | Direction::South => (0..size.0).map(|x| {
            rounds.iter().filter(|(x2, _)| *x2 == x).cloned().collect::<Vec<_>>()
        }).collect::<Vec<_>>(),
        Direction::East | Direction::West => (0..size.1).map(|y| {
            rounds.iter().filter(|(_, y2)| *y2 == y).cloned().collect::<Vec<_>>()
        }).collect::<Vec<_>>(),
    };

    let squares_per_line = match direction {
        Direction::North | Direction::South => &squares.1,
        Direction::East | Direction::West => &squares.0,
    };

    rounds.iter().map(|r| {
        let row_or_col = match direction {
            Direction::North | Direction::South => r.0,
            Direction::East | Direction::West => r.1,
        };
        get_new_pos_after_tilt(&direction, r, &rounds_per_line[row_or_col], &squares_per_line[row_or_col], &size)
    }).collect::<Vec<_>>()
}

fn get_squares_per_line(grid: &Vec<Vec<char>>) -> (Vec<Vec<Position>>, Vec<Vec<Position>>) {
    let mut squares_per_line = (0..grid.len()).map(|_| Vec::new()).collect::<Vec<_>>();
    let mut squares_per_col = (0..grid[0].len()).map(|_| Vec::new()).collect::<Vec<_>>();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == CUBE {
                squares_per_line[y].push((x, y));
                squares_per_col[x].push((x, y));
            }
        }
    }
    (squares_per_line, squares_per_col)
}

fn hash_rounds(rounds: &[Position]) -> u64 {
    // I have no idea how to hash shit rust std lib to the rescue

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    let mut rounds = rounds.to_vec();

    rounds.sort(); // Ensure order doesn't mess with things

    rounds.hash(&mut hasher);

    hasher.finish()
}

pub struct Day14;

const ROUND: char = 'O';
const CUBE: char = '#';

impl Day for Day14 {

    get_input_for_day!(14);

    fn part_1(&self, input: &str) -> i64 {
        let lines =  input.lines().collect::<Vec<_>>();
        let grid = make_grid(lines);
        let rounds = get_matches(&grid, ROUND);
        let squares_per_line = get_squares_per_line(&grid);
        let size = (grid[0].len(), grid.len());
        let tilted_rounds = tilt_rounds(Direction::North, &rounds, &squares_per_line, size);

        tilted_rounds.into_iter().map(|r| {
            grid.len() - r.1
        }).sum::<usize>() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        
        const TIMES: usize = 1000;
        
        let lines =  input.lines().collect::<Vec<_>>();
        let grid = make_grid(lines);

        let mut rounds = get_matches(&grid, ROUND);
        let squares_in_lines = get_squares_per_line(&grid);
        let size = (grid[0].len(), grid.len());

        let dirs = vec![Direction::North, Direction::West, Direction::South, Direction::East];

        rounds.sort();

        let mut seen_states = HashMap::new();

        let mut pos = 0;

        while pos < TIMES {
            for d in &dirs {
                rounds = tilt_rounds(*d, &rounds, &squares_in_lines, size);
            }

            pos += 1;

            let hash = hash_rounds(&rounds);

            let cycle_start = *seen_states.entry(hash).or_insert(pos);

            if cycle_start < pos {
                let cycle_length = pos - cycle_start;
                let to_do = (TIMES - pos) / cycle_length;
                pos += to_do * cycle_length;
            }

        }

        rounds.into_iter().map(|r| {
            grid.len() - r.1
        }).sum::<usize>() as i64
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash_rounds() {
        let rounds = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        let rounds_2 = vec![(1, 0), (0, 0), (2, 0), (3, 0)];
        assert_eq!(hash_rounds(&rounds), hash_rounds(&rounds_2));
    }

    // #[test]
    // fn test_part_1() {
    //     let day = Day14;
    //     let input = day.get_input();
    //     assert_eq!(day.part_1(&input), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     let day = Day14;
    //     let input = day.get_input();
    //     assert_eq!(day.part_2(&input), 0);
    // }

}