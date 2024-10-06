/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


/* *************************************************************************
                           HELPER FUNCTIONS
   ************************************************************************* */


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
   Puzzle - The description and details of the Advent of Code Puzzle
   ------------------------------------------------------------------------- */
struct Puzzle {
    name: String
}


impl Puzzle {
    fn read_file(file: File) -> Option<Puzzle> {
        for line in BufReader::new(file).lines() {
            todo!();
        }
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
        Err(e) => panic!("ERROR: Cannot open file {:?}!\n{:?}", &input_file, e),
    };

    // Read in the input file
    let mut puzzle_input = match Puzzle::read_file(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };
}
