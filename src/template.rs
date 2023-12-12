use crate::{day::Day, get_input_for_day};

pub struct Day_n_;

impl Day for Day_n_ {
    fn part_1(&self, input: &str) -> i64 {
        todo!();
    }

    fn part_2(&self, input: &str) -> i64 {
        todo!();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day_n_;
        let input = day.get_input();
        assert_eq!(day.part_1(&input), 0);
    }

    #[test]
    fn test_part_2() {
        let day = Day_n_;
        let input = day.get_input();
        assert_eq!(day.part_2(&input), 0);
    }

}