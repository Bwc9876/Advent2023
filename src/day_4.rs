use crate::{day::Day, get_input_for_day, utils::parse_padded_numbers};

pub struct Day4;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {

    pub fn parse(input: &str) -> Self {
        let data = input.split(':').nth(1).unwrap();
        let data = data.trim().replace("  ", " ");
        let s = data.split('|');
        let winning = parse_padded_numbers(s.clone().nth(0).unwrap());
        let numbers = parse_padded_numbers(s.clone().nth(1).unwrap());
        Self {
            winning,
            numbers,
        }
    }

    pub fn get_amount_matching(&self) -> u32 {
        self.numbers.iter().filter(|x| self.winning.contains(x)).count() as u32
    }

    pub fn get_score(&self) -> u32 {
        let matching = self.get_amount_matching();
        if matching == 0 {
            0
        } else {
            2_u32.pow(matching - 1)
        }
    }

}

impl Day for Day4 {

    get_input_for_day!(4);

    fn part_1(&self, input: &str) -> i64 {
        let lines = input.split('\n');
        let cards = lines.map(Card::parse).collect::<Vec<Card>>();

        cards.into_iter().map(|x| x.get_score()).sum::<u32>() as i64
    }

    fn part_2(&self, input: &str) -> i64 {
        let lines = input.split('\n');
        let cards = lines.map(Card::parse).collect::<Vec<Card>>();

        let mut amounts = vec![1;cards.len()];

        for (id, card) in cards.iter().enumerate() {
            let id = id as u32;
            let amount_matching = card.get_amount_matching();

            if amount_matching != 0 {
                let my_amount = *amounts.get(id as usize).unwrap();
                for i in id+1..=id+amount_matching {
                    let card_amount = amounts.get_mut(i as usize).unwrap();
                    *card_amount += my_amount;
                }
            }
        }

        amounts.into_iter().sum::<u32>() as i64
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Card 1: 1 2 3 4 5 | 6 7 8 9 10";
        let card = Card::parse(input);
        assert_eq!(card.winning, vec![1, 2, 3, 4, 5]);
        assert_eq!(card.numbers, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_get_score() {
        let input = "Card 1: 1 2 3 4 5 | 1 1 4 9 10";
        let card = Card::parse(input);
        assert_eq!(card.get_score(), 4);
    }

    #[test]
    fn test_part_1() {
        let day = Day4 {};
        let input = day.get_input();
        assert_eq!(day.part_1(input), 23750);
    }

    #[test]
    fn test_part_2() {
        let day = Day4 {};
        let input = day.get_input();
        assert_eq!(day.part_2(input), 13261850);
    }

}