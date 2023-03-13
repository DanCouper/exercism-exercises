use std::fmt;

fn char_val_to_normalised(binval: u8) -> u8 {
    match binval {
        // ranks
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        // suits
        b'S' => 0,
        b'H' => 1,
        b'D' => 2,
        b'C' => 3,
        // all possible input characters are known in advance: reasonably safe
        // to assume that any value not matched above will never *be* matched:
        _ => unreachable!(),
    }
}

fn not_space_char(binval: &u8) -> bool {
    *binval != b' '
}

fn is_straight(ranks: u16) -> bool {
    let hi_mask = 0b11111;
    let lo_mask = 0b0001000000001111;

    (ranks & lo_mask) == lo_mask || ((ranks >> ranks.trailing_zeros()) & hi_mask) == hi_mask
}

fn is_flush(suits: u8) -> bool {
    suits.count_ones() == 1
}

#[derive(Debug)]
pub struct Hand {
    tally: u64,
    ranks: u16,
    suits: u8,
    score: u32,
}

impl Hand {
    fn from_slice(hand: &str) -> Hand {
        hand.bytes()
            .into_iter()
            .filter(not_space_char)
            .map(char_val_to_normalised)
            .enumerate()
            .fold(Hand::init(), |mut hand, (i, cval)| {
                // if i is even, looking at rank. If i is odd, looking at suit
                if i % 2 == 0 {
                    hand.set_tally(cval);
                    hand.set_rank(cval);
                    hand.update_raw_score(cval);
                } else {
                    hand.set_suit(cval);
                }

                // Last element in byteslice iterator, can set the score properly:
                if i == 9 {
                    hand.normalise_raw_score();
                }
                hand
            })
    }

    fn init() -> Hand {
        Hand {
            tally: 0,
            ranks: 0,
            suits: 0,
            score: 0,
        }
    }

    fn set_tally(&mut self, rank: u8) -> &mut Hand {
        let mut nibble_start = rank * 4;
        let current_tally = (self.tally >> nibble_start & 0xf).trailing_ones() as u8;
        nibble_start += current_tally;

        self.tally |= 1 << nibble_start;
        self
    }

    fn set_rank(&mut self, rank: u8) -> &mut Hand {
        self.ranks |= 1 << rank;
        self
    }

    fn set_suit(&mut self, suit: u8) -> &mut Hand {
        self.suits |= 1 << suit;
        self
    }

    // NOTE: IT IS *EXTREMELY* IMPORTANT THAT THE POSSIBLE RETURN VALUES FOR
    // THIS ARE KNOWN, OTHERWISE MOST THINGS WILL FUCK UP WHEN ATTEMPTING TO
    // NORMALISE SCORE.
    fn update_raw_score(&mut self, rank: u8) -> &mut Hand {
        let rank_nibble = (self.tally >> (rank * 4)) & 0xf;

        self.score += 1 << rank_nibble;
        self
    }

    // NOTE: only do this at the end!
    fn normalise_raw_score(&mut self) -> &mut Hand {
        let adjusted_score = match (self.score, is_straight(self.ranks), is_flush(self.suits)) {
            (10, false, false) => 1, // high card
            (16, _, _) => 2,         // one pair
            (22, _, _) => 3,         // two pair
            (142, _, _) => 4,        // three-of-a-kind
            (10, true, false) => 5,  // straight
            (10, false, true) => 6,  // flush
            (148, _, _) => 7,        // full house
            (32908, _, _) => 8,      // four-of-a-kind
            (10, true, true) => 9,   // straight flush
            (score, iss, isf) => {
                println!(
                    "WTF!, score is: {}, we think straight is {}, flush is {}",
                    score, iss, isf
                );
                0
            }
        };

        self.score = adjusted_score;
        self
    }
}

/*----------------------------------------------------------------------------*\
 * Horrible stuff for dirty debugging from hereon in:
\*----------------------------------------------------------------------------*/

fn horrible_string_chunker(input: String, chunks: usize) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        result.push(c);
        if (i + 1) % chunks == 0 {
            result.push(' ');
        }
    }
    result
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let divider = "--------------------------------------------------------------------------------------------------";
        let sdivider = "-------------------------------------------------------------------------";

        let score_desc = match self.score {
            1 => "High card",
            2 => "One pair",
            3 => "Two pair",
            4 => "Three of a kind",
            5 => "Straight",
            6 => "Flush",
            7 => "Full house",
            8 => "Four of a kind",
            9 => "Straight flush",
            _ => panic!("Something has gone very wrong with the score calculation, sort it out!"),
        };

        let hand_rank = format!("For: {}\n{}", score_desc, divider);

        let tally_legend = "   A    K    Q    J    T    9    8    7    6    5    4    3    2";
        let tally_pprint = horrible_string_chunker(format!("{:052b}", self.tally), 4);
        let tally = format!(
            "         {}\nTallies: {}\n{}",
            tally_legend, tally_pprint, sdivider
        );

        let ranks_legend = "A K Q J T 9 8 7 6 5 4 3 2";
        let ranks_pprint = horrible_string_chunker(format!("{:013b}", self.ranks), 1);
        let ranks = format!(
            "         {}\nRanks:   {}\n{}",
            ranks_legend, ranks_pprint, sdivider
        );

        let suits_legend = "S H D C";
        let suits_pprint = horrible_string_chunker(format!("{:04b}", self.suits), 1);
        let suits = format!(
            "         {}\nSuits:   {}\n{}",
            suits_legend, suits_pprint, divider
        );

        let score = format!("Overall score: {}\n{}", self.score, divider);

        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n\n",
            divider, hand_rank, tally, ranks, suits, score
        )
    }
}
