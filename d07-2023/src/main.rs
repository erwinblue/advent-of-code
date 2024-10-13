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
    // Part 2:  Move Jack to the lowest valued card per challenge
    J,
    Some(u8),
    T, Q, K, A
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
        // Break line into hand and bid/wager
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
        // Parse the Hand of 5 cards
        let hand = parse_hand_entry(parts.get(0)?)?;
        // Parse the bid i.e. wager of the hand
        let wager = match parts.get(1)?.to_string().parse::<u32>() {
            Ok(w) => w,
            Err(_) => return None
        };
        // Figure out the hand type of the card
        let hand_type = parse_hand_type(&hand);
        // Return our value
        Some(Hand { id: (id + 1) as u32,
            entry: line.to_owned(), cards: hand.to_vec(),
            bid: wager,
            hand_type: hand_type
        })
    }
}

/* *************************************************************************
                           HELPER FUNCTIONS
   ************************************************************************* */
// Card counts to HandType
fn card_counts_to_handtype(counts: &Vec<u8>) -> HandType {
    if *counts == vec![5] {
        HandType::FiveOfAKind
    } else if *counts == vec![1, 4] {
        HandType::FourOfAKind
    } else if *counts == vec![2, 3] {
        HandType::FullHouse
    } else if *counts == vec![1, 1, 3] {
        HandType::ThreeOfAKind
    } else if *counts == vec![1, 2, 2] {
        HandType::TwoPair
    } else if *counts == vec![1, 1, 1, 2] {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

// A set of (5) cards i.e. Hand, to HandType enum
// Part 2: Jack can pretend to be whatever card is best to make hand the best HandType
fn parse_hand_type(hand: &Vec<Card>) -> HandType {
    // Get the counts of same cards
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    let mut jacks = 0u8;
    for card in hand.iter() {
        if *card == Card::J {
            jacks += 1;
        } else {
            *card_counts.entry(*card).or_insert(0u8) += 1;
        }
    }
    let mut counts: Vec<u8> = card_counts.values().into_iter().map(|x|*x).collect();
    counts.sort();
    if jacks == 5 {
        HandType::FiveOfAKind
    } else {
        counts[card_counts.len() - 1] = counts[card_counts.len()- 1] + jacks;
        card_counts_to_handtype(&counts)
    }
}

// From character primitive to a vector of Card enum
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

/* *************************************************************************
                               SOLVE FOR PART 1
   ************************************************************************* */
fn solve_part1(lines: &Vec<String>) {
    let mut hands: Vec<Hand> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let hand = match Hand::new(i, line) {
            Some(h) => h,
            None => continue
        };
        hands.push(hand);
    }

    // Sort our hands by hand_type then, for similar hand_type, sort by individual card value
    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type).then(a.cards.cmp(&b.cards)));

    // Get the winnings per hand, where: (winnings per hand) = (hand rank) * bid
    let mut winnings: BTreeMap<u32, &Hand> = BTreeMap::new();
    for (rank, h) in hands.iter().enumerate() {
        let _ = winnings.entry(rank as u32 + 1).or_insert(h);
    }

    // Get the total winnings and print the result 
    let total_winnings = winnings.iter().map(|(x, y)| *x * y.bid)
        .fold(0, |acc, a| acc + a);
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