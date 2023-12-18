use crate::{day::Day, get_input_for_day, utils::{Direction, dir::Movement}};

pub struct Instruction {
    dir: Direction,
    amount: usize,
}

impl Instruction {

    fn char_to_dir(c: char) -> Direction {
        match c {
            'U' => Direction::North,
            'D' => Direction::South,
            'L' => Direction::West,
            'R' => Direction::East,
            _ => panic!("Invalid direction char: {}", c)
        }
    }

    fn num_to_dir(n: u32) -> Direction {
        match n {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => panic!("Invalid direction number: {}", n)
        }
    }

    pub fn parse(input: &str, part_2: bool) -> Self {
        let split = input.split(' ').collect::<Vec<&str>>();

        let (dir, amount) = if part_2 {
            let color = split[2].trim_matches(|c| c == '(' || c ==')' || c == '#');
            let amount = u32::from_str_radix(&color[0..5], 16);
            let dir_num = color[5..6].parse::<u32>().unwrap();
            (Self::num_to_dir(dir_num), amount.unwrap() as usize)
        } else {
            let dir = Self::char_to_dir(split[0].chars().next().unwrap());
            let amount = split[1].parse::<usize>().unwrap();
            (dir, amount)
        };

        Self {
            dir,
            amount,
        }
    }

}

pub struct Day18;

impl Day for Day18 {

    get_input_for_day!(18);

    fn part_1(&self, input: &str) -> i64 {
        let instructions = input.lines().map(|line| Instruction::parse(line, false)).collect::<Vec<_>>();

        let start = (0, 0);

        let mut verts: Vec<(isize, isize)> = vec![start];

        let mut pos = start;

        for ins in instructions.into_iter() {
            let next_pos = ins.dir.add_to_pos_times_negative(pos, ins.amount as isize);
            pos = next_pos;
            verts.push(pos);
        }

        let area = verts.windows(2).map(|w| ((w[0].0) * (w[1].1)) - ((w[0].1) * (w[1].0))).sum::<isize>() / 2;

        let perimeter = verts.windows(2).map(|w| (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs()).sum::<isize>();

        ((area + 1) + (perimeter / 2)) as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let instructions = input.lines().map(|line| Instruction::parse(line, true)).collect::<Vec<_>>();

        let start = (0, 0);

        let mut verts: Vec<(isize, isize)> = vec![start];

        let mut pos = start;

        for ins in instructions.into_iter() {
            let next_pos = ins.dir.add_to_pos_times_negative(pos, ins.amount as isize);
            pos = next_pos;
            verts.push(pos);
        }

        let area = verts.windows(2).map(|w| ((w[0].0) * (w[1].1)) - ((w[0].1) * (w[1].0))).sum::<isize>() / 2;

        let perimeter = verts.windows(2).map(|w| (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs()).sum::<isize>();

        ((area + 1) + (perimeter / 2)) as i64
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day18;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 50603);
    }

    #[test]
    fn test_part_2() {
        let day = Day18;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 96556251590677);
    }

}