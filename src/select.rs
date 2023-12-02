use chrono::{prelude::*, NaiveDate, Weekday};
use inquire::{DateSelect, Select};

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

pub fn select() -> (u32, u32) {
    (select_date(), select_part())
}
