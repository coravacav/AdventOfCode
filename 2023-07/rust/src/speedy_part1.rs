use rust_aoc_lib::part1;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Copy)]
pub struct Hand(usize);

const VALUES: [u8; 13] = [13, 12, 11, 10, 9, 1, 2, 3, 4, 5, 6, 7, 8];

impl From<&[u8]> for Hand {
    fn from(cards: &[u8]) -> Self {
        let mut counts = [0usize; 13];

        let mut soon_to_be_usize = [0u8; 8];
        let mut soon_i = 3;

        for card in cards {
            let i = match card {
                b'A' => 0,
                b'K' => 1,
                b'Q' => 2,
                b'J' => 3,
                b'T' => 4,
                n @ b'2'..=b'9' => (*n - b'0') as usize + 3,
                _ => unreachable!(),
            };

            counts[i] += 1;
            soon_to_be_usize[soon_i] = VALUES[i];
            soon_i += 1;
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));
        let counts = &counts[..2];

        soon_to_be_usize[2] = match counts[0] {
            5 => 6, // Five of a kind
            4 => 5, // Four of a kind
            3 => {
                if counts[1] == 2 {
                    4 // Full house
                } else {
                    3 // Three of a kind
                }
            }
            2 => {
                if counts[1] == 2 {
                    2 // Two pair
                } else {
                    1 // One pair
                }
            }
            1 => 0, // High card
            _ => unreachable!(),
        };

        Hand(usize::from_be_bytes(soon_to_be_usize))
    }
}

#[part1]
pub fn speedy_part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut cards_and_bids_ranked = [(Hand(0), 0); 1000];
    let mut cards_i = 0;
    let mut i = 0;

    while i < input.len() {
        let hand: Hand = input[i..i + 5].into();

        i += 6;

        let val = {
            let mut val = 0;
            while let c @ b'0'..=b'9' = input[i] {
                val = val * 10 + (c - b'0') as usize;
                i += 1;

                if i == input.len() {
                    break;
                }
            }
            val
        };

        i += 1;

        cards_and_bids_ranked[cards_i] = (hand, val);
        cards_i += 1;
    }

    cards_and_bids_ranked.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    cards_and_bids_ranked
        .into_iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(i, bid)| bid * (i + 1))
        .sum()
}
