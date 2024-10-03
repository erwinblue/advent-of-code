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

trait Lookup {
    fn map_source(&self) -> String;
    fn map_destination(&self) -> String;
    fn convert_value(&self, value: u32) -> Option<u32>;
}

/* *************************************************************************
                         STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
   Range - a Map range definition from the input Almanac
       source - source range start
       destination - destination range start
       lenght - range lenght
   ------------------------------------------------------------------------- */
#[derive(Clone, Copy)]
struct Range {
    source: u32,
    destination: u32,
    lenght: u32
}

impl Range {
    fn process_range(range_entry: &str) -> Option<Range> {
        todo!();
    }
}

/* -------------------------------------------------------------------------
   Seed - contains seed name and mapped values
       id - the seed# (assume this comes from the input 'Almanac' file)
       categories - derived category values based on the map relationship
   ------------------------------------------------------------------------- */
struct Seed {
    id: u32,
    categories: HashMap<String, u32>
}

impl Seed {
    fn get_seeds(line: &str) -> Option<Vec<Seed>> {
        todo!();
    }
}


/* -------------------------------------------------------------------------
   MapInstance - generic map of relationship and range(s)
       id - internally mapped id number
       start_category - the category name of source
       destination_category - the category name of destination
       ranges - the range specified from the input Almanac file
   ------------------------------------------------------------------------- */
struct MapInstance {
    id: u8,
    start_category: String,
    destination_category: String,
    ranges: Vec<Range>,
}

impl MapInstance {
    fn new_instance(id_number: u8) -> MapInstance {
        MapInstance {
            id: id_number, 
            start_category: "".to_string(),
            destination_category: "".to_string(),
            ranges: vec![]
        }
    }

    fn map_categories(header: &str) -> HashMap<String, String> {
        todo!();
    }
}

/* -------------------------------------------------------------------------
   Almanac - the input document containing seeds and map information
       seeds = a list of seed numbers
       maps = a list of maps
       start = the starting category, for day 5 it is 'seed'
       destination - the end category, for day 5, it is 'location'
       path - list of Map id's on how to get from start to destination

    Note:
       - Assume there is no direct path from start ('seed') to desitination ('location')
         i.e. there is no Map between seed and location
       - We do not create hard-coded Maps in the code.
   ------------------------------------------------------------------------- */
struct Almanac {
    seeds: Vec<Seed>,
    maps: Vec<MapInstance>,
    start: String,
    destination: String,
    path: Vec<u8>
}

impl Almanac {
    // read and parse the contents of the input 'Almanac' file
    fn read_file(file: File) -> Option<Almanac>  {
        //todo!();
        let mut almanac = Almanac {
            seeds: vec![],
            maps: vec![],
            start: "".to_string(),
            destination: "".to_string(),
            path: vec![]
        };
        let mut map_header: HashMap<String, String> = HashMap::new();
        let mut map_range_buffer: Vec<Range> = vec![];
        let mut map_counter = 0u8;
        for line in BufReader::new(file).lines() {
            // check result of buf reader of line in file
            let entry = match line {
                Ok(s) => s,
                Err(_e) => continue
            };

            // skip if blank line i.e. doesn't contain any alphanumeric characters
            let is_not_blank = entry.to_lowercase()
                .chars()
                .any(|c|c.is_ascii_alphanumeric());
            if !is_not_blank {
                continue;
            }

            // get seed list if line contains seeds entry
            if entry.to_lowercase().contains("seeds:") {
                match Seed::get_seeds(entry.trim()) {
                    Some(v) => { almanac.seeds = v; continue; },
                    None => continue
                };
            // get map category and reset range buffer if not a new range
            } else if entry.to_lowercase().contains(" map:") {
                if !map_range_buffer.is_empty() {
                    // flush to make a new MapInstance and add to almanac maps
                    let source = match map_header.get("source_category") {
                        Some(c) => c,
                        None => &"source".to_string()
                    };
                    let dest = match map_header.get("destination_category") {
                        Some(c) => c,
                        None => &"destination".to_string()
                    };
                    almanac.maps.push(MapInstance {
                        id: map_counter,
                        start_category: source.to_owned(),
                        destination_category: dest.to_owned(),
                        ranges: map_range_buffer.to_vec()
                    });
                    // start a new set of map ranges
                    map_counter += 1;
                    map_range_buffer.clear();
                    map_header.clear();
                }     
                map_header = MapInstance::map_categories(&entry);
            // parse a source, destination, and length of range entry line
            } else if entry.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
                let range_entry: Range = match Range::process_range(&entry) {
                    Some(r) => r,
                    None => continue
                };
                map_range_buffer.push(range_entry);
            }
        }
        Some(almanac)
    }

    // get list of map id's (u8) which tells the path to use to get
    // from source category to destination category.
    fn make_path(&self, start: &str, end: &str) -> Option<Vec<u8>> {
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
