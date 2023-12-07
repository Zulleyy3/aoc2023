use std::{cmp::{self, Ordering}, error, fs};

#[derive(Eq,Debug)]
struct Card {
    hand: [u32; 5],
    rating: HandType,
    bid: u32,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)] 
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind
}

impl Ord for Card {
    fn cmp(&self, other:&Self) -> Ordering {
        if self.rating == other.rating {
            for i in 0..self.hand.len() {
                if self.hand[i] != other.hand[i] {
                    return self.hand[i].cmp(&other.hand[i]);
                }
            }
            return Ordering::Equal;
        }
        return self.rating.cmp(&other.rating);
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rating == other.rating && self.hand == other.hand
    }
}


impl Card {
    fn new(input: &str, jokers: bool) -> Self {
         let mut input =  input.split_whitespace();
         let hand = input.next().unwrap();
         let bid = input.next().unwrap();
         let mut card = Card {
            bid: u32::from_str_radix(bid, 10).unwrap(),
            hand: [0,0,0,0,0],
            rating: HandType::HighCard,
         };
         for (i, c) in hand.chars().enumerate() {
             card.hand[i] = match c {
                 'A' => 12,
                 'K' => 11,
                 'Q' => 10,
                 'J' => if jokers {0} else {09},
                 'T' => if jokers {09} else {08},
                 c @ '2'..='9' => c.to_digit(10).unwrap() - if jokers {1} else {2},
                 _ => panic!("invalid input")
            }
             
        }
        let mut counts = [0;13];
        
        for v in card.hand.iter() {
            counts[*v as usize] += 1;
        }
 
        let joker_count = if !jokers {0} else {counts[0]};

        let mut highest: u32 = 1;
        let mut second_highest: u32 = 1; 

        //ignore jokers during when determining highest cards
        let to_iter = if jokers {&counts[1..]} else {&counts};
        for count in to_iter.iter() {
            if *count > highest {
                second_highest = highest;
                highest = *count;
            }
            else if *count > second_highest {
                second_highest = *count;
            }
        }

        card.rating = match highest {
            5  => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => if second_highest == 2 {HandType::FullHouse} else {HandType::ThreeOfKind},
            2 => if second_highest == 2 {HandType::TwoPair} else {HandType::OnePair},
            _ => HandType::HighCard
        };

        if jokers {
            card.rating = match (card.rating, joker_count) {
                (HandType::FourOfKind, 1)  => HandType::FiveOfKind,
                (HandType::ThreeOfKind, 2) => HandType::FiveOfKind,
                (HandType::ThreeOfKind, 1) => HandType::FourOfKind,
                (HandType::TwoPair, 1) => HandType::FullHouse,
                (HandType::OnePair, 3) => HandType::FiveOfKind,
                (HandType::OnePair, 2) => HandType::FourOfKind,
                (HandType::OnePair, 1) => HandType::ThreeOfKind,
                (_, 5) => HandType::FiveOfKind,
                (_, 4) => HandType::FiveOfKind,
                (_, 3) => HandType::FourOfKind,
                (_, 2) => HandType::ThreeOfKind,
                (_, 1) => HandType::OnePair,
                (t, _) => t,
            }
        }
        
        card
    }
}


fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // let input = fs::read_to_string("test.txt")?;

    let mut cards: Vec<Card> = input.lines().map(|l| Card::new(l, false)).collect();
    cards.sort();
    let part1 = cards.iter().fold((1,0), |v, card| (v.0 +1, v.1 + v.0* card.bid)).1;
    println!("Part 1 {}", part1);
    
    let mut cards: Vec<Card> = input.lines().map(|l| Card::new(l, true)).collect();
    cards.sort();
    let part2 = cards.iter().fold((1,0), |v, card| (v.0 +1, v.1 + v.0* card.bid)).1;
    println!("Part 2 {}", part2);
    Ok(())
}