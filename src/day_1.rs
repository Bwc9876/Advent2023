fn number_word_to_digit(number_word: &str) -> &str {
    let pos = NUMBERS.iter().position(|&x| x == number_word).unwrap();
    if pos < 9 {
        &NUMBERS[pos + 9]
    } else {
        number_word
    }
}

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

fn get_first_last(search: &str, include_words: bool) -> (String, String) {

    let mut first: (Option<usize>, String) = (None, String::new());
    let mut last: (Option<usize>, String) = (None, String::new());

    let numbers = if include_words {
        Vec::from(NUMBERS)
    } else {
        Vec::from(&NUMBERS[9..])
    };

    for number in numbers {
        let pos = search.find(number);
        if let Some(pos) = pos {
            if first.0.map(|f| pos < f).unwrap_or(true) {
                first = (Some(pos), number_word_to_digit(number).to_string());
            }
        }
        let pos = search.rfind(number);
        if let Some(pos) = pos {
            if last.0.map(|f| pos > f).unwrap_or(true) {
                last = (Some(pos), number_word_to_digit(number).to_string());
            }
        }
    }

    (first.1, last.1)
}

pub fn part_1() {
    let input = include_str!("inputs/day_1.txt");

    let total: i32 = input
        .lines()
        .map(|line| {
            let (first, last) = get_first_last(line, false);

            (first + &last).parse::<i32>().unwrap()
        })
        .sum();

    println!("Total: {}", total);
}

pub fn part_2() {
    let input = include_str!("inputs/day_1.txt");

    let total: i32 = input
        .lines()
        .map(|line| {
            let (first, last) = get_first_last(line, true);

            let first_digit = number_word_to_digit(&first);
            let last_digit = number_word_to_digit(&last);

            (first_digit.to_string() + &last_digit.to_string()).parse::<i32>().unwrap()
        })
        .sum();

    println!("Total: {}", total);
}