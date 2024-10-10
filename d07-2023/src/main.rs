/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


/* *************************************************************************
                         ENUM AND METHODS
   ************************************************************************* */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Some(u8),
    T, J, Q, K, A
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
}


/* *************************************************************************
                           TRAITS
   ************************************************************************* */


/* *************************************************************************
                         STRUCTURE AND METHODS
   ************************************************************************* */
struct Hand {
    id: u32,
    entry: String,
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType
}

/* *************************************************************************
                           HELPER FUNCTIONS
   ************************************************************************* */


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
            Ok(l) => lines.push(l.to_lowercase()),
            Err(e) => panic!("ERROR: Cannot read contents of file {:?}!\n{:?}", &input_file, e)
        };
    }

    // Puzzle solution starts here
}