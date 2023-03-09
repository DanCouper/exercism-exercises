use anyhow::Result;
use std::fmt;

struct BitHand {
    hand: u64,
    score: u8,
}

fn rank_value(r: char) -> u8 {
    match r {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn suit_value(s: char) -> u8 {
    match s {
        'S' => 0,
        'H' => 1,
        'D' => 2,
        'C' => 3,
        _ => unreachable!(),
    }
}

impl BitHand {
    fn from(input_hand: &[&str]) -> Result<BitHand> {
        let (hand, ranks, suits): (u64, u16, [u8; 5]) = input_hand.iter().enumerate().fold(
            (0, 0, [0; 5]),
            |(hand_bf, ranks_bf, mut suits), (i, card)| {
                let card_chars = card.chars().collect::<Vec<char>>();
                let rank = rank_value(card_chars[0]);
                let suit = suit_value(card_chars[1]);

                suits[i] = suit;
                (
                    hand_bf | 1 << ((rank * 4) + suit),
                    ranks_bf | 1 << rank,
                    suits,
                )
            },
        );

        // Detects & produces the following values for these hands ONLY:
        // - High card: 5
        // - One pair: 6
        // - Two pair: 7
        // - Three of a kind: 9
        // - Full house: 10
        // - Four of a kind: 16
        let (score_from_hand, _) = (0..=12).fold((0, hand), |(total, hand), _| {
            let cards_in_rank = (hand & 0b1111).count_ones();
            (total + (1 << cards_in_rank) - 1, hand >> 4)
        });

        let is_straight = (ranks & 0b0001000000001111) == 0b0001000000001111
            || ((ranks >> ranks.trailing_zeros()) & 0b11111) == 0b11111;

        let is_flush = (1..=4).fold(0, |res, i| res + suits[0] - suits[i]) == 0;

        #[rustfmt::skip]
        let score = match (score_from_hand, is_straight, is_flush) {
            (5, false, false) => 0, // High card
            (6,     _,     _) => 1, // One pair
            (7,     _,     _) => 2, // Two pair
            (9,     _,     _) => 3, // Three of a kind
            (5,  true, false) => 4, // Straight
            (5, false,  true) => 5, // Flush
            (10,    _,     _) => 6, // Full house
            (16,    _,     _) => 7, // Four of a kind
            (5,  true,  true) => 8, // Straight flush
            _ => unreachable!(),
        };

        Ok(BitHand { hand, score })
    }
}

fn format_hand_bf(mut nshifter: u64) -> String {
    let mut nflag = 0;
    let mut chars: Vec<char> = vec![];

    while nflag < 79 {
        nflag += 1;
        if nflag % 5 == 0 {
            chars.push(' ');
        } else {
            chars.push(if nshifter >> 63 == 1 { '1' } else { '0' });
            nshifter <<= 1;
        }
    }
    chars.into_iter().collect()
}

impl fmt::Display for BitHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let legend1 =
            "               A    K    Q    J    T    9    8    7    6    5    4    3    2   ";
        let legend2 =
            "xxxx xxxx xxxx CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS";
        write!(f, "{}\n{}\n{}", legend1, legend2, format_hand_bf(self.hand))
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
}
