use itertools::Itertools;
use rust_aoc_lib::part2;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Card {
    A,
    K,
    Q,
    Joker,
    T,
    N(usize),
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        match (self, other) {
            (c1, c2) if c1 == c2 => Equal,
            (Card::Joker, _) => Less,
            (_, Card::Joker) => Greater,
            (Card::N(c1), Card::N(c2)) => c1.cmp(c2),
            (Card::A, _) => Greater,
            (_, Card::A) => Less,
            (Card::K, _) => Greater,
            (_, Card::K) => Less,
            (Card::Q, _) => Greater,
            (_, Card::Q) => Less,
            (Card::T, _) => Greater,
            (_, Card::T) => Less,
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::Joker),
            "T" => Ok(Card::T),
            n => Ok(Card::N(n.parse().unwrap())),
        }
    }
}

impl From<usize> for Card {
    fn from(n: usize) -> Self {
        match n {
            0 => Card::A,
            1 => Card::K,
            2 => Card::Q,
            3 => Card::Joker,
            4 => Card::T,
            n => Card::N(n - 3),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Hand {
    // Turns out it's not poker rules
    // anger.
    // FiveOfAKind(Card),
    // FourOfAKind(Card, Card),
    // FullHouse(Card, Card),
    // ThreeOfAKind(Card, Card, Card),
    // TwoPair(Card, Card, Card),
    // OnePair(Card, Card, Card, Card),
    // HighCard(Card, Card, Card, Card, Card),
    // OH SO YOU DON'T COMPARE THE FUCKING ACTUAL VALUE OF THE HAND.
    // FiveOfAKind(Card),
    // FourOfAKind(Card, [Card; 5]),
    // FullHouse(Card, Card, [Card; 5]),
    // ThreeOfAKind(Card, [Card; 5]),
    // TwoPair(Card, Card, [Card; 5]),
    // OnePair(Card, [Card; 5]),
    // HighCard(Card, [Card; 5]),
    FiveOfAKind([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card; 5]),
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        let mut counts = [0usize; 13];

        let mut joker_count = 0;

        for card in cards.iter() {
            match card {
                Card::A => counts[0] += 1,
                Card::K => counts[1] += 1,
                Card::Q => counts[2] += 1,
                Card::Joker => joker_count += 1,
                Card::T => counts[4] += 1,
                Card::N(n) => counts[n + 3] += 1,
            }
        }

        let mut counts = counts
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)
            .map(|(i, c)| (Card::from(i), *c))
            .sorted_by(|(i1, _), (i2, _)| i2.cmp(i1))
            .sorted_by(|(_, c1), (_, c2)| c2.cmp(c1))
            .collect::<Vec<_>>();

        if counts.is_empty() {
            return Hand::FiveOfAKind(cards);
        }

        counts[0] = (counts[0].0, counts[0].1 + joker_count);

        match counts[0].1 {
            5 => Hand::FiveOfAKind(cards),
            4 => Hand::FourOfAKind(cards),
            3 => {
                if counts[1].1 == 2 {
                    Hand::FullHouse(cards)
                } else {
                    Hand::ThreeOfAKind(cards)
                }
            }
            2 => {
                if counts[1].1 == 2 {
                    Hand::TwoPair(cards)
                } else {
                    Hand::OnePair(cards)
                }
            }
            1 => Hand::HighCard(cards),
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

        fn tie_break(card_set_1: &[Card; 5], card_set_2: &[Card; 5]) -> std::cmp::Ordering {
            for (card_1, card_2) in card_set_1.iter().zip(card_set_2.iter()) {
                match card_1.cmp(card_2) {
                    Equal => continue,
                    other => return other,
                }
            }

            Equal
        }

        match (self, other) {
            (h1, h2) if h1 == h2 => Equal,
            (Hand::FiveOfAKind(s1), Hand::FiveOfAKind(s2))
            | (Hand::FourOfAKind(s1), Hand::FourOfAKind(s2))
            | (Hand::FullHouse(s1), Hand::FullHouse(s2))
            | (Hand::ThreeOfAKind(s1), Hand::ThreeOfAKind(s2))
            | (Hand::TwoPair(s1), Hand::TwoPair(s2))
            | (Hand::OnePair(s1), Hand::OnePair(s2))
            | (Hand::HighCard(s1), Hand::HighCard(s2)) => tie_break(s1, s2),
            (Hand::FiveOfAKind(_), _) => Greater,
            (_, Hand::FiveOfAKind(_)) => Less,
            (Hand::FourOfAKind(_), _) => Greater,
            (_, Hand::FourOfAKind(_)) => Less,
            (Hand::FullHouse(_), _) => Greater,
            (_, Hand::FullHouse(_)) => Less,
            (Hand::ThreeOfAKind(_), _) => Greater,
            (_, Hand::ThreeOfAKind(_)) => Less,
            (Hand::TwoPair(_), _) => Greater,
            (_, Hand::TwoPair(_)) => Less,
            (Hand::OnePair(_), _) => Greater,
            (_, Hand::OnePair(_)) => Less,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::A; 5];

        for (i, card) in s.chars().enumerate() {
            cards[i] = Card::from_str(card.to_string().as_str()).unwrap();
        }

        Ok(Hand::from(cards))
    }
}

#[part2]
#[allow(unused_variables)]
pub fn part2(input: &str) -> usize {
    let cards_and_bids = input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = Hand::from_str(cards);
        let bid = bid.parse::<usize>().unwrap();

        (cards, bid)
    });

    let cards_and_bids_ranked =
        cards_and_bids.sorted_by(|(cards1, _), (cards2, _)| cards1.cmp(cards2));

    let pretty_print = cards_and_bids_ranked
        .clone()
        .enumerate()
        .map(|(i, (cards, bid))| format!("{}: cards = {:?} bid = {}", i + 1, cards, bid))
        .collect::<Vec<_>>();

    cards_and_bids_ranked
        .enumerate()
        .map(|(i, (cards, bid))| bid * (i + 1))
        .sum()
}
