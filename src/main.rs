mod day_1;
mod day_2;

use chrono::{NaiveDate, Weekday, prelude::*};
use inquire::{Select, DateSelect};

fn select_date() -> u32 {
    let today = Local::now().date_naive();
    let date = DateSelect::new("Select Day To Run")
    .with_starting_date(today)
    .with_min_date(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap())
    .with_max_date(NaiveDate::from_ymd_opt(2023, 12, 25).unwrap())
    .with_week_start(Weekday::Sun)
    .prompt();

    if let Ok(date) = date {
        date.day0() + 1
    } else {
        println!("No date selected, exiting...");
        std::process::exit(1);
    }
}

fn select_part() -> u32 {
    let part = Select::new("Select Part To Run", vec![1, 2]).prompt();

    if let Ok(part) = part {
        part
    } else {
        println!("No part selected, exiting...");
        std::process::exit(1);
    }
}

fn select() -> (u32, u32) {
    (select_date(), select_part())
}

fn main() {
    let (date, part) = select();
    
    match date {
        1 => match part {
            1 => day_1::part_1(),
            2 => day_1::part_2(),
            _ => println!("Invalid part selected, exiting..."),
        },
        2 => match part {
            1 => day_2::part_1(),
            2 => day_2::part_2(),
            _ => println!("Invalid part selected, exiting..."),
        },
        _ => println!("Invalid date selected, exiting..."),
    }
}
