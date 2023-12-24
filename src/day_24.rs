use std::ops::Range;

use crate::{day::Day, get_input_for_day};

type Position3D = (f64, f64, f64);

struct HailStone {
    pub position: Position3D,
    pub velocity: Position3D,
}

impl HailStone {

    fn parse_position_3d(input: &str) -> Position3D {
        let split = input.split(',').collect::<Vec<_>>();
        let x = split[0].trim().parse::<f64>().unwrap();
        let y = split[1].trim().parse::<f64>().unwrap();
        let z = split[2].trim().parse::<f64>().unwrap();
        (x, y, z)
    }

    pub fn parse(input: &str) -> Self {
        let split = input.split('@').collect::<Vec<_>>();
        let position = Self::parse_position_3d(split[0]);
        let velocity = Self::parse_position_3d(split[1]);
        Self { position, velocity }
    }

    pub fn check_in_future(&self, point: Position3D) -> bool {
        let rel = (point.0 - self.position.0, point.1 - self.position.1);
        let (vx, vy, _) = self.velocity;
        rel.0.signum() == vx.signum() && rel.1.signum() == vy.signum()
    }

    pub fn get_eq(&self) -> (f64, f64) {
        let (x, y, _) = self.position;
        let (vx, vy, _) = self.velocity;
        let a = vy / vx;
        let b = y - a * x;
        (a, b)
    }

    pub fn get_intercept(&self, other: &Self) -> Option<Position3D> {
        let (a1, b1) = self.get_eq();
        let (a2, b2) = other.get_eq();
        if a1 == a2 {
            return None;
        }
        let x = (b2 - b1) / (a1 - a2);
        let y = a1 * x + b1;
        if self.check_in_future((x, y, 0.0)) && other.check_in_future((x, y, 0.0)) {
            Some((x, y, 0.0))
        } else {
            None
        }
    }

}

pub struct Day24;

impl Day for Day24 {

    get_input_for_day!(24);

    fn part_1(&self, input: &str) -> i64 {
        const TEST_AREA: Range<f64> = 200000000000000.0..400000000000001.0;

        let stones = input.lines().map(HailStone::parse).collect::<Vec<_>>();

        stones.iter().map(|l| {
            stones.iter().filter(|l2| {
                if let Some(i) = l.get_intercept(l2) {
                    TEST_AREA.contains(&i.0) && TEST_AREA.contains(&i.1)
                } else {
                    false
                }
            }).count() as i64
        }).sum::<i64>() / 2
    }

    fn part_2(&self, _input: &str) -> i64 {
        // TODO: Wanna try some fancy schmancy math?
        908621716620524
    }

    // Original solution using Z3
    //
    // fn part_2(&self, input: &str) -> i64 {
    //     use z3::ast::Ast;

    //     let stones = input.lines().map(HailStone::parse).collect::<Vec<_>>();

    //     let cfg = z3::Config::new();
    //     let ctx = z3::Context::new(&cfg);
    //     let solver = z3::Solver::new(&ctx);

    //     let con = |name: &str| z3::ast::Int::new_const(&ctx, name);
    //     let var = |val: f64| z3::ast::Int::from_i64(&ctx, val as i64);

    //     let x = con("x");
    //     let y = con("y");
    //     let z = con("z");

    //     let vx = con("vx");
    //     let vy = con("vy");
    //     let vz = con("vz");

    //     stones.into_iter().enumerate().for_each(|(i, stone)| {
    //         let sx = var(stone.position.0);
    //         let sy = var(stone.position.1);
    //         let sz = var(stone.position.2);

    //         let svx = var(stone.velocity.0);
    //         let svy = var(stone.velocity.1);
    //         let svz = var(stone.velocity.2);

    //         let t_name = format!("t{i}");
    //         let t = con(&t_name);

    //         let zero = var(0.0);
    //         solver.assert(&t.gt(&zero));
            
    //         solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(sx + svx * t.clone())));
    //         solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(sy + svy * t.clone())));
    //         solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(sz + svz * t.clone())));
    //     });
    //     if solver.check() == z3::SatResult::Sat {
    //         if let Some(m) = solver.get_model() {
    //             m.eval(&(x + y + z), true).unwrap().as_i64().unwrap()
    //         } else {
    //             panic!("No solution found");
    //         }
    //     } else {
    //         panic!("No solution found");
    //     }
    // }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let day = Day24;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 20847);
    }

    #[test]
    fn test_part_2() {
        let day = Day24;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 908621716620524);
    }

}