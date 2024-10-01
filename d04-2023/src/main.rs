/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
use std::collections::VecDeque;
use std::env;
use std::fs:: File;
use std::io::{BufRead, BufReader};
   
   
/* *************************************************************************
                              HELPER FUNCTIONS       
   ************************************************************************* */
fn list_numbers(s: &str) -> Vec<u32> {
    let numbers = s.split_whitespace().into_iter()
        .map(|x| {
            let y = match x.parse::<u32>() {
                Ok(z) => z,
                Err(_) => 0u32
            };
            y
        })
        .collect();
    numbers
}
   
fn get_numbers(parts: &Vec<&str>, idx: usize) -> Option<Vec<u32>> {
    let nums = match parts.get(idx) {
        Some(x) => list_numbers(*x),
        None => return None
    };
    Some(nums)
}

/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */

/* *************************************************************************
                              TRAITS
   ************************************************************************* */
   
/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */
   
/* -------------------------------------------------------------------------
   Winning - List of winning numbers
   ------------------------------------------------------------------------- */
#[derive(Debug, Clone)]
struct Winning {
    numbers: Vec<u32>
}

/* -------------------------------------------------------------------------
   Hand - List of potential numbers
   ------------------------------------------------------------------------- */
#[derive(Debug, Clone)]
struct Hand {
    numbers: Vec<u32>
}

/* -------------------------------------------------------------------------
   Card - Contains Winning numbers and Hand
   ------------------------------------------------------------------------- */
#[derive(Clone)]
struct Card {
    id: u32,
    winning: Winning,
    hand: Hand,
    matched: Vec<u32>
}

impl Card {
    fn make_card(line: &str) -> Option<Card> {
        // Break the line with ':'.  First part is the card id #
        // Second part are the winning numbers and hand.
        let parts: Vec<&str> = line.split(':').collect();
        // Parse the card id #
        let card_id = match parts.get(0) {
            Some(h) => {
                let i = h.to_lowercase().replace("card", "");
                // TODO: Remove this for debugging only
                //println!("Debug: i: {:?}", i);
                let id = match i.trim().parse::<u32>() {
                    Ok(i) => i,
                    Err(_) => 0u32
                };
                id
            },
            None => return None
        };
        // Parse the card winning and hand
        let body = match parts.get(1) {
            Some(b) => *b,
            None => return None
        };
        let body_parts: Vec<&str> = body.split('|').into_iter().collect();
        let winning_numbers = match get_numbers(&body_parts, 0usize) {
                Some(c) => Winning { numbers: c },
                None => return None
        };
        let hand_numbers = match get_numbers(&body_parts, 1usize) {
            Some(h) => Hand { numbers: h },
            None => return None
        };
        // Create the card object
        Some(Card {
            id: card_id,
            winning: winning_numbers,
            hand: hand_numbers,
            matched: vec![]
        })
    }

    fn matched_numbers(&mut self) {
        let mut matched: Vec<u32> = vec![];
        for hand_number in self.hand.numbers.iter() {
            if self.winning.numbers.iter().any(|x| x == hand_number) {
                matched.push(*hand_number);
            }
        }
        self.matched = matched.to_owned();
    }

    fn card_points(&mut self) -> u32 {
        let points: u32 = match self.matched.len() as u32 {
            0 => 0u32,
            1 => 1u32,
            2..=u32::MAX => {
                2u32.pow(self.matched.len() as u32 - 1)
            }
        };
        points
    }

    fn cards_won(&self, max: u32) -> Vec<u32> {
        let mut won_cards: Vec<u32> = vec![];
        let numbers_matched = self.matched.len() as u32;
        if numbers_matched > 0 {
            for x in 1..=numbers_matched {
                if self.id + x > max {
                    break;
                } else {
                    won_cards.push(self.id + x);
                }
            }
        }
        // TODO: Remove these. debug print 
        //println!("Card# {:?}:", self.id);
        //println!("\tMatched cards: {:?}", self.matched);
        //println!("\t    Cards won: {:?}", won_cards);
        won_cards
    }

    fn process_copies(cards_won: Vec<u32>, pile_cards: Vec<Card>) -> VecDeque<Card> {
        let mut cards: VecDeque<Card> = VecDeque::new();
        for c in cards_won.iter() {
            for p in pile_cards.iter() {
                if p.id == *c {
                    cards.push_back(p.clone());
                }
            }
        }
        cards
    }
}


/* -------------------------------------------------------------------------
   Pile - A pile of (scratch) Cards
   ------------------------------------------------------------------------- */
#[derive(Clone)]
struct Pile {
    cards: Vec<Card>,
    max_id: u32
}

impl Pile {
    fn read_cards(file: File) -> Option<Pile> {
        let mut scratch_cards: Vec<Card> = vec![];
        for line in BufReader::new(file).lines() {
            let card_line = match line {
                Ok(s) => s,
                Err(_) => return None
            };
            let card: Card = match Card::make_card(&card_line) {
                Some(c) => c,
                None => continue
            };
            scratch_cards.push(card);
        }
        let last_card = match scratch_cards.get(scratch_cards.len() - 1) {
            Some(l) => l.id,
            None => 0u32
        };
        // TODO: Remove these. debug print 
        println!("Last card id: {:?}", last_card);
        Some(Pile { cards: scratch_cards, max_id: last_card })
    }

    fn matched_cards(&mut self) -> u32 {
        let mut total = 0u32;
        for c in self.cards.iter_mut() {
            c.matched_numbers();
            // TODO: Remove these. debug print the pile but we do need total points
            //println!("Card# {:?}", c.id);
            //println!("\t{:?}", c.winning);
            //println!("\t{:?}", c.hand);
            //println!("\tMatched {:?}", c.matched);
            total += c.card_points();

        }
        total
    }

    // For part 2
    fn total_cards(&mut self) -> u32 {
        // make our struct smaller since we're doing a lot of copies at this point of Rust adventure
        for c in self.cards.iter_mut() {
            c.winning = Winning { numbers: vec![] };
            c.hand = Hand { numbers: vec![] };
        }
        let mut total = 0u32;
        let mut card_pile: VecDeque<Card> = VecDeque::from(self.cards.clone());
        while !card_pile.is_empty() {
            let card = match card_pile.pop_front() {
                Some(c) => c,
                None => break
            };
            total += 1;
            let mut card_list = Card::process_copies(card.cards_won(self.max_id), self.cards.clone());
            card_pile.append(&mut card_list);
        }
        total
    }
}


/* *************************************************************************
                                 MAIN PROGRAM         
   ************************************************************************* */
fn main() {
    // Get the input file name
    let input_file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("ERROR: Program requires an argument: <input_file>");
            std::process::exit(1);
        }
    };

    // Try to open the input file and get a File object
    let f = match File::open(&input_file) {
        Ok(k) => k,
        Err(e) => panic!("ERROR: Cannot open file {:?}!\n{:?}", &input_file, e)
    };

    // Read in the pile of cards i.e. input file
    let mut pile = match Pile::read_cards(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };
    
    // Part 1: Get the total points for the cards matched numbers
    let total_points = pile.matched_cards();
    println!("Total (bogus) points: {:?}", total_points);

    // Part 2: Get the cards with matched numbers and the cards they won
    let total_cards = pile.total_cards();
    println!("Total scratch cards won: {:?}", total_cards);

}
