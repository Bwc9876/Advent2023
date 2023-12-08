use std::collections::HashMap;
use std::cmp::{max, min};

use crate::{day::Day, get_input_for_day};


pub struct Day8;

struct Node {
    pub id: String,
    left: String,
    right: String,
}

impl Node {

    pub fn parse(input: &str) -> Self {
        let mut split = input.split(" = ");

        let id = split.next().unwrap();

        let children = split.next().unwrap().split(", ").collect::<Vec<&str>>();

        let left = children[0].replace('(', "");
        let right = children[1].replace(')', "");

        Self {
            id: id.to_string(),
            left,
            right
        }
    }

    pub fn get_distance(&self, condition: impl Fn(&str) -> bool, directions: &Vec<char>, nodes: &HashMap<String, Node>) -> i64 {
        let mut current = self.id.clone();

        let dir_len = directions.len();
        let mut dir_pointer = 0;

        while !condition(&current) {
            let node = nodes.get(&current).unwrap();

            let dir = directions[dir_pointer % dir_len]; 

            if dir == 'L' {
                current = node.left.clone();
            } else {
                current = node.right.clone();
            }   

            dir_pointer += 1;
        }

        dir_pointer as i64
    }

}

fn greatest_common_denominator(a: i64, b: i64) -> i64 {
    let mut max = max(a, b);
    let mut min = min(a, b);

    while min != 0 {
        let hold = min;
        min = max % min;
        max = hold;
    }

    max
}

fn least_common_denominator(a: i64, b: i64) -> i64 {
    (a * b) / greatest_common_denominator(a, b)
}

impl Day for Day8 {

    get_input_for_day!(8);

    fn part_1(&self, input: &str) -> i64 {
        let lines = input.lines().collect::<Vec<&str>>();
        let directions = lines[0].chars().collect::<Vec<char>>();
        let nodes = lines[2..].iter().map(|l| Node::parse(l)).map(|n| (n.id.clone(), n)).collect::<HashMap<String, Node>>();
        let start = nodes.get("AAA").unwrap();

        start.get_distance(|s| s == "ZZZ", &directions, &nodes)
    }

    fn part_2(&self, input: &str) -> i64 {
        let lines = input.lines().collect::<Vec<&str>>();

        let directions = lines[0].chars().collect::<Vec<char>>();

        let nodes = lines[2..].iter().map(|l| Node::parse(l)).map(|n| (n.id.clone(), n)).collect::<HashMap<String, Node>>();

        let distances_to_z: Vec<i64> = nodes.values().filter(|n| n.id.ends_with('A')).map(|n| {
            n.get_distance(|s| s.ends_with('Z'), &directions, &nodes)
        }).collect();

        
        distances_to_z.into_iter().fold(1, least_common_denominator)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day8;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 12169);
    }

    #[test]
    fn test_part_2() {
        let day = Day8;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 12030780859469);
    }
}