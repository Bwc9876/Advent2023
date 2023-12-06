use crate::{day::Day, get_input_for_day, util::parse_padded_numbers};


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

    pub fn ways_to_win(&self) -> u64 {
        let time_range = 1..self.time;
        time_range.filter(|t| {
            let speed = t;
            let distance_traveled = speed * (self.time - t);
            distance_traveled >= self.distance
        }).count() as u64
    }

}

impl Day for Day6 {

    get_input_for_day!(6);

    fn part_1(&self, input: &str) -> i32 {
        let races = Race::parse(input);
        
        let mut error_margin = 1;

        for race in races {
            error_margin *= race.ways_to_win();
        }

        error_margin as i32
    }

    fn part_2(&self, input: &str) -> i32 {
        let race = Race::parse_part_2(input);
        
        race.ways_to_win() as i32
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
