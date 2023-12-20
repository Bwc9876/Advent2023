use std::{collections::HashMap, cmp::{max, min}};

use crate::{day::Day, get_input_for_day};

#[derive(Debug)]
pub enum ModuleLogic {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

impl ModuleLogic {

    pub fn broadcaster() -> Self {
        Self::Broadcaster
    }

    pub fn flip_flop() -> Self {
        Self::FlipFlop(false)
    }

    pub fn conjunction() -> Self {
        Self::Conjunction(HashMap::new())
    }

    pub fn unwrap_conj(&self) -> &HashMap<String, bool> {
        match self {
            Self::Conjunction(inputs) => inputs,
            _ => panic!("Not a conjunction")
        }
    }

    pub fn init_conj(&mut self, id: &str, modules: &[(String, Vec<String>)]) {
        if let Self::Conjunction(inputs) = self {
            *inputs = modules.iter().filter_map(|(m, targets)| {
                if targets.contains(&id.to_string()) {
                    Some((m.clone(), false))
                } else {
                    None
                }
            }).collect();
        }
    }

    pub fn process(&mut self, from: String, freq: bool) -> Option<bool> {
        match self {
            Self::Broadcaster => Some(freq),
            Self::FlipFlop(state) => {
                if freq {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            },
            Self::Conjunction(inputs) => {
                inputs.insert(from, freq);
                Some(!inputs.values().all(|v| *v))
            }
        }
    }

}

#[derive(Debug)]
pub struct Module {
    pub id: String,
    logic: ModuleLogic,
    pub targets: Vec<String>
}

impl Module {

    pub fn parse(input: &str) -> Self {

        let mut parts = input.split(" -> ");
        
        let header = parts.next().unwrap();

        let logic_char = header.chars().nth(0).unwrap();

        let logic = match logic_char {
            '%' => ModuleLogic::flip_flop(),
            '&' => ModuleLogic::conjunction(),
            _ => ModuleLogic::broadcaster()
        };

        let id_range = match logic_char {
            '%' | '&' => 1..,
            _ => 0..
        };

        let id = header[id_range].to_string();

        let targets = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect();

        Self {
            id,
            logic,
            targets
        }
    }

    pub fn process(&mut self, from: String, freq: bool) -> Option<bool> {
        self.logic.process(from, freq)
    }

}

fn greatest_common_denominator(a: i64, b: i64) -> i64 {
    let mut max = max(a, b);
    let mut min = min(a, b);

    while min != 0 {
        let hold = min;
        min = max % min;
        max = hold;
    }

    max
}

fn least_common_denominator(a: i64, b: i64) -> i64 {
    (a * b) / greatest_common_denominator(a, b)
}

pub struct Day20;

impl Day for Day20 {

    get_input_for_day!(20);

    fn part_1(&self, input: &str) -> i64 {
        
        const TIMES: usize = 1000;
        
        let modules = input.lines().map(Module::parse).collect::<Vec<_>>();

        let name_targets = modules.iter().map(|m| (m.id.clone(), m.targets.clone())).collect::<Vec<_>>();

        let mut modules = modules.into_iter().map(|mut m| {
            m.logic.init_conj(&m.id, &name_targets);
            (m.id.to_string(), m)
        }).collect::<HashMap<_, _>>();

        let mut queue: Vec<(String, String, bool)> = vec![];

        let mut sent = (0, 0);

        for _ in 1..=TIMES {
            queue.push(("button".to_string(), "broadcaster".to_string(), false));

            while let Some((from, target, freq)) = queue.pop() {
                if freq {
                    sent.1 += 1;
                } else {
                    sent.0 += 1;
                }

                if let Some(module) = modules.get_mut(&target) {
                    if let Some(freq) = module.process(from, freq) {
                        for t in module.targets.iter() {
                            queue.push((target.clone(), t.clone(), freq));
                        }
                    }
                }
            }
        }

        sent.0 * sent.1
    }

    fn part_2(&self, input: &str) -> i64 {        
        let modules = input.lines().map(Module::parse).collect::<Vec<_>>();

        let name_targets = modules.iter().map(|m| (m.id.clone(), m.targets.clone())).collect::<Vec<_>>();

        let mut modules = modules.into_iter().map(|mut m| {
            m.logic.init_conj(&m.id, &name_targets);
            (m.id.to_string(), m)
        }).collect::<HashMap<_, _>>();

        let silly_goose = name_targets.into_iter().find(|(_, targets)| targets.contains(&"rx".to_string())).unwrap().0;

        let sources = modules.get(&silly_goose).unwrap().logic.unwrap_conj().keys().cloned().collect::<Vec<_>>();

        let mut cycles_per = sources.iter().map(|k| (k, vec![])).collect::<HashMap<_, Vec<usize>>>();

        let mut i = 1;

        let mut queue: Vec<(String, String, bool)> = vec![];

        while cycles_per.values().any(|v| v.len() != 2) {
            queue.push(("button".to_string(), "broadcaster".to_string(), false));

            while let Some((from, target, freq)) = queue.pop() {
                if freq && target == silly_goose {
                    let cycles_for = cycles_per.get_mut(&from).unwrap();
                    if cycles_for.len() < 2 {
                        cycles_for.push(i);
                    }
                }
                if let Some(module) = modules.get_mut(&target) {
                    if let Some(freq) = module.process(from, freq) {
                        for t in module.targets.iter() {
                            queue.push((target.clone(), t.clone(), freq));
                        }
                    }
                }
            }

            i += 1;
        }

        cycles_per.values().map(|v| (v[1] - v[0]) as i64).fold(1, least_common_denominator)

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day20;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 825896364);
    }

    #[test]
    fn test_part_2() {
        let day = Day20;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 243566897206981);
    }

}