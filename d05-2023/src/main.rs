/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::collections::HashMap;
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
   Seed - contains seed name and mapped values
       id - the seed#
       categories - derived category values based on the map relationship
   ------------------------------------------------------------------------- */
struct Seed {
    id: u32,
    categories: HashMap<String, u32>
}


/* -------------------------------------------------------------------------
   Map - generic map of relationship its range
       start_category - the category name of source
       destination_category - the category name of destination
       start_range_min - derived starting range of map
       destination_range_max - derived end range of map
   ------------------------------------------------------------------------- */
struct Map {
    id: u32,
    start_category: String,
    destination_category: String,
    start_range_min: u32,
    destination_range_max: u32
}

/* -------------------------------------------------------------------------
   Almanac - the input document containing seeds and map information
       seeds = a list of seed numbers
       maps = a list of maps
       start = the starting category, for day 5 it is 'seed'
       destination - the end category, for day 5, it is 'location'
       path - list of Map id's on how to get from start to destination
              (assume there is no direct path/map from 'seed' to 'location')
   ------------------------------------------------------------------------- */
struct Almanac {
    seeds: Vec<Seed>,
    maps: Vec<Map>,
    start: String,
    destination: String,
    path: Vec<u32>
}

impl Almanac {
    fn read_file(file: File) -> Option<Almanac>  {
        todo!();
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
    let almanac = match Almanac::read_file(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };
}
