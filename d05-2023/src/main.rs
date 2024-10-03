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

/*trait Lookup {
    //fn map_source(&self) -> String;
    //fn map_destination(&self) -> String;
    fn convert_value(&self, value: u32) -> Option<u32>;
}*/

/* *************************************************************************
                         STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
   Range - a Map range definition from the input Almanac
       source - source range start
       destination - destination range start
       length - range length
   ------------------------------------------------------------------------- */
#[derive(Clone, Copy, Debug)]
struct Range {
    source: u32,
    destination: u32,
    length: u32
}

impl Range {
    fn process_range(range_entry: &str) -> Option<Range> {
        let parts: Vec<&str> = range_entry.split_whitespace().into_iter().collect();
        if parts.len() == 3 {
            return Some(Range {
                source: parts.get(1).unwrap_or(&"0").parse::<u32>().unwrap_or(0u32),
                destination: parts.get(0).unwrap_or(&"0").parse::<u32>().unwrap_or(0u32),
                length: parts.get(2).unwrap_or(&"0").parse::<u32>().unwrap_or(0u32)
            });
        } else {
            return None;
        }
    }

    fn within_range(&self, value: u32) -> Option<u32> {
        let destination_value = if self.source <= value && value <= (self.source + self.length) {
            self.destination + (value - self.source)
        } else {
            return None
        };
        Some(destination_value)
    }
}

/* -------------------------------------------------------------------------
   Seed - contains seed name and mapped values
       id - the seed# (assume this comes from the input 'Almanac' file)
       categories - derived category values based on the map relationship
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Seed {
    id: u32,
//    categories: HashMap<String, u32>
}

impl Seed {
    fn get_seeds(line: &str) -> Option<Vec<Seed>> {
        let mut seeds: Vec<Seed> = vec![];
        let numbers = match line.split(':').last() {
            Some(n) => n,
            None => return None
        };
        for num in numbers.trim().split_whitespace().into_iter() {
            //if num.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
            if num.chars().any(|x|x.is_digit(10)) {
                seeds.push(Seed {
                    id: num.parse::<u32>().unwrap_or(0u32),
 //                   categories: HashMap::new()
                });
            }
        }
        Some(seeds)
    }
}


/* -------------------------------------------------------------------------
   MapInstance - generic map of relationship and range(s)
       id - internally mapped id number
       start_category - the category name of source
       destination_category - the category name of destination
       ranges - the range specified from the input Almanac file
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct MapInstance {
    id: u8,
    start_category: String,
    destination_category: String,
    ranges: Vec<Range>,
}

impl MapInstance {
    fn get_categories(header: &str) -> HashMap<String, String> {
        let mut map_header: HashMap<String, String> = HashMap::new();
        let parsed_header = header.replace(" map:", "");
        let parts: Vec<&str> = parsed_header.split('-').into_iter().collect();
        if parts.len() == 3 {
            let first = parts.get(0).unwrap_or(&"");
            let last = parts.get(2).unwrap_or(&"");
            map_header.insert("source".to_string(), first.to_string());
            map_header.insert("destination".to_string(), last.to_string());
        }
        map_header
    }

    // Per challenge problem, if value is not within range, destination value = source value
    fn convert_value(&self, value: u32) -> (String, u32) {
        let name = self.destination_category.to_owned();
        let mut extrapolated = value;
        for r in self.ranges.iter() {
            match r.within_range(value) {
                Some(v) => {
                    extrapolated = v;
                    break;
                },
                None => continue
            };
        }
        (name, extrapolated)
    }
}

/* -------------------------------------------------------------------------
   Almanac - the input document containing seeds and map information
       seeds = a list of seed numbers
       maps = a list of maps
       start = the starting category, for day 5 exercise it is 'seed'
       destination - the end category, for day 5 exercise, it is 'location'
       route - list of Map id's on how to get from start to destination

    Note:
       - Assume there is no direct path from start ('seed') to desitination ('location')
         i.e. there is no Map between seed and location
       - We do not create hard-coded Maps in the code.
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Almanac {
    seeds: Vec<Seed>,
    maps: Vec<MapInstance>,
    start: String,
    destination: String,
    route: Vec<u8>
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
            route: vec![]
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
            if !entry.trim().chars().any(|c|c.is_ascii_alphanumeric()) {
                continue;
            }

            // get seed list if line contains seeds entry
            if entry.to_lowercase().contains("seeds:") {
                almanac.seeds = Seed::get_seeds(entry.trim()).unwrap_or(vec![]);
            // get map category and reset range buffer if not a new range
            } else if entry.to_lowercase().contains(" map:") {
                if !map_range_buffer.is_empty() {
                    // flush to make a new MapInstance and add to almanac maps
                    almanac.maps.push(MapInstance {
                        id: map_counter,
                        start_category: map_header.get("source")
                            .unwrap_or(&"source".to_string()).to_owned(),
                        destination_category: map_header.get("destination")
                            .unwrap_or(&"destination".to_string()).to_owned(),
                        ranges: map_range_buffer.to_vec()
                    });
                    // start a new set of map ranges
                    map_counter += 1;
                    map_range_buffer.clear();
                    map_header.clear();
                }     
                map_header = MapInstance::get_categories(&entry);
            // parse a source, destination, and length of range entry line
            } else if entry.chars().any(|x|x.is_digit(10)) {
                let range_entry: Range = match Range::process_range(&entry) {
                    Some(r) => r,
                    None => continue
                };
                map_range_buffer.push(range_entry);
            }
        }
        // flush one last time at the end of the file
        if !map_range_buffer.is_empty() {
            almanac.maps.push(MapInstance {
                id: map_counter,
                start_category: map_header.get("source")
                    .unwrap_or(&"source".to_string()).to_owned(),
                destination_category: map_header.get("destination")
                    .unwrap_or(&"destination".to_string()).to_owned(),
                ranges: map_range_buffer.to_vec()
            });
        }     
        Some(almanac)
    }

    // get list of map id's (u8) which tells the path to use to get
    // from source category to destination category.
    fn make_path(&mut self, start_category: &str, end_category: &str) -> Option<Vec<u8>> {
        self.start = start_category.to_string();
        self.destination = end_category.to_string();
        if start_category == end_category {
            panic!("ERROR: Source and destination categories cannot be the same!");
        }
        let mut counter = self.maps.len()*2;
        // TODO: remove this for debugging only
        //println!("counter: {:?}", counter);
        let mut path_complete: bool = false;
        let mut path_ids: Vec<u8> = vec![];
        let mut buffer_source = start_category;
        let mut buffer_dest = end_category;
        while counter != 0 && !path_complete {
            for m in self.maps.iter() {
                if m.start_category == buffer_source {
                    buffer_dest = &m.destination_category;
                    path_ids.push(m.id);
                }
            }
            if buffer_dest == end_category {
                path_complete = true;
            } else {
                buffer_source = buffer_dest;
            }
            counter = counter - 1;
        }
        if path_complete {
            Some(path_ids)
        } else {
            None
        }
    }

    // return the map instance based on the map id
    fn lookup_mapid(&self, map_id: u8) -> Option<&MapInstance> {
        for m in self.maps.iter() {
            if map_id == m.id {
                return Some(m);
            }
        }
        return None;
    }

    // get the seed category values
    fn seed_categories(&self) -> HashMap<u32, HashMap<String, u32>> {
        let mut sd: HashMap<u32, HashMap<String, u32>> = HashMap::new();
        for seed in self.seeds.iter() {
            let mut source_value= seed.id;
            let mut cat_map: HashMap<String, u32> = HashMap::new();
            for map_id in self.route.iter() {
                let mi = match self.lookup_mapid(*map_id) {
                    Some(m) => m,
                    None => continue
                };
                let (category, val) = mi.convert_value(source_value);
                let v = cat_map.entry(category).or_insert(val);
                *v = val;
                source_value = val;
            }
            let m = sd.entry(seed.id).or_insert(cat_map.clone());
            *m = cat_map.clone();
            cat_map.clear();
        }
        sd
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
    let mut almanac = match Almanac::read_file(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };

    // TODO: Remove this, for debugging only
    //dbg!(&almanac);

    // Figure out how to get from source to desitination based on input almanac (file)
    let src = "seed".to_string();
    let des = "location".to_string();
    let map_path = match almanac.make_path(&src, &des) {
        Some(mp) => mp,
        None => panic!("ERROR: No path found from {:?} to {:?}!\nCheck the input almanac {:?}",
            &src, &des, &input_file)
    };

    // TODO: Remove this, for debugging only
    //dbg!(&map_path);

    almanac.route = map_path;

    // Now get the seed category values
    let seed_categories = almanac.seed_categories();

    // TODO: Remove this, for debugging only
    //dbg!(&seed_categories);

    // Get the lowest location number from all the seeds
    let mut locations: Vec<u32> = vec![];
    for (_, t) in seed_categories.iter() {
        for (k, v) in t.iter() {
            if *k == "location".to_string() {
                locations.push(*v);
            }
        }
    }
    locations.sort();
    let lowest = locations.first().unwrap_or(&0u32);
    println!("Lowest location: {:?}", lowest);

}
