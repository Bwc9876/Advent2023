use std::{collections::HashMap, ops::Range};

use crate::{day::Day, get_input_for_day};

type FactorVal = u32;

type PossibleMap = Vec<HashMap<PartFactor, Range<FactorVal>>>;

const MAX: FactorVal = 4000;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum PartFactor {
    Cool,
    Musical,
    Aerodynamic,
    Shiny
}

impl PartFactor {
    fn parse(c: char) -> Self {
        match c {
            'x' => Self::Cool,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Invalid part factor {}", c)
        }
    }

    fn possible_map() -> HashMap<PartFactor, Range<FactorVal>> {
        vec![
            (Self::Cool, 1..MAX+1),
            (Self::Musical, 1..MAX+1),
            (Self::Aerodynamic, 1..MAX+1),
            (Self::Shiny, 1..MAX+1)
        ].into_iter().collect()
    }
}

#[derive(Clone, Debug)]
enum WorkflowResult {
    Accept,
    Reject,
    Jump(String)
}

impl WorkflowResult {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Jump(s.to_string())
        }
    }
}

#[derive(Debug)]
enum Check {
    Greater,
    Less,
}

impl Check {
    fn parse(c: char) -> Self {
        match c {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("Invalid check {}", c)
        }
    }
}

#[derive(Debug)]
struct Condition {
    check: Check,
    factor: PartFactor,
    value: FactorVal
}

impl Condition {

    fn parse(input: &str) -> Self {
        let chars = input.chars().collect::<Vec<char>>();
        let factor = PartFactor::parse(chars[0]);
        let check = Check::parse(chars[1]);
        let value = chars[2..].iter().collect::<String>().parse::<FactorVal>().unwrap();

        Self {
            check,
            factor,
            value
        }
    }

    fn evaluate(&self, part: &Part) -> bool {
        let factor = part.factors.get(&self.factor).unwrap_or(&0);
        match self.check {
            Check::Greater => factor > &self.value,
            Check::Less => factor < &self.value,
        }
    }

    fn apply_to_range(&self, range: &Range<FactorVal>) -> Range<FactorVal> {
        match self.check {
            Check::Greater => self.value+1..range.end,
            Check::Less => range.start..self.value,
        }
    }

    fn apply_to_range_rev(&self, range: &Range<FactorVal>) -> Range<FactorVal> {
        match self.check {
            Check::Greater => range.start..self.value+1,
            Check::Less => self.value..range.end,
        }
    }

}

#[derive(Debug)]
struct WorkflowStep {
    cond: Option<Condition>,
    result: WorkflowResult
}

impl WorkflowStep {

    fn parse(input: &str) -> Self {
        if input.contains(':') {
            let split = input.split(':').collect::<Vec<&str>>();
            let cond = Condition::parse(split[0]);
            let result = WorkflowResult::parse(split[1]);
            Self {
                cond: Some(cond),
                result
            }
        } else {
            Self {
                cond: None,
                result: WorkflowResult::parse(input)
            }
        }
    }

    fn process(&self, part: &Part) -> Option<WorkflowResult> {
        match &self.cond {
            Some(cond) => {
                if cond.evaluate(part) {
                    Some(self.result.clone())
                } else {
                    None
                }
            },
            None => Some(self.result.clone())
        }
    }

    fn dfs_accepted(&self, possible: PossibleMap, workflows: &HashMap<String, Workflow>, rest: Option<Vec<&Self>>) -> Option<PossibleMap> {
        match &self.cond {
            Some(cond) => {       
                let mut new_possible = possible.clone();
                let t_possible = new_possible.get_mut(0).unwrap();
                let target = t_possible.get_mut(&cond.factor).unwrap();
                *target = cond.apply_to_range(target);

                let mut new_possible_rev = possible.clone();
                let t_possible_rev = new_possible_rev.get_mut(0).unwrap();
                let target_rev = t_possible_rev.get_mut(&cond.factor).unwrap();
                *target_rev = cond.apply_to_range_rev(target_rev);

                let rest = rest.unwrap(); // If we have a condition, we must have a rest

                let true_branch = match &self.result {
                    WorkflowResult::Accept => {
                        Some(new_possible)
                    }
                    WorkflowResult::Reject => {
                        None
                    }
                    WorkflowResult::Jump(target) => {
                        let workflow = workflows.get(target).unwrap();
                        workflow.steps[0].dfs_accepted(new_possible, workflows, Some(workflow.steps[1..].iter().collect()))
                    }
                };

                let false_branch = rest[0].dfs_accepted(new_possible_rev, workflows, Some(rest[1..].to_vec()));

                match (true_branch, false_branch) {
                    (Some(t), Some(f)) => {
                        Some(t.into_iter().chain(f).collect::<Vec<_>>())
                    },
                    (Some(true_branch), None) => Some(true_branch),
                    (None, Some(false_branch)) => Some(false_branch),
                    (None, None) => None
                } 
            },
            None => {
                match &self.result {
                    WorkflowResult::Accept => {
                        Some(possible)
                    }
                    WorkflowResult::Reject => {
                        None
                    }
                    WorkflowResult::Jump(target) => {
                        let workflow = workflows.get(target).unwrap();
                        workflow.steps[0].dfs_accepted(possible, workflows, Some(workflow.steps[1..].iter().collect()))
                    }
                }
            }
        }
    }

}

struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>
}

impl Workflow {

    fn parse(input: &str) -> Self {
        let split = input.split('{').collect::<Vec<&str>>();
        let name = split[0].trim().to_string();

        let steps = split[1].trim_end_matches('}').split(',').map(WorkflowStep::parse).collect();

        Self {
            name,
            steps
        }
    }

    fn process(&self, part: &Part) -> Option<WorkflowResult> {
        for step in &self.steps {
            if let Some(result) = step.process(part) {
                return Some(result);
            }
        }
        None
    }

}

struct Part {
    pub factors: HashMap<PartFactor, FactorVal>
}

impl Part {

    fn parse(input: &str) -> Self {
        let input = input.trim_matches(|c| c == '{' || c =='}');

        let pairs = input.split(',').map(|p| {
            let split = p.split('=').collect::<Vec<&str>>();
            let factor = PartFactor::parse(split[0].chars().next().unwrap());
            let value = split[1].parse::<FactorVal>().unwrap();

            (factor, value)
        }).collect();

        Self {
            factors: pairs
        }
    }

}

pub struct Day19;

impl Day for Day19 {

    get_input_for_day!(19);
    
    fn part_1(&self, input: &str) -> i64 {
        let split = input.split("\n\n").collect::<Vec<&str>>();

        let workflows = split[0].split('\n').map(Workflow::parse).map(|w| (w.name.clone(), w)).collect::<HashMap<_, _>>();
        let parts = split[1].split('\n').map(Part::parse).collect::<Vec<Part>>();

        let accepted = parts.iter().filter(|p| {
            let mut workflow = workflows.get("in").unwrap();

            loop {
                match workflow.process(p) {
                    Some(WorkflowResult::Accept) => return true,
                    Some(WorkflowResult::Reject) => return false,
                    Some(WorkflowResult::Jump(name)) => {
                        workflow = workflows.get(&name).unwrap();
                    },
                    None => return false
                }
            }
        });

        accepted.map(|p| {
            p.factors.values().sum::<FactorVal>() as i64
        }).sum()
    }

    fn part_2(&self, input: &str) -> i64 {
                
        let split = input.split("\n\n").collect::<Vec<&str>>();

        let workflows = split[0].split('\n').map(Workflow::parse).map(|w| (w.name.clone(), w)).collect::<HashMap<_, _>>();
        
        let possible = vec![PartFactor::possible_map()];

        let workflow = workflows.get("in").unwrap();

        let accepted = workflow.steps[0].dfs_accepted(possible, &workflows, Some(workflow.steps[1..].iter().collect())).unwrap();

        accepted.iter().map(|p| {
            p.values().map(|r| (r.end - r.start) as i64).product::<i64>()
        }).sum::<i64>()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day19;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 377025);
    }

    #[test]
    fn test_part_2() {
        let day = Day19;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 135506683246673);
    }

}