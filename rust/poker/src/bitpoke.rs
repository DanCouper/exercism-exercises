use anyhow::Result;
use std::fmt;

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
        _ => unreachable!()
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

#[derive(Copy, Debug, Clone)]
struct BitHand {
    hand_bf: u64,
    ranks_bf: u16,
}

impl fmt::Display for BitHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let legend1 = "               A    K    Q    J    T    9    8    7    6    5    4    3    2   ";
        let legend2 = "xxxx xxxx xxxx CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS CDHS";
        write!(f, "{}\n{}\n{}", legend1, legend2, format_hand_bf(self.hand_bf))

    }
}

impl BitHand {
  fn create_from_slice(hand: &[&str]) -> Result<BitHand> {
      let (ranks, suits) = prepare_input(hand)?;
      let (hand_bf, ranks_bf) = construct_bitfields(ranks, suits);
      
      Ok(BitHand { hand_bf, ranks_bf })
  }

  fn score(&self) -> u32 {
    match (score_from_count(self.hand_bf), is_straight(self.ranks_bf), is_flush(self.hand_bf)) {
        (5, false, false) => 0, // High card
        (6, _, _)         => 1, // One pair
        (7, _, _)         => 2, // Two pair
        (9, _, _)         => 3, // Three of a kind
        (5, true, false)  => 4, // Straight
        (5, false, true)  => 5, // Flush
        (10, _, _)        => 6, // Full house
        (16, _, _)        => 7, // Four of a kind
        (5, true, true)   => 8, // Straight flush
        _ => unreachable!(),
    }
  }
  
//   fn tie_breaker(&self, comparator: BitHand) {
      
//   }
} 

fn prepare_input(hand: &[&str]) -> Result<([u8; 5], [u8; 5])> {
    let mut ranks = [0; 5];
    let mut suits = [0; 5];
    for i in 0..=4 {
        let card_chars = hand[i].chars().collect::<Vec<char>>();
        ranks[i] = rank_value(card_chars[0]);
        suits[i] = suit_value(card_chars[1]);
    }
    Ok((ranks, suits))
}

fn construct_bitfields(ranks: [u8; 5], suits: [u8; 5]) -> (u64, u16) {
    (0..=4).fold((0, 0), |(hand_bf, ranks_bf), i| {
        (hand_bf | 1 << ((ranks[i] * 4) + suits[i]), ranks_bf | 1 << ranks[i])
    })
}

// Detects & produces the following values for these hands ONLY:
// - High card: 5
// - One pair: 6
// - Two pair: 7
// - Three of a kind: 9
// - Full house: 10
// - Four of a kind: 16
fn score_from_count(mut hand_bitfield: u64) -> u32 {
    (0..=12).fold(0, |total, _| {
        let cards_in_rank = (hand_bitfield & 0b1111).count_ones();
        hand_bitfield >>= 4;
        total + (1 << cards_in_rank) - 1
    })
}

// Detects presence of straight via masking (including of the low-ace form A2345)
fn is_straight(ranks: u16) -> bool {
    let ace_low_mask = 0b0001000000001111;
    let standard_mask = 0b11111;
    
    (ranks & ace_low_mask) == ace_low_mask || 
    ((ranks >> ranks.trailing_zeros()) & standard_mask)  == standard_mask
}

// Detects presence of flush by iterating nibbles
fn is_flush(mut hand: u64) -> bool {
    let mut v = 0;
    for _ in 0..=12 {
        let nibble = (hand & 0b1111);
        if nibble > 0 {
            
        } else {
            discrete_count += bits_set;
            hand >>= 4;
        }
    }
    discrete_count == 5
}


pub fn main() -> Result<()> {
    let high_card = BitHand::create_from_slice(&["4C", "6D", "5H", "7S", "KH"])?;
    println!("High card score: {}", high_card.score());
    let one_pair = BitHand::create_from_slice(&["4C", "4D", "5H", "6S", "KH"])?;
    println!("One pair score: {}", one_pair.score());
    let two_pair = BitHand::create_from_slice(&["4C", "4D", "5H", "5S", "KH"])?;
    println!("Two pair score: {}", two_pair.score());
    let three_of_a_kind = BitHand::create_from_slice(&["4C", "4D", "4H", "5S", "KH"])?;
    println!("Three of a kind score: {}", three_of_a_kind.score());
    let full_house = BitHand::create_from_slice(&["4C", "4D", "7H", "7S", "4H"])?;
    println!("Full house score: {}", full_house.score());
    let four_of_a_kind = BitHand::create_from_slice(&["4C", "4D", "4H", "4S", "KH"])?;
    println!("Four of a kind score: {}", four_of_a_kind.score());
    
    println!("------------------------------------");
    println!("{}", high_card);
    
    println!("0b0000100000111100 is a straight? {}", is_straight(0b0000100000111100));
    println!("0b0001000000001111 is a straight? {}", is_straight(0b0001000000001111));
    println!("0b0000000001111100 is a straight? {}", is_straight(0b0000000001111100));


    
    Ok(())
}

