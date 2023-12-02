mod day;
mod select;

mod_days!(day_1, day_2);

use std::io;
use std::io::Read;

use day_1::Day1;
use day_2::Day2;

use day::Day;
use select::select;

fn match_day(day: u32) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
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

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let (date, part) = if let Some(arg) = args.get(1) {
        if arg == "help" {
            println!("Usage: advent_2023 [day:part] [input]");
            std::process::exit(0);
        } else {
            let date_part = arg.split(':').collect::<Vec<&str>>();

            let date = date_part.first().unwrap().parse::<u32>().unwrap();
            let part = date_part.get(1).unwrap().parse::<u32>().unwrap();

            (date, part)
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

    let start = std::time::Instant::now();

    let result = match_part(part, day, &input);

    println!("Day {} Part {} Result: {}", date, part, result);

    println!("Done in {}ms", start.elapsed().as_millis());
}
