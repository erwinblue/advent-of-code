/* *************************************************************************
                        LIBRARIES AND DECLARATIONS
   ************************************************************************* */
use std::collections::BTreeMap;
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
   Race - the mini-boat race
       id -> The race number
       duration -> Lenght of race in milliseconds
       record -> Longest recorded distance in millimeters
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Race {
    id: u16,
    duration: u32,
    distance: u32
}

impl Race {
    fn new_race(number: usize, t: &str, d: &str) -> Option<Race> {
        let dur = match t.to_string().parse::<u32>() {
            Ok(d) => d,
            Err(_) => return None
        };
        let length = match d.to_string().parse::<u32>() {
            Ok(l) => l,
            Err(_) => return None
        };
        Some(Race {
            id: number as u16,
            duration: dur,
            distance: length
        })
    }
}

/* -------------------------------------------------------------------------
   Waytowin - Potential winner entry
       race -> race id number (i.e. Race id attribute value)
       pressed -> how long button is pressed in milliseconds
       travel -> how far boat will travel in millemeters after releasing the button
   ------------------------------------------------------------------------- */
#[derive(Clone, Copy, Debug)]
struct Waytowin {
    race: u16,
    pressed: u32,
    travel: u32
}

impl Waytowin {
    fn ways_to_win(race: &Race) -> Vec<Waytowin> {
        let distance_to_beat = race.distance;
        let race_duration = race.duration;
        let mut ways: Vec<Waytowin> = vec![];
        if race_duration > 0 {
            for button_press in 1..race_duration {
                let press_travel = (race_duration - button_press) * button_press;
                if press_travel > distance_to_beat {
                    ways.push(Waytowin{
                        race: race.id,
                        pressed: button_press,
                        travel: press_travel
                    })
                }
            }
        }
        ways
    }
}

/* -------------------------------------------------------------------------
   Puzzle - The description and details of the Advent of Code Puzzle
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Puzzle {
    name: String,
    id: f32,
    races: Vec<Race>,
}

impl Puzzle {
    fn read_file(file: File) -> Option<Puzzle> {
        let mut puzzle = Puzzle {
            name: "Advent of Code 2023".to_string(),
            id: 6.1,
            races: vec![]
        };
        let mut durations: Vec<String> = vec![];
        let mut distances: Vec<String> = vec![];
        let mut attribute: String = String::new();
        // Assume the format of the input file is something like,
        //    attribute  race#1  race#2   race#3
        //    Time:      7       15       30
        //    Distance:  9       40       200
        for line in BufReader::new(file).lines() {
            let stat_line = match line {
                Ok(l) => l.to_lowercase(),
                Err(_) => return None
            };
            for (i, part) in stat_line.split_whitespace().map(|x|x.to_owned()).enumerate() {
                if i == 0 {
                    attribute = part;
                    continue;
                } else if !part.contains("time:") || !part.contains("distance:") {
                    if attribute.contains("time") {
                        durations.push(part);
                    } else if attribute.contains("distance") {
                        distances.push(part);
                    }
                }
            }
        }
        let mut race_instances: Vec<Race> = vec![];
        if durations.len() == distances.len() {
            for (i, d) in durations.iter().enumerate() {
                let r = match Race::new_race(i+1, d, distances.get(i).unwrap_or(&"".to_string())) {
                    Some(s) => s,
                    None => continue
                };
                race_instances.push(r);
            }
            puzzle.races = race_instances;
        } else {
            return None;
        }
        Some(puzzle)
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
    let puzzle_input = match Puzzle::read_file(f) {
        Some(p) => p,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };

    // TODO: For debugging only, remove
    //dbg!(&puzzle_input);

    // Find the ways to win per race
    let mut ways_to_win: BTreeMap<u16, Vec<Waytowin>> = BTreeMap::new();
    for race in puzzle_input.races.iter() {
        let ways = Waytowin::ways_to_win(race);
        for w in ways.iter() {
            let entries = ways_to_win.entry(w.race).or_insert(vec![]);
            //*entry = *w;
            entries.push(w.clone());
        }
    }

    // TODO: For debugging only, remove
    //dbg!(&ways_to_win);

    // Count the number of ways to win per race and get their product
    let mut ways_multiplied = 1u32;
    for k in ways_to_win.keys().into_iter() {
        let number_of_ways = match ways_to_win.get(k) {
            Some(w) => w.len(),
            None => continue
        };
        // TODO: For debugging only, remove
        println!("Race# {:?}: Number of ways to win: {:?}", k, number_of_ways);
        ways_multiplied *= number_of_ways as u32;
    }
    
    // Print the required answer for puzzle
    println!("Answer for Puzzle {} {}: {:?}", puzzle_input.name, puzzle_input.id, ways_multiplied);

}
