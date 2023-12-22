use std::collections::{HashMap, HashSet};

use crate::{day::Day, get_input_for_day};

type Pos3D = (i64, i64, i64);

struct Brick {
    start_pos: Pos3D,
    end_pos: Pos3D,
}

impl Brick {

    fn parse_pos(input: &str) -> Pos3D {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        (x, y, z)
    }

    pub fn parse(input: &str) -> Self {
        let mut split = input.split('~');
        let start_pos = Self::parse_pos(split.next().unwrap());
        let end_pos = Self::parse_pos(split.next().unwrap());
        Self { start_pos, end_pos }
    }

    pub fn get_parts(&self) -> Vec<Pos3D> {
        let mut parts = Vec::new();
        // Technically only one component changes but eh
        for x in self.start_pos.0..=self.end_pos.0 {
            for y in self.start_pos.1..=self.end_pos.1 {
                for z in self.start_pos.2..=self.end_pos.2 {
                    parts.push((x, y, z));
                }
            }
        }
        parts
    }

}

fn get_branch_size(node: usize, visited: &mut Vec<usize>, supported_by: &Vec<HashSet<usize>>, supports: &Vec<HashSet<usize>>) -> usize {
    visited.push(node);

    let children = &supports[node];

    let size = children.iter().map(|c| {
        if supported_by[*c].len() > 1 && supported_by[*c].iter().any(|s| !visited.contains(s)) {
            0
        } else if supports[*c].is_empty() {
            1
        } else {
            1 + get_branch_size(*c, visited, supported_by, supports)
        }
    }).sum::<usize>();

    size
}

pub struct Day22;

impl Day for Day22 {

    get_input_for_day!(22);

    fn part_1(&self, input: &str) -> i64 {
        let bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();

        let mut highest: HashMap<(i64, i64), i64> = HashMap::new();

        let highest_z = bricks.iter().map(|b| b.end_pos.2.max(b.start_pos.2)).max().unwrap();

        let mut new_bricks = vec![];

        let parts = bricks.iter().map(|b| b.get_parts()).collect::<Vec<_>>();

        let mut done_bricks = HashSet::<usize>::new();

        for z in 1..=highest_z {
            for (i, brick) in parts.iter().enumerate().filter(|(_, ps)| ps.iter().any(|p| p.2 == z)) {
                if done_bricks.contains(&i) {
                    continue;
                }

                let closest = brick.iter().map(|p| (p, highest.get(&(p.0, p.1)).copied().unwrap_or(0))).max_by(|a, b| {
                    a.1.cmp(&b.1).then(a.0.2.cmp(&b.0.2).reverse())
                }).unwrap();

                let delta = closest.0.2 - (closest.1 + 1);

                let new_brick = brick.iter().map(|p| (p.0, p.1, p.2 - delta)).collect::<Vec<_>>();

                for p in new_brick.iter() {
                    if let Some(h) = highest.get_mut(&(p.0, p.1)) {
                        if *h < p.2 {
                            *h = p.2;
                        }
                    } else {
                        highest.insert((p.0, p.1), p.2);
                    }
                }

                new_bricks.push(new_brick);
                done_bricks.insert(i);
            }
        }

        assert_eq!(bricks.len(), new_bricks.len());

        let mut supports = vec![HashSet::<usize>::new(); new_bricks.len()];

        for (i, brick) in new_bricks.iter().enumerate() {
            for p in brick.iter() {
                for (j, other) in new_bricks.iter().enumerate() {
                    if i != j && other.iter().any(|o| o.0 == p.0 && o.1 == p.1 && o.2 == p.2 - 1) {
                        supports[i].insert(j);
                    }
                }
            }
        }

        supports.iter().enumerate().filter(|(i, _)| {
            supports.iter().all(|s| !s.contains(i) || s.len() > 1)
        }).count() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();

        let mut highest: HashMap<(i64, i64), i64> = HashMap::new();

        let highest_z = bricks.iter().map(|b| b.end_pos.2.max(b.start_pos.2)).max().unwrap();

        let mut new_bricks = vec![];

        let parts = bricks.iter().map(|b| b.get_parts()).collect::<Vec<_>>();

        let mut done_bricks = HashSet::<usize>::new();

        for z in 1..=highest_z {
            for (i, brick) in parts.iter().enumerate().filter(|(_, ps)| ps.iter().any(|p| p.2 == z)) {
                if done_bricks.contains(&i) {
                    continue;
                }

                let closest = brick.iter().map(|p| (p, highest.get(&(p.0, p.1)).copied().unwrap_or(0))).max_by(|a, b| {
                    a.1.cmp(&b.1).then(a.0.2.cmp(&b.0.2).reverse())
                }).unwrap();

                let delta = closest.0.2 - (closest.1 + 1);

                let new_brick = brick.iter().map(|p| (p.0, p.1, p.2 - delta)).collect::<Vec<_>>();

                for p in new_brick.iter() {
                    if let Some(h) = highest.get_mut(&(p.0, p.1)) {
                        if *h < p.2 {
                            *h = p.2;
                        }
                    } else {
                        highest.insert((p.0, p.1), p.2);
                    }
                }

                new_bricks.push(new_brick);
                done_bricks.insert(i);
            }
        }

        assert_eq!(bricks.len(), new_bricks.len());

        // Supports is switched here, we want what each brick supports, not what supports each brick
        let mut supports = vec![HashSet::<usize>::new(); new_bricks.len()];
        let mut supported_by = vec![HashSet::<usize>::new(); new_bricks.len()];

        for (i, brick) in new_bricks.iter().enumerate() {
            for p in brick.iter() {
                for (j, other) in new_bricks.iter().enumerate() {
                    if i != j && other.iter().any(|o| o.0 == p.0 && o.1 == p.1 && o.2 == p.2 + 1) {
                        supports[i].insert(j);
                        supported_by[j].insert(i);
                    }
                }
            }
        }

        let mut total = 0;

        for (i, _) in supports.iter().enumerate() {
            let mut v = vec![];
            total += get_branch_size(i, &mut v, &supported_by, &supports);
        }

        total as i64
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day22;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 527);
    }

    #[test]
    fn test_part_2() {
        let day = Day22;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 100376);
    }

}