mod day;
mod select;

mod_days!(day_1, day_2, day_3, day_4);

use std::io;
use std::io::Read;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;

use day::Day;
use select::select;

fn match_day(day: u32) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
        3 => Box::new(Day3),
        4 => Box::new(Day4),
        _ => panic!("Invalid day selected, exiting..."),
    }
}

fn match_part(part: u32, day: Box<dyn Day>, input: &str) -> i32 {
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

    for day_num in 1..=4 {
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

    let _start = std::time::Instant::now();

    run_day(date, part, &input);
}
