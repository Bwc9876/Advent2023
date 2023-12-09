use crate::{day::Day, get_input_for_day};

struct History {
    nums: Vec<i64>,
}

impl History {
    fn parse(input: &str) -> Self {
        Self {
            nums: input
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
        }
    }

    fn get_pyramid(&self) -> Vec<Vec<i64>> {
        let mut current = self.nums.clone();
        let mut rows = vec![self.nums.clone()];

        while current.iter().any(|n| *n != 0_i64) {
            let mut new = vec![];

            for i in 0..current.len() - 1 {
                new.push(current[i + 1] - current[i]);
            }

            rows.push(new.clone());
            current = new;
        }

        rows
    }

    fn calc_next(&self) -> i64 {
        let pyramid = self.get_pyramid();

        let mut val = 0;

        for row in pyramid.iter().rev().skip(1) {
            val += row.last().unwrap();
        }

        val
    }

    fn calc_prev(&self) -> i64 {
        let pyramid = self.get_pyramid();

        let mut val = 0;

        for row in pyramid.iter().rev().skip(1) {
            val = row.first().unwrap() - val;
        }

        val
    }
}

pub struct Day9;

impl Day for Day9 {

    get_input_for_day!(9);

    fn part_1(&self, input: &str) -> i64 {
        let histories = input.lines().map(History::parse).collect::<Vec<_>>();

        histories.iter().map(|h| h.calc_next()).sum()
    }

    fn part_2(&self, input: &str) -> i64 {
        let histories = input.lines().map(History::parse).collect::<Vec<_>>();

        histories.iter().map(|h| h.calc_prev()).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day9;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 1901217887);
    }

    #[test]
    fn test_part_2() {
        let day = Day9;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 905);
    }
}
