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

    fn _get_pyramid(ins: &[i64]) -> Vec<Vec<i64>> {
        let res = vec![ins.to_owned()];
        if ins.iter().all(|i| *i == 0) {
            res
        } else {
            let next = ins.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            res.iter().chain(Self::_get_pyramid(&next).iter()).cloned().collect()
        }
    }

    fn get_pyramid(&self) -> Vec<Vec<i64>> {
        Self::_get_pyramid(&self.nums)
    }

    fn calc_next(&self) -> i64 {
        let pyramid = self.get_pyramid();

        pyramid.iter().rev().skip(1).map(|r| r.last().unwrap()).sum()
    }

    fn calc_prev(&self) -> i64 {
        let pyramid = self.get_pyramid();

        pyramid.iter().rev().skip(1).fold(0, |acc, r| {
            r.first().unwrap() - acc
        })
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
