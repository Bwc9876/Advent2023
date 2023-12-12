pub fn bootstrap_file(number: u32) {
    let template = include_str!("template.rs");

    let template = template.replace("_n_", &number.to_string());

    let file_name = format!("src/day_{}.rs", number);

    if std::path::Path::new(&file_name).exists() {
        println!("File {} already exists", file_name);
        std::process::exit(1);
    }

    std::fs::write(file_name, template).expect("Failed to write file");
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
