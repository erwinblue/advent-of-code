
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


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
   Subset struct and functions
   ------------------------------------------------------------------------- */
struct Subset {
    red: u32,
    green: u32,
    blue: u32
}

/* -------------------------------------------------------------------------
   Game struct and functions
   ------------------------------------------------------------------------- */
struct Game {
    id: u32,
    subsets: Vec<Subset>
}

impl Game {
    fn from(input_line: &str) -> Option<Game> {
        let arr: Vec<&str> = input_line.split(':').collect();
        if arr.len() > 2 {
            return None;
        }
        let id = match get_game_id(arr[0]) {
            Ok(i) => i,
            Err(e) => return None
        };
        let sets: Vec<Subset> = Vec::new();
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

}