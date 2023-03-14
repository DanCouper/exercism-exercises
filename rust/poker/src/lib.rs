#![feature(stmt_expr_attributes)]

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut prev_hand = HandBitfields {
        tally: 0,
        ranks: 0,
        suits: 0,
        raw_score: 0,
    };
    let mut prev_tiebreaker = TieBreaker(0, 0, 0);
    let mut rank_indices: Vec<usize> = vec![];

    for (i, hand) in hands.iter().enumerate() {
        let curr_hand = HandBitfields::from_slice(hand);
        let curr_tiebreaker = TieBreaker::create_from_hand(&curr_hand);
        let (curr_score, prev_score) = (curr_hand.score(), prev_hand.score());

        if (curr_score > prev_score)
            || ((curr_score == prev_score) && (curr_tiebreaker > prev_tiebreaker))
        {
            prev_hand = curr_hand;
            prev_tiebreaker = curr_tiebreaker;
            rank_indices.clear();
            rank_indices.push(i);
        } else if (curr_score == prev_score) && (curr_tiebreaker == prev_tiebreaker) {
            rank_indices.push(i);
        }
    }
    // REVIEW: can I use a bitfield for the indices rather than creating a vec?
    rank_indices.iter().map(|i| hands[*i]).collect()
}

fn map_face_value(binary_val: u8) -> u8 {
    match binary_val {
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'0' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

fn map_suit_value(binary_val: u8) -> u8 {
    match binary_val {
        b'S' => 0,
        b'H' => 1,
        b'D' => 2,
        b'C' => 3,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
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
pub struct HandBitfields {
    tally: u64,
    ranks: u16,
    suits: u8,
    raw_score: u32,
}

impl HandBitfields {
    fn from_slice(hand: &str) -> HandBitfields {
        hand.bytes()
            .into_iter()
            // NOTE: to be able to map face_value/suits as even/odd, need an even number of
            // face_value/suit pairs. So remove spaces, & treat '0' as a ten by removing '1'
            .filter(|&binary_val| binary_val != b' ' && binary_val != b'1')
            .enumerate()
            .fold(
                HandBitfields {
                    tally: 0,
                    ranks: 0,
                    suits: 0,
                    raw_score: 0,
                },
                |mut hand, (i, current_val)| {
                    if i % 2 == 0 {
                        let rank = map_face_value(current_val);
                        let target_tally_location = (rank * 4) as u32;
                        let target_tally = hand.tally >> target_tally_location & 0xf;

                        hand.ranks |= 1 << rank;
                        hand.tally |= 1 << (target_tally_location + target_tally.count_ones());
                        // NOTE: tally has been updated, isolate the *new* tally to get updated score
                        hand.raw_score += 1 << (hand.tally >> target_tally_location & 0b1111);
                    } else {
                        let suit = map_suit_value(current_val);
                        hand.suits |= 1 << suit;
                    }
                    hand
                },
            )
    }

    fn is_high_straight(&self) -> bool {
        let high_mask = 0b11111;
        ((self.ranks >> self.ranks.trailing_zeros()) & high_mask) == high_mask
    }

    fn is_low_straight(&self) -> bool {
        let low_mask = 0b0001000000001111;
        (self.ranks & low_mask) == low_mask
    }

    fn is_flush(&self) -> bool {
        self.suits.count_ones() == 1
    }

    fn score(&self) -> HandRank {
        if self.raw_score == 0 {
            return HandRank::Unknown;
        }

        let is_straight = self.is_low_straight() || self.is_high_straight();

        #[rustfmt::skip]
        match (self.raw_score, is_straight, self.is_flush()) {
            (10, false, false)  => HandRank::HighCard,
            (16, _, _)          => HandRank::OnePair,
            (22, _, _)          => HandRank::TwoPair,
            (142, _, _)         => HandRank::ThreeOfAKind,
            (10, true, false)   => HandRank::Straight,
            (10, false, true)   => HandRank::Flush,
            (148, _, _)         => HandRank::FullHouse,
            (32908, _, _)       => HandRank::FourOfAKind,
            (10, true, true)    => HandRank::StraightFlush,
            _                   => HandRank::Unknown,
        }
    }
}

#[derive(Debug)]
struct TieBreaker(u32, u32, u32);

impl TieBreaker {
    fn create_from_hand(hand: &HandBitfields) -> TieBreaker {
        match hand.score() {
            // NOTE: if the hand is a straight and aces are not low, logic is identical to high card and flush.
            HandRank::HighCard | HandRank::Straight | HandRank::Flush | HandRank::StraightFlush => {
                match hand.is_low_straight() {
                    true => TieBreaker(0b1000, 0, 0),
                    false => TieBreaker(hand.ranks as u32, 0, 0),
                }
            }
            // NOTE: for four of a kind, only need to check if lowest three bits are set for a given rank, same as full house. Five cards, and only one other rank.
            HandRank::FullHouse | HandRank::FourOfAKind => {
                let (left_rank, right_rank) =
                    (15 - hand.ranks.leading_zeros(), hand.ranks.trailing_zeros());
                match (hand.tally >> (left_rank * 4) & 0b111) == 0b111 {
                    true => TieBreaker(left_rank, right_rank, 0),
                    false => TieBreaker(right_rank, left_rank, 0),
                }
            }
            // NOTE: three of a kind and two pair are *almost* identical, but the cascade can use a
            // single value here (simply based on highest rank).
            HandRank::ThreeOfAKind => {
                let (left_rank, right_rank) =
                    (15 - hand.ranks.leading_zeros(), hand.ranks.trailing_zeros());
                // NOTE: zero highest bit so the middle bit is now the highest
                let remaining_ranks = hand.ranks & ((1 << left_rank) - 1);
                let middle_rank = 15 - remaining_ranks.leading_zeros();

                let left_rank_contains_trip = (hand.tally >> (left_rank * 4) & 0b111) == 0b111;
                let middle_rank_contains_trip = (hand.tally >> (middle_rank * 4) & 0b111) == 0b111;

                match (left_rank_contains_trip, middle_rank_contains_trip) {
                    (true, false) => {
                        TieBreaker(left_rank, (hand.ranks as u32) & ((1 << left_rank) - 1), 0)
                    }
                    (false, true) => TieBreaker(
                        middle_rank,
                        (hand.ranks as u32) & ((1 << middle_rank) - 1),
                        0,
                    ),
                    (false, false) => {
                        TieBreaker(right_rank, (hand.ranks as u32) & ((1 << right_rank) - 1), 0)
                    }
                    _ => unreachable!(),
                }
            }
            // NOTE: two pair is the only hand that can cascade twice & needs both cascaded ranks
            // located.
            HandRank::TwoPair => {
                let (left_rank, right_rank) =
                    (15 - hand.ranks.leading_zeros(), hand.ranks.trailing_zeros());
                // NOTE: zero highest bit so the middle bit is now the highest
                let remaining_ranks = hand.ranks & ((1 << left_rank) - 1);
                let middle_rank = 15 - remaining_ranks.leading_zeros();

                let left_rank_contains_pair = (hand.tally >> (left_rank * 4) & 0b11) == 0b11;
                let middle_rank_contains_pair = (hand.tally >> (middle_rank * 4) & 0b11) == 0b11;

                match (left_rank_contains_pair, middle_rank_contains_pair) {
                    (true, true) => TieBreaker(left_rank, middle_rank, right_rank),
                    (false, true) => TieBreaker(middle_rank, right_rank, left_rank),
                    (true, false) => TieBreaker(left_rank, right_rank, middle_rank),
                    _ => unreachable!(),
                }
            }
            // NOTE: one pair is also the most onerous to check because there are four different ranks, but the pair must be located.
            HandRank::OnePair => {
                let mut pair_rank = 15 - hand.ranks.leading_zeros();

                loop {
                    if pair_rank == 0 || (hand.tally >> (pair_rank * 4) & 0b11) == 0b11 {
                        break;
                    } else {
                        pair_rank -= 1;
                        continue;
                    }
                }

                TieBreaker(pair_rank, (hand.ranks as u32) & ((1 << pair_rank) - 1), 0)
            }
            HandRank::Unknown => TieBreaker(0, 0, 0),
        }
    }
}

impl std::cmp::PartialEq for TieBreaker {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2)
    }
}

impl std::cmp::PartialOrd for TieBreaker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0, &self.1, &self.2).cmp(&(other.0, &other.1, &other.2)))
    }
}
