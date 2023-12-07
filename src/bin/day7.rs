use color_eyre::Result;
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    hand: Vec<Card>,
    bid: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::NINE,
            '8' => Card::EIGHT,
            '7' => Card::SEVEN,
            '6' => Card::SIX,
            '5' => Card::FIVE,
            '4' => Card::FOUR,
            '3' => Card::THREE,
            '2' => Card::TWO,
            _ => panic!("Invalid"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Card2 {
    A,
    K,
    Q,
    T,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
    // I fucking hate this
    J,
}

impl From<&Card> for Card2 {
    fn from(value: &Card) -> Self {
        match value {
            Card::A => Card2::A,
            Card::K => Card2::K,
            Card::Q => Card2::Q,
            Card::J => Card2::J,
            Card::T => Card2::T,
            Card::NINE => Card2::NINE,
            Card::EIGHT => Card2::EIGHT,
            Card::SEVEN => Card2::SEVEN,
            Card::SIX => Card2::SIX,
            Card::FIVE => Card2::FIVE,
            Card::FOUR => Card2::FOUR,
            Card::THREE => Card2::THREE,
            Card::TWO => Card2::TWO,
        }
    }
}

#[derive(Eq, Debug)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && self
                .cards
                .iter()
                .zip(other.cards.iter())
                .all(|(a, b)| a == b)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

#[derive(Eq, Debug)]
pub struct Hand2 {
    hand_type: HandType,
    cards: Vec<Card2>,
    bid: u32,
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && self
                .cards
                .iter()
                .zip(other.cards.iter())
                .all(|(a, b)| a == b)
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    HighCard,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day7.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use crate::Card;

    use super::Input;
    use advent_of_code2023::parsing::parse_number;
    use color_eyre::Result;

    use itertools::Itertools;
    use nom::bytes::complete::take;
    use nom::error::Error;
    use nom::sequence::separated_pair;
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
        map(take(5usize), |cards: &str| {
            cards.chars().map(Card::from).collect_vec()
        })(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(parse_cards, tag(" "), parse_number),
            |(hand, bid)| Input { hand, bid },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn get_hand_type(cards: &[Card]) -> HandType {
    // So many conditions
    let card_counts: HashMap<&Card, u32> = cards.iter().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    });
    match card_counts.keys().len() {
        1 => HandType::Five,
        2 => match card_counts.values().next() {
            Some(4) | Some(1) => HandType::Four,
            Some(3) | Some(2) => HandType::FullHouse,
            _ => panic!("Unexpected card counts"),
        },
        3 => {
            // Could be two pair or three of a kind
            match card_counts.values().max() {
                Some(3) => HandType::Three,
                Some(2) => HandType::TwoPair,
                _ => panic!("Unexpected card counts"),
            }
        }
        4 => HandType::Pair,
        5 => HandType::HighCard,
        _ => panic!("Unexpected card counts"),
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    input
        .iter()
        .map(|i| Hand {
            hand_type: get_hand_type(&i.hand),
            cards: i.hand.clone(),
            bid: i.bid,
        })
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn get_hand_type_part_2(cards: &[Card]) -> HandType {
    // Maybe this'll work by getting the hand type minus jokers, then using that info
    let base_hand_type = get_hand_type(cards);
    let joker_count = cards.iter().filter(|&c| *c == Card::J).count();
    if joker_count == 0 {
        return base_hand_type;
    }
    match base_hand_type {
        // Can't beat that
        HandType::Five => HandType::Five,
        HandType::Four => HandType::Five,
        HandType::FullHouse => HandType::Five,
        HandType::Three => match joker_count {
            1 => HandType::Four,
            2 => HandType::Five,
            // If there was also a pair, this would be a full house
            3 => HandType::Four,
            _ => panic!("Invalid"),
        },
        HandType::TwoPair => {
            if joker_count == 1 {
                HandType::FullHouse
            } else {
                HandType::Four
            }
        }
        HandType::Pair => HandType::Three,
        HandType::HighCard => HandType::Pair,
    }
}

fn solve_part2(input: &[Input]) -> u32 {
    input
        .iter()
        .map(|i| Hand2 {
            hand_type: get_hand_type_part_2(&i.hand),
            cards: i.hand.iter().map(Card2::from).collect_vec(),
            bid: i.bid,
        })
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 6440);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 5905);
        Ok(())
    }
}
