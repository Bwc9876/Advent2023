use std::str::FromStr;

pub fn parse_padded_numbers<N>(input: &str) -> Vec<N>
where
    N: FromStr,
    <N as FromStr>::Err: std::fmt::Debug,
{
    input
        .split_whitespace()
        .map(|s| s.parse::<N>().unwrap())
        .collect()
}
