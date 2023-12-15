use crate::{day::Day, get_input_for_day};

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| {
        let acc = acc + (c as u64);
        let acc = acc * 17;
        acc % 256
    })
}

enum Operation {
    Update(u64),
    Remove,
}

struct Step {
    label: String,
    address: u64,
    operation: Operation
}

impl Step {
    pub fn parse(input: &str) -> Self {
        if let Some(equal_pos) = input.find(|c| c == '=') {
            let label = input[..equal_pos].trim();
            let val = input[equal_pos + 1..].trim();
            Self {
                label: label.to_string(),
                address: hash(label),
                operation: Operation::Update(val.parse().unwrap()),
            }
        } else {
            let label = input[..input.len() - 1].trim();
            Self {
                label: label.to_string(),
                address: hash(label),
                operation: Operation::Remove,
            }
        }
    }
}

pub struct Day15;

impl Day for Day15 {

    get_input_for_day!(15);

    fn part_1(&self, input: &str) -> i64 {
        let split = input.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        split.iter().map(|s| hash(s)).sum::<u64>() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let split = input.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let steps = split.iter().map(|s| Step::parse(s)).collect::<Vec<_>>();

        let mut boxes: Vec<Vec<(String, u64)>> = vec![vec![]; 256];

        for step in steps {
            match step.operation {
                Operation::Update(val) => {
                    let address = step.address as usize;
                    let index = address % 256;
                    let key = step.label;
                    if let Some((_, old_val)) = boxes[index].iter_mut().find(|(k, _)| k == &key) {
                        *old_val = val;
                    } else {
                        boxes[index].push((key, val));
                    }
                }
                Operation::Remove => {
                    let address = step.address as usize;
                    let index = address % 256;
                    if let Some(pos) = boxes[index].iter().position(|(k, _)| k == &step.label) {
                        boxes[index].remove(pos);
                    }
                }
            }
        }

        boxes.iter().enumerate().map(|(bi, b)| {
            b.iter().enumerate().map(|(li, l)| {
                (bi + 1) * (li + 1) * (l.1 as usize)
            }).sum::<usize>()
        }).sum::<usize>() as i64
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day15;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 509167);
    }

    #[test]
    fn test_part_2() {
        let day = Day15;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 259333);
    }

}