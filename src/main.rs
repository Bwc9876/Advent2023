mod bootstrap;
mod day;
mod select;
mod utils;

mod_days!(
    day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11, day_12, day_13,
    day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21, day_22, day_23, day_24
);

use std::io;
use std::io::Read;

use day_1::Day1;
use day_10::Day10;
use day_11::Day11;
use day_12::Day12;
use day_13::Day13;
use day_14::Day14;
use day_15::Day15;
use day_16::Day16;
use day_17::Day17;
use day_18::Day18;
use day_19::Day19;
use day_2::Day2;
use day_20::Day20;
use day_21::Day21;
use day_22::Day22;
use day_23::Day23;
use day_24::Day24;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;
use day_7::Day7;
use day_8::Day8;
use day_9::Day9;

use day::Day;
use select::select;

fn match_day(day: u32) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
        3 => Box::new(Day3),
        4 => Box::new(Day4),
        5 => Box::new(Day5),
        6 => Box::new(Day6),
        7 => Box::new(Day7),
        8 => Box::new(Day8),
        9 => Box::new(Day9),
        10 => Box::new(Day10),
        11 => Box::new(Day11),
        12 => Box::new(Day12),
        13 => Box::new(Day13),
        14 => Box::new(Day14),
        15 => Box::new(Day15),
        16 => Box::new(Day16),
        17 => Box::new(Day17),
        18 => Box::new(Day18),
        19 => Box::new(Day19),
        20 => Box::new(Day20),
        21 => Box::new(Day21),
        22 => Box::new(Day22),
        23 => Box::new(Day23),
        24 => Box::new(Day24),
        _ => panic!("Invalid day selected, exiting..."),
    }
}

fn match_part(part: u32, day: Box<dyn Day>, input: &str) -> i64 {
    match part {
        1 => day.part_1(input),
        2 => day.part_2(input),
        _ => panic!("Invalid part selected, exiting..."),
    }
}

fn run_day(day_num: u32, part: u32, input: &str) {
    let day = match_day(day_num);

    let start = std::time::Instant::now();

    let result = match_part(part, day, input);

    println!(
        "Day {} Part {} Result: {} (in {}ms)",
        day_num,
        part,
        result,
        start.elapsed().as_millis()
    );
}

fn run_all_days() {
    let start = std::time::Instant::now();

    for day_num in 1..=24 {
        for part in 1..=2 {
            let day = match_day(day_num);

            let input = day.get_input();

            run_day(day_num, part, input);
        }
    }

    println!("Total time: {}ms", start.elapsed().as_millis());
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let (date, part) = if let Some(arg) = args.get(1) {
        match arg.trim() {
            "help" => {
                println!("Usage: advent_2023 [day:part] [input]");
                std::process::exit(0);
            }
            "bootstrap" => {
                let num = args
                    .get(2)
                    .and_then(|n| n.parse::<u32>().ok())
                    .unwrap_or_else(bootstrap::get_next_highest_day);
                bootstrap::bootstrap_file(num);
                println!("Created day_{}.rs", num);
                std::process::exit(0);
            }
            "*" => {
                run_all_days();
                std::process::exit(0);
            }
            _ => {
                let date_part = arg.split(':').collect::<Vec<&str>>();

                let date = date_part.first().unwrap().parse::<u32>().unwrap();
                let part = date_part.get(1).unwrap().parse::<u32>().unwrap();

                (date, part)
            }
        }
    } else {
        select()
    };

    let day = match_day(date);

    let input = if let Some(arg) = args.get(2) {
        if arg.trim() != "" {
            if arg == "-" {
                let mut input = String::new();

                io::stdin()
                    .read_to_string(&mut input)
                    .expect("Failed to read input");

                input.trim().to_string()
            } else {
                arg.to_string()
            }
        } else {
            day.get_input().to_string()
        }
    } else {
        day.get_input().to_string()
    };

    run_day(date, part, &input);
}
