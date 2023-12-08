pub trait Day {
    fn part_1(&self, input: &str) -> i64;

    fn part_2(&self, input: &str) -> i64;

    fn get_input(&self) -> &'static str;
}

#[macro_export]
macro_rules! mod_days {
    ($($day:ident),*) => {
        $(
            mod $day;
        )*
    };
}

#[macro_export]
macro_rules! get_input_for_day {
    ($day:literal) => {
        fn get_input(&self) -> &'static str {
            include_str!(concat!("inputs/day_", stringify!($day), ".txt"))
        }
    };
}
