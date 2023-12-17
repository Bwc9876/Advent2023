use std::collections::HashMap;

use crate::{day::Day, get_input_for_day, utils::Grid};

pub struct Day3;

const EMPTY_CHAR: char = '.';

type Manual = Grid<char>;

fn str_to_2d_map(input: &str) -> Manual {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for line in map.iter_mut() {
        line.push(EMPTY_CHAR); // Pad with a . at the end so numbers at the ends get processed
    }

    Manual::new(map)
}

fn check_char(char: char) -> bool {
    !char.is_ascii_digit() && char != EMPTY_CHAR
}

type AdjacentCharMap = Vec<((usize, usize), char)>;

fn get_adjacent_chars(input_map: &Manual, row: usize, starting_index: usize, ending_index: usize) -> AdjacentCharMap {
    let mut chars: AdjacentCharMap = vec![];

    let start_minus_one = starting_index.saturating_sub(1);

    if let Some(next_on_this_level) = input_map.get_row(row).and_then(|r| r.get(start_minus_one).copied()) {
        chars.push(((row, start_minus_one), *next_on_this_level));
    }

    if let Some(prev_on_this_level) = input_map.get_row(row).and_then(|r| r.get(ending_index + 1).copied()) {
        chars.push(((row, ending_index + 1), *prev_on_this_level));
    }

    let above = if row == 0 { None } else { input_map.get_row(row - 1) };
    let below = input_map.get_row(row + 1);

    for current_col in start_minus_one..ending_index + 2 {
        if let Some(above_row) = above.as_ref().and_then(|r| r.get(current_col).copied()).copied() {
            chars.push(((row - 1, current_col), above_row));
        }
        if let Some(below_row) = below.as_ref().and_then(|r| r.get(current_col).copied()).copied(){
            chars.push(((row + 1, current_col), below_row));
        }
    }

    chars
}

impl Day for Day3 {

    get_input_for_day!(3);

    fn part_1(&self, input: &str) -> i64 {
        
        let input_map = str_to_2d_map(input);

        input_map.iter_rows().enumerate().map(|(x, r)| {
            let mut start_index = 0;
            let mut current_num = String::new();
            let mut total = 0;

            for (y, char) in r.iter().enumerate() {
                if char.is_ascii_digit() {
                    current_num.push(**char);
                    if current_num.len() == 1 {
                        start_index = y;
                    }
                } else {

                    if current_num.is_empty() {
                        continue;
                    }

                    let chars_to_check = get_adjacent_chars(
                        &input_map, 
                        x, 
                        start_index, 
                        y - 1
                    );

                    if chars_to_check.iter().any(|char| check_char(char.1)) {
                        total += current_num.clone().parse::<i64>().unwrap();
                    }

                    current_num.clear();
                    start_index = 0;

                }
            }

            total
        }).sum::<i64>()
    }

    fn part_2(&self, input: &str) -> i64 {
        
        let input_map = str_to_2d_map(input);
    
        let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for (i, line) in input_map.iter_rows().enumerate() {

            let mut start_index = 0;
            let mut current_num = String::new();

            for (j, char) in line.iter().enumerate() {
                if char.is_ascii_digit() {
                    current_num.push(**char);
                    if current_num.len() == 1 {
                        start_index = j;
                    }
                } else {

                    if current_num.is_empty() {
                        continue;
                    }

                    let chars_to_check = get_adjacent_chars(
                        &input_map, 
                        i, 
                        start_index, 
                        j - 1
                    );

                    for c in chars_to_check.iter() {
                        if c.1 == '*' {
                            if let Some(star) = stars.get_mut(&c.0) {
                                star.push(current_num.clone().parse::<u32>().unwrap());
                            } else {
                                stars.insert(c.0, vec![current_num.clone().parse::<u32>().unwrap()]);
                            }
                        }
                    }

                    current_num.clear();
                    start_index = 0;

                }
            }
        }

        let total: u32 = stars.values()
            .filter_map(|s| {
                if s.len() == 2 {
                    Some(s[0] * s[1])
                } else {
                    None
                }
            }).sum();

        total as i64
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adjacent_chars() {
        let input = "1.2.3.4.5.6...\n7.8.9.10.11.12\n13.14.15.16.17";

        let input_map = str_to_2d_map(input);

        let target = (1, 6, 7);

        let expected = vec![
            ((1, 5), '.'),
            ((1, 8), '.'),
            ((0, 5), '.'),
            ((2,5), '.'),
            ((0, 6), '4'),
            ((2,6), '1'),
            ((0,7), '.'),
            ((2,7), '5'),
            ((0,8), '5'),
            ((2,8), '.')
        ];

        let actual = get_adjacent_chars(&input_map, target.0, target.1, target.2);

        assert_eq!(expected, actual);

    }

    #[test]
    fn test_part_1() {
        let day = Day3;
        let input = day.get_input();
        let expected = 550064;
        let actual = day.part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let day = Day3;
        let input = day.get_input();
        let expected = 85010461;
        let actual = day.part_2(input);
        assert_eq!(expected, actual);
    }

}

        
