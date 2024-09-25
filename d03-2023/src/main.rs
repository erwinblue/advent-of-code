/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
use std::env;
use std::fs:: File;
use std::io::{BufRead, BufReader};


/* *************************************************************************
                              HELPER FUNCTIONS       
   ************************************************************************* */


/* *************************************************************************
                            ENUM AND METHODS
   ************************************************************************* */


/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
   Coordinate struct and functions
   ------------------------------------------------------------------------- */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Coordinate {
    x: u32,
    y: u32
}


/* -------------------------------------------------------------------------
   Symbol struct and functions
   - Symbols can only be a single character
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Symbol {
    symbol: char,
    coordinate: Coordinate,
}

impl Symbol {
    // Our init or new method
    fn make_symbol(s: char, c: Coordinate) -> Symbol {
        Symbol { symbol: s, coordinate: c} 
    }

    // Return the possible adjacent coordinates
    fn adjacent_coordinates(&self, max: Coordinate) -> Vec<Coordinate> {
        // initialize an empty return value
        let mut adjacents: Vec<Coordinate> = vec![];
        // minimum and maximum coordinates adjacent to the symbol
        let min_x = match self.coordinate.x {
            0 => self.coordinate.x,
            _ => self.coordinate.x - 1
        };
        let min_y = match self.coordinate.y {
            0 => self.coordinate.y,
            _ => self.coordinate.y - 1
        };
        let max_x = match (self.coordinate.x + 1) > max.x {
            true => self.coordinate.x,
            false => self.coordinate.x + 1
        };
        let max_y = match (self.coordinate.y + 1) > max.y {
            true => self.coordinate.y,
            false => self.coordinate.y + 1
        };
        for loc_y in min_y..=max_y {
            for loc_x in min_x..=max_x {
                let c = Coordinate { x: loc_x.to_owned(), y: loc_y.to_owned() };
                // a symbols coordinate is his own and a number cannot be at that point
                if c != self.coordinate {
                    adjacents.push(c); 
                }
            }
        }
        adjacents
    }
}

/* -------------------------------------------------------------------------
   Number struct and functions
   - Numbers can have any number of digits 
   ------------------------------------------------------------------------- */
#[derive(Debug)]
struct Number {
    number: u32,
    coordinates: Vec<Coordinate>
}

impl Number {
    // Our init or new method
    fn make_number(d: Vec<char>, c: Vec<Coordinate>) -> Number {
        let num: String = d.into_iter().collect();
        Number { number: num.parse::<u32>().unwrap_or(0u32), coordinates: c }
    }
}

/* -------------------------------------------------------------------------
   Document struct and functions
   ------------------------------------------------------------------------- */
struct Document {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
    maxpoint: Coordinate
}

impl Document {

    // Read a file as our input document and parse each line and each character
    // in a line.  A document can have symbols and numbers.
    fn make_document(file: File) -> Option<Document> {
        // Initialize our Document object
        let mut document = Document { 
            symbols: Vec::new(),
            numbers: Vec::new(),
            maxpoint: Coordinate {
                x: 0,
                y: 0
            }
        };
        // Loop through each line and each character in the line to get possible 
        // symbols and numbers
        let mut line_number: u32 = 0; // Our current y value of coordinate
        let mut char_number: u32 = 0; // Our current x value of coordinate
        let mut current_coordinate = Coordinate{ x: char_number, y: line_number };
        for line_string in BufReader::new(file).lines()
            .map(|x| match x {
                Ok(y) => y,
                Err(_) => String::from("<CANNOT READ LINE>")
            }) {
            // Loop through each character of the line to parse symbols and numbers
            // We assume only numbers, symbols (including periods) only exist in our document
            char_number = 0;
            let mut buffer_number_digits: Vec<char> = vec![];
            let mut buffer_number_coordinates: Vec<Coordinate> = vec![];
            let mut is_flush_buffer: bool = false;
            let line_length = line_string.trim().chars().count() - 1;
            for (index, character) in line_string.trim().chars().into_iter().enumerate() {
                // set the current coordinate
                current_coordinate = Coordinate{ x: char_number, y: line_number };
                if character.is_digit(10) {
                    // if a digit, append the character and coordinates to our buffers.
                    buffer_number_digits.push(character);
                    buffer_number_coordinates.push(current_coordinate);
                    // if we are the last character in the line,
                    // we need to flush the buffer before going to the next line
                    if index == line_length {
                        is_flush_buffer = true;
                    }
                } else if character == '.' {
                    is_flush_buffer = true;
                } else {
                    // if a symbol, create a new symbol object and add to the document
                    // we also set flag to flush buffers as previous character 
                    // may have been part of a series of digits
                    document.symbols.push(
                        Symbol::make_symbol(character, current_coordinate)
                    );
                    is_flush_buffer = true;
                }
                // flush the buffer to create a number and get its coordinates
                if is_flush_buffer {
                    // only flush the buffer if it is not empty
                    if !buffer_number_coordinates.is_empty() && !buffer_number_digits.is_empty() {
                        document.numbers.push(
                            Number::make_number(buffer_number_digits.to_owned(), buffer_number_coordinates.to_vec())
                        );
                        buffer_number_coordinates.clear();
                        buffer_number_digits.clear();
                    }
                    // reset the flag even if buffer is empty as there could be multiple periods in succession
                    is_flush_buffer = false;
                }
                // increase our x coordinate
                char_number += 1;
            }
            // increase our y coordinate
            line_number += 1;
        }
        document.maxpoint = current_coordinate;
        
        // TODO: Remvoe this only used for debugging
        //dbg!(&document.symbols);
        Some(document)
    }

    // The part numbers.  According to the challenge exercise,
    // part numbers are the numbers that are adjacent to a symbol including diagonally.
    fn part_numbers(&self) -> Vec<u32> {
        let mut parts: Vec<u32> = Vec::new();
        let mut adjacent_points: Vec<Coordinate> = Vec::new();
        for symbol in self.symbols.iter() {
            let mut symbol_ajacents = symbol.adjacent_coordinates(self.maxpoint);
            // TODO: Remvoe this only used for debugging
            //println!("Symbol: {:?} {:?}", symbol.symbol, symbol_ajacents);
            adjacent_points.append(&mut symbol_ajacents);
        }

        let mut is_adjacent = false;
        for num in self.numbers.iter() {
            for ajacent_point in adjacent_points.iter() {
                for num_point in num.coordinates.iter() {
                    if num_point.x == ajacent_point.x && num_point.y == ajacent_point.y {
                        is_adjacent = true; 
                    }
                }
            }
            if is_adjacent {
                parts.push(num.number);
                is_adjacent = false;
            }
        }
        parts
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

    let document = match Document::make_document(f) {
        Some(d) => d,
        None => {
            println!("ERROR: Cannot read input file {:?}", &input_file);
            std::process::exit(1);
        }
    };
    let part_numbers = document.part_numbers();
    // TODO: Remvoe this only used for debugging
    //dbg!(&part_numbers);

    // show the sum of all part numbers
    let mut sum_of_parts = 0u32;
    for p in part_numbers.iter() {
        sum_of_parts += *p;
    }
    println!("Sum of part numbers: {:?}", sum_of_parts);

    
}
