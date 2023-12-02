#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn parse_section(raw: &str) -> (String, u32) {
        let raw = raw.trim().split(' ').collect::<Vec<&str>>();

        let number = raw.first().unwrap().parse::<u32>().unwrap();
        let color = raw.get(1).unwrap();

        (color.to_string(), number)
    }

    pub fn parse(raw: &str) -> Self {
        let sections = raw.split(", ").collect::<Vec<&str>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for section in sections {
            let (color, number) = Draw::parse_section(section);

            match color.as_str() {
                "red" => red += number,
                "green" => green += number,
                "blue" => blue += number,
                _ => panic!("Invalid color: {}", color),
            }
        }

        Draw { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn parse(raw: &str) -> Self {
        let re = regex::Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re.captures(raw).unwrap();

        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let draws = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(Draw::parse)
            .collect::<Vec<Draw>>();

        Self { id, draws }
    }

    pub fn get_totals(&self) -> Vec<(u32, u32, u32)> {
        let mut totals = Vec::new();

        for draw in &self.draws {
            totals.push((draw.red, draw.green, draw.blue));
        }

        totals
    }

    pub fn get_totals_three_lists(&self) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let mut reds = Vec::new();
        let mut greens = Vec::new();
        let mut blues = Vec::new();

        for draw in &self.draws {
            reds.push(draw.red);
            greens.push(draw.green);
            blues.push(draw.blue);
        }

        (reds, greens, blues)
    }
}

pub fn part_1() {
    let input = include_str!("inputs/day_2.txt");

    let maxes = (12, 13, 14);

    let mut total = 0;

    for line in input.lines() {
        let game = Game::parse(line);
        let draw_totals = game.get_totals();

        if draw_totals
            .iter()
            .all(|(red, green, blue)| red <= &maxes.0 && green <= &maxes.1 && blue <= &maxes.2)
        {
            total += game.id;
        }
    }

    println!("Total: {}", total);
}

pub fn part_2() {
    let input = include_str!("inputs/day_2.txt");

    let mut total = 0;

    for line in input.lines() {
        let game = Game::parse(line);
        let draw_totals = game.get_totals_three_lists();

        let maxes = (
            draw_totals.0.iter().max().unwrap(),
            draw_totals.1.iter().max().unwrap(),
            draw_totals.2.iter().max().unwrap(),
        );

        let power = maxes.0 * maxes.1 * maxes.2;

        total += power;
    }

    println!("Total: {}", total);
}
