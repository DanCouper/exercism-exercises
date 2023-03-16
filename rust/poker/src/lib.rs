#![feature(iter_array_chunks)]

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut prev_hand = Hand(HandRank::Unknown, 0, 0, 0);
    let mut winners = vec![];

    for (i, hand) in hands.iter().enumerate() {
        let curr_hand = Hand::from_slice(hand);

        if curr_hand > prev_hand {
            winners.clear();
            winners.push(hands[i]);
            prev_hand = curr_hand;
        } else if curr_hand == prev_hand {
            winners.push(hands[i]);
        }
    }

    winners
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Hand(HandRank, u16, u16, u16);

impl Hand {
    fn from_slice(hand_slice: &str) -> Hand {
        let bfs = hand_slice
            .chars()
            .filter_map(parse_valid_hand_char)
            .array_chunks()
            .fold(Bitfields::init(), |mut bfs, [rank, suit]| {
                // Set highest unset bit in the tally.
                let new_tally_for_rank = bfs.tally_for_rank(rank) << 1 | 1;

                bfs.ranks |= 1 << rank;
                bfs.suits |= 1 << suit;
                bfs.tally |= new_tally_for_rank << (rank * 4);
                bfs.tally_score += 1 << new_tally_for_rank;
                bfs
            });

        Hand::from_bitfields(bfs)
    }

    fn from_bitfields(bfs: Bitfields) -> Hand {
        match bfs.tally_score {
            10 => {
                let is_straight = bfs.is_low_straight() | bfs.is_high_straight();
                let is_flush = bfs.suits.count_ones() == 1;
                let kicker = if bfs.is_low_straight() {
                    0b1111
                } else {
                    bfs.ranks
                };

                match (is_straight, is_flush) {
                    (false, false) => Hand(HandRank::HighCard, kicker, 0, 0),
                    (true, false) => Hand(HandRank::Straight, kicker, 0, 0),
                    (false, true) => Hand(HandRank::Flush, kicker, 0, 0),
                    (true, true) => Hand(HandRank::StraightFlush, kicker, 0, 0),
                }
            }
            16 => {
                let (hi, midhi, midlo, lo) = bfs.rankpos4();
                let (pair, kicker) = match (
                    bfs.tally_for_rank(hi) == 0b11,
                    bfs.tally_for_rank(midhi) == 0b11,
                    bfs.tally_for_rank(midlo) == 0b11,
                ) {
                    (true, false, false) => (hi, bfs.zero_out_rank(hi)),
                    (false, true, false) => (midhi, bfs.zero_out_rank(midhi)),
                    (false, false, true) => (midhi, bfs.zero_out_rank(midlo)),
                    (false, false, false) => (midlo, bfs.zero_out_rank(lo)),
                    _ => unreachable!(),
                };

                Hand(HandRank::OnePair, pair, kicker, 0)
            }
            22 => {
                let (hi, mid, lo) = bfs.rankpos3();
                let (high_pair, low_pair, kicker) = match (
                    bfs.tally_for_rank(hi) == 0b11,
                    bfs.tally_for_rank(mid) == 0b11,
                ) {
                    (true, true) => (hi, mid, lo),
                    (true, false) => (hi, lo, mid),
                    (false, true) => (mid, lo, hi),
                    _ => unreachable!(),
                };

                Hand(HandRank::TwoPair, high_pair, low_pair, kicker)
            }
            142 => {
                let (hi, lo) = bfs.rankpos2();
                let (trip, kicker) = match bfs.tally_for_rank(hi) == 0b111 {
                    true => (hi, bfs.zero_out_rank(hi)),
                    false => (lo, bfs.zero_out_rank(lo)),
                };

                Hand(HandRank::ThreeOfAKind, trip, kicker, 0)
            }
            148 => {
                let (hi, lo) = bfs.rankpos2();
                let (trip, pair) = match bfs.tally_for_rank(hi) == 0b111 {
                    true => (hi, lo),
                    false => (lo, hi),
                };

                Hand(HandRank::FullHouse, trip, pair, 0)
            }
            32908 => {
                let (hi, lo) = bfs.rankpos2();
                let (quad, kicker) = match bfs.tally_for_rank(hi) == 0b1111 {
                    true => (hi, bfs.zero_out_rank(hi)),
                    false => (lo, bfs.zero_out_rank(lo)),
                };

                Hand(HandRank::FourOfAKind, quad, kicker, 0)
            }
            _ => Hand(HandRank::Unknown, 0, 0, 0),
        }
    }
}

// NOTE: `trailing_zeros` returning a u32 is annoying when dealing with a u16.
fn u16_trailing_zeros(n: u16) -> u16 {
    u16::try_from(n.trailing_zeros()).unwrap()
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq)]
enum HandRank {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug)]
struct Bitfields {
    tally: u64,
    tally_score: u32,
    ranks: u16,
    suits: u16,
}

impl Bitfields {
    fn init() -> Bitfields {
        Bitfields {
            tally: 0,
            tally_score: 0,
            ranks: 0,
            suits: 0,
        }
    }

    // A sequence of 5 contiguous set bits in the ranks represents a straight.
    fn is_high_straight(&self) -> bool {
        let high_mask = 0b11111;
        (self.ranks >> self.ranks.trailing_zeros() & high_mask) == high_mask
    }

    // Aces occupy the highest bit set in ranks. A straight can be A,2,3,4,5, so
    // in that case, looking for 5 contiguous bits won't work.
    fn is_low_straight(&self) -> bool {
        let low_mask = 0b0001000000001111;
        (self.ranks & low_mask) == low_mask
    }

    // If there are two ranks present, get lowest and highest positions
    fn rankpos2(&self) -> (u16, u16) {
        let lorank = u16_trailing_zeros(self.ranks);
        let hirank = u16_trailing_zeros(self.zero_out_rank(lorank));

        (hirank, lorank)
    }

    // If there are three ranks present, get lowest, highest and middle positions
    fn rankpos3(&self) -> (u16, u16, u16) {
        let mut ranks = self.ranks;
        let lorank = u16_trailing_zeros(ranks);
        ranks &= !(1 << lorank);
        let midrank = u16_trailing_zeros(ranks);
        ranks &= !(1 << midrank);
        let hirank = u16_trailing_zeros(ranks);

        (hirank, midrank, lorank)
    }

    // If there are four ranks present, get positions of all four
    fn rankpos4(&self) -> (u16, u16, u16, u16) {
        let mut ranks = self.ranks;
        let lorank = u16_trailing_zeros(ranks);
        ranks &= !(1 << lorank);
        let midlorank = u16_trailing_zeros(ranks);
        ranks &= !(1 << midlorank);
        let midhirank = u16_trailing_zeros(ranks);
        ranks &= !(1 << midhirank);
        let hirank = u16_trailing_zeros(ranks);

        (hirank, midhirank, midlorank, lorank)
    }

    // Isolate the four bits representing a tally for a given card rank
    fn tally_for_rank(&self, rank: u16) -> u64 {
        self.tally >> (rank * 4) & 0b1111
    }

    // Return a new set of ranks with given rank zeroed out
    fn zero_out_rank(&self, rank: u16) -> u16 {
        self.ranks & !(1 << rank)
    }
}

// Parse
fn parse_valid_hand_char(c: char) -> Option<u16> {
    match c {
        '2'..='9' => Some((c as u16) - 50),
        '0' => Some(8),
        'J' => Some(9),
        'Q' => Some(10),
        'K' => Some(11),
        'A' => Some(12),
        // Suits
        'S' => Some(0),
        'H' => Some(1),
        'D' => Some(2),
        'C' => Some(3),
        _ => None,
    }
}
