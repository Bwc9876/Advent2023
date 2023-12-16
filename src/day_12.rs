use std::mem::swap;

use crate::{day::Day, get_input_for_day};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Spring {
    Working,
    Broken,
    Unknown
}

impl Spring {
    fn parse(input: char) -> Self {
        match input {
            '.' => Spring::Working,
            '#' => Spring::Broken,
            _ => Spring::Unknown
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct SpringRow {
    springs: Vec<Spring>,
    groups: Vec<usize>
}

impl SpringRow {
    fn parse(input: &str, part_2: bool) -> Self {
        let mut split = input.split(' ');
        let springs = if part_2 { 
            let initial = split.next().unwrap();
            [initial; 5].join("?").chars().map(Spring::parse).collect()
        } else {
            split.next().unwrap().chars().map(Spring::parse).collect() 
        };
        let mut groups: Vec<_> = split.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        if part_2 {
            groups = groups.repeat(5)
        }
        SpringRow {
            springs,
            groups
        }
    }

    fn find_combinations(&self) -> usize {
        let current_springs = if self.springs.last().unwrap() == &Spring::Working { &self.springs[..self.springs.len() - 1] } else { &self.springs[..] };
        
        let mut new_springs = Vec::with_capacity(current_springs.len() + 1);
        new_springs.push(Spring::Working);
        new_springs.extend_from_slice(current_springs);

        let count = new_springs.len() + 1;

        let (mut old, mut new) = (vec![0; count], vec![0; count]);
        old[0] = 1;

        for i in 1..new_springs.len() {
            if new_springs[i] != Spring::Broken {
                old[i] = 1;
            } else {
                break;
            }
        }

        for group in &self.groups {
            let group = *group;
            let mut current_group = 0;

            for (i, &c) in new_springs.iter().enumerate() {
                if c == Spring::Working {
                    current_group = 0;
                } else {
                    current_group += 1;
                }

                if c != Spring::Broken {
                    new[i + 1] += new[i];
                }
                if current_group >= group && new_springs[i - group] != Spring::Broken {
                    new[i + 1] += old[i - group];
                }
            }
            old.iter_mut().for_each(|v| *v = 0);
            swap(&mut old, &mut new);
        }

        old[count - 1]
    }
}

pub struct Day12;

impl Day for Day12 {

    get_input_for_day!(12);

    fn part_1(&self, input: &str) -> i64 {
        let rows = input.lines().map(|l| SpringRow::parse(l, false)).collect::<Vec<_>>();
        rows.iter().map(|r| r.find_combinations()).sum::<usize>() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let rows = input.lines().map(|l| SpringRow::parse(l, true)).collect::<Vec<_>>();
        rows.iter().map(|r| r.find_combinations()).sum::<usize>() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day12;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 7379);
    }    

    #[test]
    fn test_part_2() {
        let day = Day12;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 7732028747925);
    }

}
