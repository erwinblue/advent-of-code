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

/*
 * Part 1 Main Function
 */
fn solve_part1(file_input: &String, lines: &Vec<String> ) {
    println!("\n************* PART 1 ***************");
    let puzzle_input = match Puzzle::read_data(&lines) {
        Some(p) => p,
        None => panic!("ERROR: Cannot read data in file {:?}", file_input)
    };

    // TODO: For debugging only, remove
    println!("Puzzle input:");
    dbg!(&puzzle_input);

    // Solve the Puzzle challenge based on input data
    let (_ways_to_win, ways_multiplied) = solve_ways_to_win(&puzzle_input);

    // TODO: For debugging only, remove
    //dbg!(&ways_to_win);

    // Print the required answer for puzzle
    println!("Answer for Puzzle {}: {:?}", &puzzle_input.name, ways_multiplied);
}

/*
 * Part 2 Main Function
 */
fn solve_part2(file_input: &String, lines: &Vec<String>) {

    println!("\n************* PART 2 ***************");
    let puzzle_input2 = match Puzzle::read_data2(&lines) {
        Some(p) => p,
        None => panic!("ERROR: Cannot read input file {:?}", &file_input)
    };
    // TODO: For debugging only, remove
    println!("Puzzle input:");
    dbg!(&puzzle_input2);

    // Solve the Puzzle challenge based on input data
    let (_ways_to_win, ways_multiplied) = solve_ways_to_win(&puzzle_input2);

    // TODO: For debugging only, remove
    //dbg!(&ways_to_win);

    // Print the required answer for puzzle
    println!("Answer for Puzzle {}: {:?}", &puzzle_input2.name, ways_multiplied);
}

/*
 * Solve the number of ways to win per race
 */
fn get_waystowin(race: &Race) -> u64 {
    let distance_to_beat = race.distance;
    let race_duration = race.duration;
    let mut ways = 0u64;
    if race_duration > 0 {
        for button_press in 1..race_duration {
            let press_travel = (race_duration - button_press) * button_press;
            if press_travel > distance_to_beat {
                ways += 1;
            }
        }
    }
    ways
}

/*
 * Solve the number of ways to win for the whole puzzle
 */
fn solve_ways_to_win(puzzle: &Puzzle) -> (BTreeMap<u16, u64>, u64) {

    // Find the ways to win per race
    let mut ways_to_win: BTreeMap<u16, u64> = BTreeMap::new();
    for race in puzzle.races.iter() {
        let ways = get_waystowin(race);
        let entry = ways_to_win.entry(race.id).or_insert(0u64);
        *entry = ways;
    }

    // Count the number of ways to win per race and get their product
    let mut ways_multiplied = 1u64;
    for k in ways_to_win.keys().into_iter() {
        let number_of_ways = match ways_to_win.get_key_value(k) {
            Some((_w, x)) => *x,
            None => continue
        };
        // TODO: For debugging only, remove
        println!("Race# {:?}: Number of ways to win: {:?}", k, number_of_ways);
        ways_multiplied *= number_of_ways as u64;
    }
    
    (ways_to_win, ways_multiplied)
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
   Race - the mini-boat race
       id -> The race number
       duration -> Lenght of race in milliseconds
       record -> Longest recorded distance in millimeters
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Race {
    id: u16,
    duration: u64,
    distance: u64
}

impl Race {
    fn new_race(number: usize, t: &str, d: &str) -> Option<Race> {
        let dur = match t.to_string().parse::<u64>() {
            Ok(d) => d,
            Err(_) => return None
        };
        let length = match d.to_string().parse::<u64>() {
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
   Puzzle - The description and details of the Advent of Code Puzzle
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Puzzle {
    name: String,
    races: Vec<Race>,
}

impl Puzzle {
    // Part 1: Assume the format of the input file is something like,
    //    attribute  race#1  race#2   race#3
    //    Time:      7       15       30
    //    Distance:  9       40       200
    fn read_data(lines: &Vec<String>) -> Option<Puzzle> {
        let mut puzzle = Puzzle {
            name: "Advent of Code 2023 Day 6 Part 1".to_string(),
            races: vec![]
        };
        let mut durations: Vec<String> = vec![];
        let mut distances: Vec<String> = vec![];
        let mut attribute: String = String::new();
        for stat_line in lines.iter() {
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

    // Part 2: Ignore the spaces between the numbers, hence there is only 1 big race
    //    attribute  race#1  race#2   race#3  --> attribute: race#1
    //    Time:      7       15       30  ------> Time:      71530
    //    Distance:  9       40       200 ------> Distance:  940200
    fn read_data2(lines: &Vec<String>) -> Option<Puzzle> {
        let mut puzzle = Puzzle {
            name: "Advent of Code 2023 Day 6 Part 2".to_string(),
            races: vec![]
        };
        let mut durations: Vec<String> = vec![];
        let mut distances: Vec<String> = vec![];
        for stat_line in lines.iter() {
            let parts: Vec<&str> = stat_line.split(':').into_iter().collect();
            let val = match parts.get(1) {
                Some(v) => v.to_string().replace(" ", ""),
                None => return None
            };
            if stat_line.contains("time:") {
                durations.push(val);
            } else if stat_line.contains("distance:") {
                distances.push(val);
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

    solve_part1(&input_file, &lines);

    solve_part2(&input_file, &lines);

}
