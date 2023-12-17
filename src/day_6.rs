use crate::{day::Day, get_input_for_day, utils::parse_padded_numbers};


pub struct Day6;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {

    pub fn parse(input: &str) -> Vec<Self> {
        let split = input.lines().collect::<Vec<&str>>();
        let times = split[0].split(": ").nth(1).unwrap();
        let distances = split[1].split(": ").nth(1).unwrap();

        let times = parse_padded_numbers(times);
        let distances = parse_padded_numbers(distances);

        times.into_iter().zip(distances).map(|(time, distance)| Self { time, distance }).collect::<Vec<Self>>()
    }

    pub fn parse_part_2(input: &str) -> Self {
        let split = input.lines().collect::<Vec<&str>>();
        let time = split[0].split(": ").nth(1).unwrap().replace(' ', "").parse::<u64>().unwrap();
        let distance = split[1].split(": ").nth(1).unwrap().replace(' ', "").parse::<u64>().unwrap();

        Self { time, distance }
    }

    // This is my original solution, which I want to redo bc of how easy day 6 was
    // pub fn ways_to_win(&self) -> u64 {
    //     let time_range = 1..self.time;
    //     time_range.filter(|t| {
    //         let speed = t;
    //         let distance_traveled = speed * (self.time - t);
    //         distance_traveled >= self.distance
    //     }).count() as u64
    // }

    // New solution using quadratics
    pub fn ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let rt = (time * time - 4_f64 * distance).sqrt();
        let lower_bound = ((-time + rt) / -2_f64).ceil();
        let upper_bound = ((-time - rt) / -2_f64).ceil();
        (upper_bound - lower_bound) as u64
    }

}

impl Day for Day6 {

    get_input_for_day!(6);

    fn part_1(&self, input: &str) -> i64 {
        Race::parse(input).into_iter().fold(1_u64, |acc, race| acc * race.ways_to_win()) as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        Race::parse_part_2(input).ways_to_win() as i64
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day6;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 3316275);
    }

    #[test]
    fn test_part_2() {
        let day = Day6;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 27102791);
    }

}
