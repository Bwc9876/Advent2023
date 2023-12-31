use std::ops::Range;

use crate::{day::Day, get_input_for_day, utils::parse_padded_numbers};


pub struct Day5;

#[derive(Debug)]
struct MapRow {
    dest_range: std::ops::Range<i64>,
    source_range: std::ops::Range<i64>,
}

impl MapRow {

    pub fn parse(input: &str) -> Self {
        let nums = parse_padded_numbers(input);
        let dest_start = nums[0];
        let source_start = nums[1];
        let length = nums[2];
        Self {
            dest_range: dest_start..dest_start+length,
            source_range: source_start..source_start+length,
        }
    }

    // Here's a little remnant of my descent into madness:
    // Range: ()
    // Source: []
    // Dest: {}
    // (1 2 [3 4) 5 6] {7 8 9 10}
    // [1 2 (3 4] 5 6) {7 8 9 10}
    // (1 [2 3 4] 5) 6 7 {8 9 10}
    // [1 (2 3 4) 5] 6 7 {8 9 10}

    pub fn in_range(&self, num: i64) -> bool {
        self.source_range.contains(&num)
    }

    pub fn map_range_split(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let start_dest_diff: i64 = self.dest_range.start - self.source_range.start;
        let range_has_source_start = range.contains(&self.source_range.start);
        let range_has_source_end = range.contains(&(self.source_range.end - 1));

        if range == self.source_range {
            vec![self.dest_range.clone()]
        } else if range_has_source_start && range_has_source_end {
            vec![range.start..self.source_range.start,
                self.dest_range.start..self.dest_range.end,
                self.source_range.end..range.end]
        } else if self.source_range.contains(&range.start) && self.source_range.contains(&(range.end - 1)) {
            // Stop clippy from complaining about doing a Vec<Range> with only one element (clippy::single_range_in_vec_init)
            std::iter::once(range.start + start_dest_diff..range.end + start_dest_diff).collect()
        }
        else if range_has_source_start {
            vec![range.start..self.source_range.start, self.dest_range.start..range.end+start_dest_diff]
        } else if range_has_source_end {
            vec![self.source_range.end..range.end, range.start+start_dest_diff..self.dest_range.end]
        } else {
            vec![]
        }
    }

}

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>,
}

impl Map {

    pub fn parse(input: &str) -> Self {
        let rows = input.lines().map(MapRow::parse).collect();
        Self {
            rows,
        }
    }

    pub fn parse_all(lines: Vec<&str>) -> Vec<Self> {
        let mut lines = lines;

        lines.push("map:");

        let mut maps: Vec<Map> = Vec::new();

        let mut current_map_input: Vec<&str> = vec![];

        let len = lines.len();

        for (i, line) in lines.into_iter().skip(1).enumerate() {
            if line.contains("map:") || i == len - 1 {
                let map = Map::parse(current_map_input.join("\n").as_str());
                if map.has_rows() {
                    maps.push(map);
                }
                current_map_input = vec![];
            } else if !line.is_empty() {
                current_map_input.push(line);
            }
        }

        maps
    }

    pub fn map(&self, in_num: i64) -> i64 {
        for map in &self.rows {
            if map.in_range(in_num) {
                return map.dest_range.start + (in_num - map.source_range.start);
            }
        }
        in_num
    }

    pub fn has_rows(&self) -> bool {
        !self.rows.is_empty()
    }

    pub fn map_ranges(&self, ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut mapped_ranges = vec![];
        let mut ranges_mapped = vec![false;ranges.len()];
        for map in &self.rows {
            for (i, range) in ranges.iter().enumerate() {
                let mapped = map.map_range_split(range.clone());
                if !mapped.is_empty() {
                    ranges_mapped[i] = true;
                    mapped_ranges.extend(mapped);
                }
            }
        }

        if ranges_mapped.contains(&false) {
            mapped_ranges.extend(ranges_mapped.into_iter().enumerate().filter_map(|(i, mapped)| if mapped { None } else { Some(ranges[i].clone()) }).collect::<Vec<Range<i64>>>());
        }
        
        mapped_ranges
    }

}

impl Day for Day5 {

    get_input_for_day!(5);

    fn part_1(&self, input: &str) -> i64 {
        let mut lines = input.lines().collect::<Vec<&str>>();

        lines.push("map:");

        let seeds = parse_padded_numbers(lines.first().unwrap().split(": ").nth(1).unwrap());

        let maps = Map::parse_all(lines);

        let locations = seeds.iter().map(|s| {
            maps.iter().fold(*s, |num, map| map.map(num))
        }).collect::<Vec<i64>>();

        *locations.iter().min().unwrap() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let lines = input.lines().collect::<Vec<&str>>();

        let seeds = parse_padded_numbers(lines.first().unwrap().split(": ").nth(1).unwrap());

        let mut ranges = vec![];
        let mut start = 0;

        for (i, s) in seeds.iter().enumerate() {
            if i % 2 == 1 {
                ranges.push(start..start+*s);
            } else {
                start = *s;
            }
        }

        let maps = Map::parse_all(lines);

        let mut starts = vec![];

        for range in ranges.into_iter() {
            let mapped_ranges: Vec<Range<i64>> = maps.iter().fold(vec![range.clone()], |ranges, map| map.map_ranges(ranges));
            let mut _starts = mapped_ranges.into_iter().filter_map(|r| if r.start == 0 { None } else { Some(r.start) }).collect::<Vec<i64>>();
            starts.append(&mut _starts);
        }

        *starts.iter().min().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let day = super::Day5;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 174137457);
    }

    #[test]
    fn test_part_2() {
        let day = super::Day5;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 1493866);
    }

}