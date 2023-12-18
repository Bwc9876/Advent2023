use std::collections::{HashMap, BinaryHeap};

use crate::{day::Day, get_input_for_day, utils::{Grid, grid::Position, Direction, dir::CARDINALS}};

struct CityBlock(usize);

impl From<char> for CityBlock {
    fn from(c: char) -> Self {
        Self(c.to_digit(10).unwrap() as usize)
    }
}

type CityGrid = Grid<CityBlock>;

struct Node {
    cost: i64,
    pos: Position,
    dir: Option<Direction>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

fn calc_path(input: &str, min: usize, max: usize) -> i64 {
    let grid = CityGrid::parse(input);

    let target = (grid.width - 1, grid.height - 1);

    let start = (0, 0);

    let start_node = Node {
        cost: 0,
        pos: start,
        dir: None,
    };

    let mut distances = HashMap::new();
    let mut frontier = BinaryHeap::from_iter([start_node]);

    while let Some(node) = frontier.pop() {
        if node.pos == target {
            return -node.cost;
        }

        if node.pos != start && distances.get(&(node.pos, node.dir.unwrap())).map(|&c| -node.cost > c).unwrap_or(false) {
            continue;
        }

        for dir in CARDINALS {
            if node.dir.map(|d| d == dir || d.opposite() == dir ).unwrap_or(false) {
                continue;
            }

            let mut next_cost = -node.cost;

            for step in 1..=max {
                if let Some(moved) = grid.get_next_pos_times(node.pos, dir, step) {
                    next_cost += grid.get(moved).unwrap().0 as i64;

                    if min <= step && distances.get(&(moved, dir)).map(|&c| next_cost < c).unwrap_or(true) {
                        distances.insert((moved, dir), next_cost);
                        frontier.push(Node {
                            cost: -next_cost,
                            pos: moved,
                            dir: Some(dir),
                        });
                    }       
                }
            }
        }
    }

    unreachable!("No path found");
}

pub struct Day17;

impl Day for Day17 {

    get_input_for_day!(17);

    fn part_1(&self, input: &str) -> i64 {
        calc_path(input, 1, 3)
    }

    fn part_2(&self, input: &str) -> i64 {
        calc_path(input, 4, 10)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day17;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 638);
    }

    #[test]
    fn test_part_2() {
        let day = Day17;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 748);
    }

}