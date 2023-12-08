use itertools::Itertools;
use rust_aoc_lib::{part1, part2};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Card {
    A,
    K,
    Q,
    J,
    Joker,
    T,
    N(usize),
}

impl Card {
    fn rank(&self) -> usize {
        match self {
            Card::A => 0,
            Card::K => 1,
            Card::Q => 2,
            Card::J => 3,
            Card::T => 4,
            Card::N(n) => 15 - *n,
            Card::Joker => 69,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.rank().cmp(&self.rank())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Hand {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl Hand {
    fn rank(&self) -> usize {
        match self {
            Hand::FiveOfAKind(_) => 6,
            Hand::FourOfAKind(_) => 5,
            Hand::FullHouse(_) => 4,
            Hand::ThreeOfAKind(_) => 3,
            Hand::TwoPair(_) => 2,
            Hand::OnePair(_) => 1,
            Hand::HighCard(_) => 0,
        }
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        let mut joker_count = 0;

        let mut counts = cards
            .iter()
            .fold([0; 13], |mut arr, card| {
                match card {
                    Card::A => arr[0] += 1,
                    Card::K => arr[1] += 1,
                    Card::Q => arr[2] += 1,
                    Card::J => arr[3] += 1,
                    Card::Joker => joker_count += 1,
                    Card::T => arr[4] += 1,
                    Card::N(n) => arr[n + 3] += 1,
                }

                arr
            })
            .into_iter()
            .enumerate()
            .filter(|&(_, count)| count > 0)
            .map(|(index_into_counts, count)| {
                (
                    match index_into_counts {
                        0 => Card::A,
                        1 => Card::K,
                        2 => Card::Q,
                        3 => Card::J,
                        4 => Card::T,
                        i => Card::N(i - 3),
                    },
                    count,
                )
            })
            .sorted_by(|(i1, c1), (i2, c2)| c2.cmp(c1).then_with(|| i2.cmp(i1)))
            .map(|(_, count)| count);

        match (
            counts.next().map(|count| count + joker_count),
            counts.next(),
        ) {
            (Some(5), _) => Hand::FiveOfAKind(cards),
            (Some(4), _) => Hand::FourOfAKind(cards),
            (Some(3), Some(2)) => Hand::FullHouse(cards),
            (Some(3), _) => Hand::ThreeOfAKind(cards),
            (Some(2), Some(2)) => Hand::TwoPair(cards),
            (Some(2), _) => Hand::OnePair(cards),
            (Some(1), _) => Hand::HighCard(cards),
            (None, None) => Hand::FiveOfAKind(cards),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        fn tie_break(card_set_1: &[Card], card_set_2: &[Card]) -> std::cmp::Ordering {
            for (card_1, card_2) in card_set_1.iter().zip(card_set_2.iter()) {
                match card_1.cmp(card_2) {
                    Equal => continue,
                    other => return other,
                }
            }

            Equal
        }

        match (self, other) {
            (Hand::FiveOfAKind(s1), Hand::FiveOfAKind(s2))
            | (Hand::FourOfAKind(s1), Hand::FourOfAKind(s2))
            | (Hand::FullHouse(s1), Hand::FullHouse(s2))
            | (Hand::ThreeOfAKind(s1), Hand::ThreeOfAKind(s2))
            | (Hand::TwoPair(s1), Hand::TwoPair(s2))
            | (Hand::OnePair(s1), Hand::OnePair(s2))
            | (Hand::HighCard(s1), Hand::HighCard(s2)) => tie_break(s1, s2),
            (h1, h2) => h1.rank().cmp(&h2.rank()),
        }
    }
}

fn calculate_dynamic_card_func(input: &str, card_from_letter: impl Fn(char) -> Card) -> usize {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(unparsed_cards, bid)| (unparsed_cards, bid.parse::<usize>().unwrap()))
        .map(|(unparsed_cards, bid)| (unparsed_cards.chars().map(&card_from_letter), bid))
        .map(|(unparsed_cards, bid)| (Hand::from(unparsed_cards.collect_vec()), bid))
        .sorted_by(|(cards1, _), (cards2, _)| cards1.cmp(cards2))
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[part1]
pub fn shared_impl_part1(input: &str) -> usize {
    calculate_dynamic_card_func(input, |card| match card {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '0'..='9' => Card::N(card.to_string().parse::<usize>().unwrap()),
        _ => unreachable!(),
    })
}

#[part2]
pub fn shared_impl_part2(input: &str) -> usize {
    calculate_dynamic_card_func(input, |card| match card {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::Joker,
        'T' => Card::T,
        '0'..='9' => Card::N(card.to_string().parse::<usize>().unwrap()),
        _ => unreachable!(),
    })
}
