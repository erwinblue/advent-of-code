/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


/* *************************************************************************
                         ENUM AND METHODS
   ************************************************************************* */
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Some(u8),
    T, J, Q, K, A
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
}


/* *************************************************************************
                           TRAITS
   ************************************************************************* */


/* *************************************************************************
                         STRUCTURE AND METHODS
   ************************************************************************* */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    id: u32,
    entry: String,
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType
}

impl Hand {
    fn new(id: usize, line: &str) -> Option<Hand> {
        /*
            Break line into hand and bid, assume each line is
                yyyyy zzzzz
              Where:
                yyyyy = Hand comprised of 5 cards
                zzzzz = bid i.e. integer
        */
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();

        // For debugging only, remove
        //println!("Parts: {:?}", &parts);

        // Parse the Hand of 5 cards
        /*let hand = match parts.get(0) {
            Some(h) => {
                let x = match parse_hand_entry(*h) {
                    Some(x) => x,
                    None => return None
                };
                x
            },
            None => return None
        };*/
        let hand = parse_hand_entry(parts.get(0)?)?;

        // For debugging only, remove
        //println!("Hand: {:?}", &hand);

        // Parse the bid i.e. wager of the hand
        /*let wager= match parts.get(1) {
            Some(b) => {
                let x = match b.to_string().parse::<u32>() {
                    Ok(y) => y,
                    Err(_) => return None
                };
                x
            },
            None => return None
        };*/
        let wager = match parts.get(1)?.to_string().parse::<u32>() {
            Ok(w) => w,
            Err(_) => return None
        };

        // Figure out the hand type of the card
        let hand_type = parse_hand_type(&hand);
        
        // Return our value
        Some(Hand {
            id: (id + 1) as u32,
            entry: line.to_owned(),
            cards: hand.to_vec(),
            bid: wager,
            hand_type: hand_type
        })
    }
}

/* *************************************************************************
                           HELPER FUNCTIONS
   ************************************************************************* */
fn parse_hand_type(hand: &Vec<Card>) -> HandType {
    // Get the counts of same cards
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    for card in hand.iter() {
        let c = card_counts.entry(*card).or_insert(0u8);
        *c += 1;
    }

    // For debugging only, remove
    //println!("card counts: {:?}", &card_counts);

    // Figure out the types we have
    let mut has_five = false;
    let mut has_four = false;
    let mut has_three = false;
    let mut has_pair = 0u8;
    for count in card_counts.values() {
        if *count == 5 {
            has_five = true;
        } else if *count == 4 {
            has_four = true;
        } else if *count == 3 {
            has_three = true;
        } else if *count == 2 {
            has_pair += 1;
        }
    }
    // Return the type we have
    if has_five {
        return HandType::FiveOfAKind;
    } else if has_four {
        return HandType::FourOfAKind;
    } else if has_three && has_pair == 1 {
        return HandType::FullHouse;
    } else if has_three && has_pair == 0 {
        return HandType::ThreeOfAKind;
    } else if has_pair == 2 {
        return HandType::TwoPair;
    } else if has_pair == 1 {
        return HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}

fn parse_hand_entry(entry: &str) -> Option<Vec<Card>> {
    let mut cards: Vec<Card> = vec![];
    for c in entry.chars().into_iter() {
        let card = match c {
            '2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let x = match c.to_string().parse::<u8>() {
                    Ok(y) => y,
                    Err(_) => return None
                };
                Card::Some(x)
            },
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => return None
        };
        cards.push(card);
    }

    // Minor sanity check if we have hand that has correct number of cards
    if cards.len() == 5 {
        Some(cards)
    } else {
        None
    }
}

fn solve_part1(lines: &Vec<String>) {
    // For debugging only, remove
    //println!("-------------------------------");
    //println!("Read Hands from puzzle input:");
    //println!("-------------------------------");

    let mut hands: Vec<Hand> = vec!{};
    for (i, line) in lines.iter().enumerate() {
        let hand = match Hand::new(i, line) {
            Some(h) => h,
            None => continue
        };

        // For debugging only, remove
        //println!("Line# {:?}: {:?} -> hand: {:?} bid: {:?} type: {:?}", (i + 1), &line, &hand.cards, &hand.bid, &hand.hand_type);

        hands.push(hand);
    }

    // Sort our hands by hand_type then, for similar hand_type, sort by individual card value
    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type)
        .then(a.cards.cmp(&b.cards))
    );

    // For debugging only, remove
    //println!("-------------------------------");
    //println!("Hands Sorted by HandType then by Card values:");
    //println!("-------------------------------");

    let mut winnings: BTreeMap<u32, &Hand> = BTreeMap::new();
    for (rank, h) in hands.iter().enumerate() {
        let _ = winnings.entry(rank as u32 + 1).or_insert(h);

        // For debugging only, remove
        //println!("{:?} -> hand: {:?} bid: {:?} type: {:?}", &h.entry, &h.cards, &h.bid, &h.hand_type);
    }

    let mut total_winnings = 0u32;
    for (i, h) in winnings.iter() {
        total_winnings += i * h.bid;
    }
    println!("-------------------------------");
    println!("Total winnings: {:?}", total_winnings);
    println!("-------------------------------\n");

}


/* *************************************************************************
                          MAIN PROGRAM
   ************************************************************************* */
fn main() {
    // Get the input file name
    let input_file = match env::args().nth(1) {
        Some(f) => f,
        None => panic!("ERROR: Program requires an argument: <input_file>")
    };

    // Try to open the input file and get a File object
    let f = match File::open(&input_file) {
        Ok(k) => k,
        Err(e) => panic!("ERROR: Cannot open file {:?}!\n{:?}", &input_file, e),
    };

    // Put contents of file to a vector
    let mut lines: Vec<String> = vec![];
    for line in BufReader::new(f).lines() {
        match line {
            Ok(l) => lines.push(l.to_string()),
            Err(e) => panic!("ERROR: Cannot read contents of file {:?}!\n{:?}", &input_file, e)
        };
    }

    // Puzzle solution starts here
    solve_part1(&lines);

}