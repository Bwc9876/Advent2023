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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_padded() {
        let input = "3  56  45  23       56";
        let expected = vec![3, 56, 45, 23, 56];
        assert_eq!(parse_padded_numbers::<i64>(input), expected);
    }
}
