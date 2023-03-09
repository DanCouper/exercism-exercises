use anyhow::Result;
use std::fmt;

fn rankshift(rank: u8) -> u8 {
    match rank {
        b'2'..=b'9' => rank - 50,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

fn suitshift(suit: u8) -> u8 {
    match suit {
        b'S' => 0,
        b'H' => 1,
        b'D' => 2,
        b'C' => 3,
        _ => unreachable!(),
    }
}

enum FlushCheck {
    Unchecked,
    PossibleFlush(u8),
    NotFlush,
}

struct Hand {
    bitfield: u64,
    score: u8,
}

impl Hand {
    fn from_slice(hand: &str) -> Hand {
        assert!(hand.len() == 14 && hand.is_ascii());
        let mut rank_counts: u64 = 0;
        let mut ranks_present: u16 = 0;
        let mut flush_state = FlushCheck::Unchecked;
        let mut characters = hand.as_bytes();

        loop {
            match characters {
                [b' ', rest @ ..] => {
                    characters = rest;
                }
                [rank, suit, rest @ ..] => {
                    rank_counts |= 1 << ((rankshift(*rank) * 4) + suitshift(*suit));
                    ranks_present |= 1 << rankshift(*rank);
                    flush_state = match flush_state {
                        FlushCheck::Unchecked => FlushCheck::PossibleFlush(*suit),
                        FlushCheck::PossibleFlush(s) if s != *suit => FlushCheck::NotFlush,
                        _ => flush_state,
                    };
                    characters = rest;
                }
                [..] => break,
            }
        }

        Hand {
            bitfield: rank_counts,
            score: score_hand(rank_counts, ranks_present, flush_state),
        }
    }
}

fn score_hand(rank_counts: u64, ranks_present: u16, flush_state: FlushCheck) -> u8 {
    let (score_from_rank_counts, _) = (0..=12).fold((0, rank_counts), |(total, rank_counts), _| {
        let cards_in_rank = (rank_counts & 0b1111).count_ones();
        (total + (1 << cards_in_rank) - 1, rank_counts >> 4)
    });

    let is_straight = (ranks_present & 0b0001000000001111) == 0b0001000000001111
        || ((ranks_present >> ranks_present.trailing_zeros()) & 0b11111) == 0b11111;

    let is_flush = matches!(flush_state, FlushCheck::PossibleFlush(_));

    match (score_from_rank_counts, is_straight, is_flush) {
        (5, false, false) => 0, // High card
        (6, _, _) => 1,         // One pair
        (7, _, _) => 2,         // Two pair
        (9, _, _) => 3,         // Three of a kind
        (5, true, false) => 4,  // Straight
        (5, false, true) => 5,  // Flush
        (10, _, _) => 6,        // Full house
        (16, _, _) => 7,        // Four of a kind
        (5, true, true) => 8,   // Straight flush
        _ => unreachable!(),
    }
}

fn format_bf(mut bitfield: u64) -> String {
    let mut nflag = 0;
    let mut chars: Vec<char> = vec![];

    while nflag < 79 {
        nflag += 1;
        if nflag % 5 == 0 {
            chars.push(' ');
        } else {
            chars.push(if bitfield >> 63 == 1 { '1' } else { '0' });
            bitfield <<= 1;
        }
    }
    chars.into_iter().collect()
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scored_hand = match self.score {
            0 => "High card",
            1 => "One pair",
            2 => "Two pair",
            3 => "Three of a kind",
            4 => "Straight",
            5 => "Flush",
            6 => "Full house",
            7 => "Four of a kind",
            8 => "Straight flush",
            _ => unreachable!(),
        };

        let legend1 =
            "               A    K    Q    J    T    9    8    7    6    5    4    3    2   ";
        let legend2 =
            "xxxx xxxx xxxx CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS";
        write!(
            f,
            "Score: {}\n{}\n{}\n{}",
            scored_hand,
            legend1,
            legend2,
            format_bf(self.bitfield)
        )
    }
}
