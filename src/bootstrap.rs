pub fn bootstrap_file(number: u32) {
    let file_name = format!("src/inputs/day_{}.txt", number);

    if std::path::Path::new(&file_name).exists() {
        println!("File {} already exists", file_name);
        std::process::exit(1);
    }

    std::fs::write(file_name, "").expect("Failed to write file");

    let template = include_str!("template.rs");

    let template = template.replace("_n_", &number.to_string());

    let file_name = format!("src/day_{}.rs", number);

    if std::path::Path::new(&file_name).exists() {
        println!("File {} already exists", file_name);
        std::process::exit(1);
    }

    std::fs::write(file_name, template).expect("Failed to write file");

    let main_file = include_str!("main.rs");

    // Let's do smth a bit silly

    let yesterday = number - 1;

    let main_file = main_file.replace(
        &format!(", day_{yesterday}"),
        &format!(", day_{yesterday}, day_{number}"),
    );
    let main_file = main_file.replace(
        &format!("use day_{yesterday}::Day{yesterday};"),
        &format!("use day_{yesterday}::Day{yesterday};\nuse day_{number}::Day{number};"),
    );
    let main_file = main_file.replace(
        &format!("{yesterday} => Box::new(Day{yesterday}),"),
        &format!(
            "{yesterday} => Box::new(Day{yesterday}),\n\t\t{number} => Box::new(Day{number}),"
        ),
    );
    let main_file = main_file.replace(&format!("..={yesterday}"), &format!("..={number}"));

    std::fs::write("src/main.rs", main_file).expect("Failed to write file");
}

pub fn get_next_highest_day() -> u32 {
    std::fs::read_dir("src")
        .unwrap()
        .filter_map(|p| {
            let p = p.unwrap().path();
            let p = p
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap();
            p.strip_prefix("day_").map(|p| p.parse::<u32>().unwrap())
        })
        .max()
        .unwrap_or(0)
        + 1
}
