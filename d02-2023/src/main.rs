
/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
use std::env;
use std::fs:: File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;


/* *************************************************************************
                              HELPER FUNCTIONS       
   ************************************************************************* */

fn get_game_id(header: &str) -> Result<u16, ParseIntError> {
    let number = header.to_lowercase()
        .replace("game ", "")
        .replace(" ", "");
    number.to_owned().parse::<u16>()
}

fn parse_set_color(s: &str) -> Option<(String, u32)> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();
    let color = match parts.get(1) {
        None => return None,
        Some(c) => {
            match *c {
                "red" => "red".to_string(),
                "green" => "green".to_string(),
                "blue" => "blue".to_string(),
                _ => return None
            }
        }
    };
    /*let count = match parts.get(0) {
        Some(c) => c.to_owned().parse::<u32>().unwrap_or(0u32),
        None => 0u32
    };*/
    let count = parts.get(0)
        .unwrap_or(&"0")
        .to_owned().parse::<u32>()
        .unwrap_or(0u32);
    return Some((color, count));
}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
   Subset (a.k.a. set) struct and functions
   ------------------------------------------------------------------------- */
struct Subset {
    red: u32,
    green: u32,
    blue: u32
}

impl Subset {
    fn from(b: &str) -> Option<Subset> {
        let mut z = Subset { red: 0, green: 0, blue: 0 };
        for c in b.to_lowercase().trim().split(',').into_iter() {
            match parse_set_color(c) {
                None => continue,
                Some((x, y)) => {
                    match x.as_str() {
                        "red" => { z.red = y },
                        "green" => { z.green = y },
                        "blue" => { z.blue = y },
                        _ => continue
                    };
                }
            };
        }
        Some(z)
    }

    fn show(&self) {
        println!("\tred: {:?}, green: {:?}, blue: {:?}", self.red, self.green, self.blue);
    }

    fn possible(&self, reference: &Subset) -> bool {
        if self.red > reference.red {
            false
        } else if self.green > reference.green {
            false
        } else if self.blue > reference.blue {
            false
        } else {
            true
        }
    }

}


/* -------------------------------------------------------------------------
   Game struct and functions
   ------------------------------------------------------------------------- */
struct Game {
    id: u16,
    subsets: Vec<Subset>
}

impl Game {
    fn from(input_line: &str) -> Option<Game> {
        //Split and lazy check if the game is invalid i.e. more elements after split
        let arr: Vec<&str> = input_line.split(':').collect();
        if arr.len() > 2 {
            return None;
        }

        // Get the game number i.e. game id
        let game_id = match get_game_id(arr[0]) {
            Ok(i) => i,
            Err(_e) => return None
        };

        // Get the subset details of the game
        // Assume a game must have at least one subset
        let mut sets: Vec<Subset> = Vec::new();
        for body in arr[1].to_lowercase().split(';') {
            match Subset::from(body) {
                Some(b) => sets.push(b),
                None => continue
            };
        }
   
        // TODO: Remove this.  Used only for debugging.
        println!("Game#: {:?}", game_id);
        for zz in sets.iter() {
            zz.show();
        }

        Some(Game { id: game_id, subsets: sets })
    }

    fn possible(&self, r: &Subset) -> (u16, bool) {
        for s in self.subsets.iter() {
            if s.possible(r) {
                continue;
            } else {
                return (self.id, false);
            }
        } 
        return (self.id, true);
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

    // Read each line in file and parse to get game id and subset details
    let mut games: Vec<Game> = Vec::new();
    for (_line_number, line_string) in BufReader::new(f).lines()
        .map(|x| match x {
            Ok(y) => y,
            Err(_) => String::from("<CANNOT READ LINE>")
        }).enumerate() {
        let g = match Game::from(&line_string) {
            Some(k) => k,
            None => continue
        };
        games.push(g);
    }

    // Perform test of the possible games
    let max = Subset {
        red: 12,
        green: 13,
        blue: 14
    };
    let mut possible_games: Vec<u16> = vec![];
    let mut sum_possible_games: u32 = 0;
    for g in games.iter() {
        if g.possible(&max).1 {
            possible_games.push(g.id);
            sum_possible_games += u32::from(g.id);
        }
    }

    // Show result of test of possible games
    println!("\nPossible games: {:?}", possible_games);
    println!("Sum of id's of possible games: {:?}", sum_possible_games);

}