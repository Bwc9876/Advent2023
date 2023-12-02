use crate::{day::Day, get_input_for_day};

#[derive(Debug, PartialEq, Eq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn parse_section(raw: &str) -> (String, u32) {
        let raw = raw.trim().split(' ').collect::<Vec<&str>>();

        let number = raw.first().unwrap().parse::<u32>().unwrap();
        let color = raw.get(1).unwrap();

        (color.to_string(), number)
    }

    pub fn parse(raw: &str) -> Self {
        let sections = raw.split(", ").collect::<Vec<&str>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for section in sections {
            let (color, number) = Draw::parse_section(section);

            match color.as_str() {
                "red" => red += number,
                "green" => green += number,
                "blue" => blue += number,
                _ => panic!("Invalid color: {}", color),
            }
        }

        Draw { red, green, blue }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn parse(raw: &str) -> Self {
        let re = regex::Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re.captures(raw).unwrap();

        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let draws = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(Draw::parse)
            .collect::<Vec<Draw>>();

        Self { id, draws }
    }

    pub fn get_totals(&self) -> Vec<(u32, u32, u32)> {
        let mut totals = Vec::new();

        for draw in &self.draws {
            totals.push((draw.red, draw.green, draw.blue));
        }

        totals
    }

    pub fn get_totals_three_lists(&self) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let mut reds = Vec::new();
        let mut greens = Vec::new();
        let mut blues = Vec::new();

        for draw in &self.draws {
            reds.push(draw.red);
            greens.push(draw.green);
            blues.push(draw.blue);
        }

        (reds, greens, blues)
    }
}

pub struct Day2;

impl Day for Day2 {

    get_input_for_day!(2);

    fn part_1(&self, input: &str) -> i32 {    
        const MAXES: (u32, u32, u32) = (12, 13, 14);
        let mut total = 0;
    
        for line in input.lines() {
            let game = Game::parse(line);
            let draw_totals = game.get_totals();
    
            if draw_totals
                .iter()
                .all(|(red, green, blue)| red <= &MAXES.0 && green <= &MAXES.1 && blue <= &MAXES.2)
            {
                total += game.id;
            }
        }
        total.try_into().unwrap()
    }

    fn part_2(&self, input: &str) -> i32 {
        let mut total = 0;
    
        for line in input.lines() {
            let game = Game::parse(line);
            let draw_totals = game.get_totals_three_lists();
    
            let maxes = (
                draw_totals.0.iter().max().unwrap(),
                draw_totals.1.iter().max().unwrap(),
                draw_totals.2.iter().max().unwrap(),
            );
    
            let power = maxes.0 * maxes.1 * maxes.2;
    
            total += power;
        }
    
        total.try_into().unwrap()
    }    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_section() {
        let raw = "1 red";
        let expected = ("red".to_string(), 1);
        let actual = Draw::parse_section(raw);
        assert_eq!(expected, actual);

        let raw = "2 green";
        let expected = ("green".to_string(), 2);
        let actual = Draw::parse_section(raw);
        assert_eq!(expected, actual);

        let raw = "3 blue";
        let expected = ("blue".to_string(), 3);
        let actual = Draw::parse_section(raw);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_draw() {
        let raw = "1 red, 2 green, 3 blue";
        let expected = Draw {
            red: 1,
            green: 2,
            blue: 3,
        };
        let actual = Draw::parse(raw);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game() {
        let raw = "Game 1: 1 red, 2 green, 3 blue; 2 red, 3 green, 4 blue";
        let expected = Game {
            id: 1,
            draws: vec![
                Draw {
                    red: 1,
                    green: 2,
                    blue: 3,
                },
                Draw {
                    red: 2,
                    green: 3,
                    blue: 4,
                },
            ],
        };
        let actual = Game::parse(raw);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day_2_part_1() {
        let day = Day2;
        let input = day.get_input();
        let expected = 2164;
        let actual = day.part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day_2_part_2() {
        let day = Day2;
        let input = day.get_input();
        let expected = 69929;
        let actual = day.part_2(input);
        assert_eq!(expected, actual);
    }
}