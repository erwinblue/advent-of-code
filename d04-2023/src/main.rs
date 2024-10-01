/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
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
#[derive(Debug)]
struct Winning {
    numbers: Vec<u32>
}

impl Winning {
    fn get_numbers(input: &str) -> Option<Winning> {
        let parts: Vec<&str> = input.split('|').into_iter().collect();
        let nums = match parts.get(0) {
            Some(x) => list_numbers(*x),
            None => return None
        };
        Some(Winning { numbers: nums })
    }
}

/* -------------------------------------------------------------------------
   Hand - List of potential numbers
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Hand {
    numbers: Vec<u32>
}

impl Hand {
    fn get_numbers(input: &str) -> Option<Hand> {
        let parts: Vec<&str> = input.split('|').into_iter().collect();
        let nums = match parts.get(1) {
            Some(x) => list_numbers(*x),
            None => return None
        };
        Some(Hand { numbers: nums })
    }
}

/* -------------------------------------------------------------------------
   Card - Contains Winning numbers and Hand
   ------------------------------------------------------------------------- */
struct Card {
    id: u32,
    winning: Winning,
    hand: Hand,
    matched: Vec<u32>,
    points: u32
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
        let winning_numbers= match Winning::get_numbers(body) {
            Some(w) => w,
            None => return None
        };
        let hand_numbers = match Hand::get_numbers(body) {
            Some(h) => h,
            None => return None
        };

        let mut new_card = Card {
            id: card_id,
            winning: winning_numbers,
            hand: hand_numbers,
            matched: vec![],
            points: 0u32
        };
        new_card.matched = new_card.matched_numbers();
        new_card.points = new_card.card_points();
        Some(new_card)
    }

    fn matched_numbers(&self) -> Vec<u32> {
        let mut matched: Vec<u32> = vec![];
        for hand_number in self.hand.numbers.iter() {
            if self.winning.numbers.iter().any(|x| x == hand_number) {
                matched.push(*hand_number);
            }
        }
        matched
    }

    fn card_points(&self) -> u32 {
        let points: u32 = match self.matched.len() as u32 {
            0 => 0u32,
            1 => 1u32,
            2..=u32::MAX => {
                2u32.pow(self.matched.len() as u32 - 1)
            }
        };
        points
    }

}


/* -------------------------------------------------------------------------
   Pile - A pile of Cards
   ------------------------------------------------------------------------- */
struct Pile {
    cards: Vec<Card>
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
        Some(Pile { cards: scratch_cards })
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
    let pile = match Pile::read_cards(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };
    
    // Get the total points for the cards matched numbers
    let mut total_points = 0u32;
    for c in pile.cards.iter() {
        // debug print the pile but we do need total points
        //println!("Card# {:?}", c.id);
        //println!("\t{:?}", c.winning);
        //println!("\t{:?}", c.hand);
        //println!("\tMatched {:?} | Points: {:?}", c.matched, c.points);
        total_points += c.points;
    }
    println!("Total points: {:?}", total_points);
}
