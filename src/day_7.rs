use std::cmp::Ordering;

use crate::{day::Day, get_input_for_day};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    Pair,
    HighCard
}



#[derive(Debug)]
struct Hand {
    values: Vec<char>,
    consider_jokers: bool,
    pub bid: u64
}

impl Hand {

    const VALS_JOKERS: &'static str = "J23456789TJQKA";
    const VALS: &'static str = "23456789TJQKA";

    pub fn parse(input: &str, consider_jokers: bool) -> Self {
        let mut split = input.split(' ');
        let values = split.next().unwrap().chars().collect::<Vec<char>>();
        let bid = split.next().unwrap().parse::<u64>().unwrap();

        Self {
            values,
            consider_jokers,
            bid
        }
    }

    fn determine_type(&self) -> HandType {
        if self.consider_jokers {
            self.determine_type_with_jokers()
        } else {
            self.determine_type_without_jokers()
        }
    }

    fn determine_type_without_jokers(&self) -> HandType {
        let counts = self.values.iter().fold(std::collections::HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0_u64) += 1_u64;
            acc
        });

        let mut counts = counts.into_iter().collect::<Vec<(&char, u64)>>();

        counts.sort_by(|a, b| { 
            b.1.cmp(&a.1)
        });

        match counts[0].1 {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => {
                if counts[1].1 == 2_u64 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            },
            2 => {
                if counts[1].1 == 2_u64 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => HandType::HighCard
        }
    }

    fn determine_type_with_jokers(&self) -> HandType {
        let counts = self.values.iter().fold(std::collections::HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0_u64) += 1_u64;
            acc
        });

        let joker_count = *counts.get(&'J').unwrap_or(&0_u64);

        let mut counts = counts.into_iter().collect::<Vec<(&char, u64)>>();

        counts.sort_by(|a, b| { 
            let j_cmp = if *a.0 == 'J' { Ordering::Less } else if *b.0 == 'J' { Ordering::Greater } else { Ordering::Equal };
            b.1.cmp(&a.1).then(j_cmp)
        });

        match counts[0].0 {
            'J' => {
                match counts[0].1 {
                    5 => HandType::FiveOfKind,
                    4 => HandType::FiveOfKind,
                    3 => {
                        if counts[1].1 == 2_u64 {
                            HandType::FiveOfKind
                        } else {
                            HandType::FourOfKind
                        }
                    },
                    2 => {
                        if counts[1].1 == 2_u64 {
                            HandType::FourOfKind
                        } else {
                            HandType::ThreeOfKind
                        }
                    }
                    1 => HandType::Pair,
                    _ => unreachable!("Joker count should never be 0 because counts only has chars with count > 0")
                }
            },
            _ => match counts[0].1 {
                5 => HandType::FiveOfKind,
                4 => {
                    if joker_count == 1 {
                        HandType::FiveOfKind
                    } else {
                        HandType::FourOfKind
                    }
                },
                3 => {
                    match joker_count {
                        2 => HandType::FiveOfKind,
                        1 => HandType::FourOfKind,
                        _ => {
                            if counts[1].1 == 2_u64 {
                                HandType::FullHouse
                            } else {
                                HandType::ThreeOfKind
                            }
                        }
                    }
                }
                2 => {
                    match joker_count {
                        2 => HandType::FourOfKind,
                        1 => {
                            if counts[1].1 == 2_u64 {
                                HandType::FullHouse
                            } else {
                                HandType::ThreeOfKind
                            }
                        },
                        _ => {
                            if counts[1].1 == 2_u64 {
                                HandType::TwoPair
                            } else {
                                HandType::Pair
                            }
                        }
                    }
                },
                _ => HandType::HighCard
            }
        }
    }

    fn compare_values(&self, other: Vec<char>) -> std::cmp::Ordering {
        for (char, other) in self.values.iter().zip(other.iter()) {
            let vals = if self.consider_jokers { Self::VALS_JOKERS } else { Self::VALS };
            let self_val = vals.find(*char).unwrap();
            let other_val = vals.find(*other).unwrap();

            let ord = self_val.cmp(&other_val);

            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    }

}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.determine_type() == other.determine_type() && self.compare_values(other.values.clone()) == std::cmp::Ordering::Equal
    }
}

impl Eq for Hand {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.determine_type();
        let other_type = other.determine_type();

        self_type.cmp(&other_type).reverse().then(self.compare_values(other.values.clone()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day7;

impl Day for Day7 {
    get_input_for_day!(7);

    fn part_1(&self, input: &str) -> i32 {
        let mut hands = input.lines().map(|l| Hand::parse(l, false)).collect::<Vec<Hand>>();
        hands.sort();
        hands.into_iter().enumerate().map(|(i, h)| h.bid * ((i as u64) + 1) ).sum::<u64>() as i32
    }

    fn part_2(&self, input: &str) -> i32 {
        let mut hands = input.lines().map(|l| Hand::parse(l, true)).collect::<Vec<Hand>>();
        hands.sort();
        hands.into_iter().enumerate().map(|(i, h)| h.bid * ((i as u64) + 1) ).sum::<u64>() as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_card_type() {

        let inputs = vec![
            ("JJJJJ 100", HandType::FiveOfKind),
            ("JJJJ2 100", HandType::FiveOfKind),
            ("JJJ22 100", HandType::FiveOfKind),
            ("JJ222 100", HandType::FiveOfKind),
            ("J2222 100", HandType::FiveOfKind),
            ("J2223 100", HandType::FourOfKind),
            ("JJ224 100", HandType::FourOfKind),
            ("JJJ24 100", HandType::FourOfKind),
            ("JJJ44 100", HandType::FiveOfKind),
            ("J4433 100", HandType::FullHouse),
            ("J4432 100", HandType::ThreeOfKind),
            ("J4434 100", HandType::FourOfKind),
            ("J4435 100", HandType::ThreeOfKind),
            ("J1234 100", HandType::Pair),
            ("J1225 100", HandType::ThreeOfKind),
            ("J1122 100", HandType::FullHouse),
        ];

        for (input, expected) in inputs {
            println!("Testing: {}", input);
            let hand = Hand::parse(input, true);
            assert_eq!(hand.determine_type(), expected);
        }

    }

    #[test]
    fn test_part_1() {
        let day = Day7;
        let input = day.get_input();
        assert_eq!(day.part_1(input), 249638405);
    }

    #[test]
    fn test_part_2() {
        let day = Day7;
        let input = day.get_input();
        assert_eq!(day.part_2(input), 249776650);
    }

}
