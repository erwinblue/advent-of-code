use core::num;
/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs:: File;
use std::io::{BufRead, BufReader};



/* *************************************************************************
                              HELPER FUNCTIONS       
   ************************************************************************* */

fn get_digits<'a>(line: &'a str, is_spelled: bool) -> Option<Vec<usize>> {
    // The BTreeMap will contain the first and last occurrences of the digits only
    // since Rust only provides find and rfind functions and we are too lazy to 
    // create a more complex pattern search of occurences.

    let mut digits: BTreeMap<usize, usize> = BTreeMap::new();
    let mut return_digits: Vec<usize> = Vec::new();

    // Look for first occurrence of numeric digits and put in BTreeMap 
    for (index, pattern) in vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].into_iter().enumerate() {
        let x = match line.find(pattern) {
            Some(y) => y,
            None => continue
        };
        digits.insert(x, index);
    }

    // Look for first occurrence of string digits and put in BTreeMap if we consider spelled out numbers
    if is_spelled {
        for (index, pattern) in vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().enumerate() {
            let x = match line.find(pattern) {
                Some(y) => y,
                None => continue
            };
            digits.insert(x, index);
        }
    }

    // Now get the very first occurence of a digit
    // BTreeMap auto-sorts by key hence just get the first entry since we have numbers for the keys
    if digits.len() > 0 {
        let (_, first_digit)= digits.pop_first().unwrap_or((0, 0));
        return_digits.push(first_digit);
    } else {
        // If we reach this code then it means there was no digit parsed from the row of string
        // Hence our last digit occurence won't matter.
        return None;
    }

    // Reset our BTreeMap to get the last digit occurence
    digits.clear();

    // Look for last occurrence of numeric digits and put in BTreeMap 
    for (index, pattern) in vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].into_iter().enumerate() {
        let x = match line.rfind(pattern) {
            Some(y) => y,
            None => continue
        };
        digits.insert(x, index);
    }

    // Look for last occurrence of string digits and put in BTreeMap if we consider spelled out numbers
    if is_spelled {
        for (index, pattern) in vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().enumerate() {
            let x = match line.rfind(pattern) {
                Some(y) => y,
                None => continue
            };
            digits.insert(x, index);
        }
    }

    // Get the last digit occurence.  Again BTreeMap auto-sorts this collection hence
    // just get the last element.  
    if digits.len() > 0 {
        // We can safely use unwrap_or()... since we tested for the HashMap lenght.
        let (_, last_digit)= digits.pop_last().unwrap_or((0, 0));
        return_digits.push(last_digit);
    }

    Some(return_digits)

}

/* *************************************************************************
                            STRUCTURE AND METHODS
   ************************************************************************* */

/* -------------------------------------------------------------------------
    CalibrationCode struct and functions
    
    A digit can be either a number spelled our or,
    a numeric number from 0 to 9. 
  -------------------------------------------------------------------------- */
struct CalibrationCode {
    first: usize,
    last: usize,
    value: u8
}

impl CalibrationCode {
    fn new(f: usize, l: usize) -> CalibrationCode {
        let mut v: String = String::new();
        v.push_str(&f.to_string());
        v.push_str(&l.to_string());
        let c = v.to_string().parse::<u8>().unwrap();
        return CalibrationCode {
            first: f,
            last: l,
            value: c   
        };
    }
}

/* -------------------------------------------------------------------------
    CalibrationRow struct and functions
    
    A row can have or not have any digits as numeric or spelled-out string form.
    Hence, the digits attribute is an Option enum of a BTreeMap collection.
    BTreeMap collection is chosen to be able to sort the digit index/position
    on an given line of document or paragraph.
  -------------------------------------------------------------------------- */
struct CalibrationRow {
    line: String,
    code: Option<CalibrationCode>
}

impl CalibrationRow {
    // Create a new calibration row from a given line
    fn from<'a>(input_line: &'a str, is_spelled: bool) -> CalibrationRow {
        let digits = match get_digits(input_line, is_spelled) {
            Some(d) => d,
            None => {
                return CalibrationRow {
                    line: input_line.to_owned(),
                    code: None
                }
            }
        };
        let c = CalibrationCode::new(
            *digits.get(0).unwrap(),
            *digits.get(1).unwrap()
        );
        CalibrationRow{
            line: input_line.to_owned(),
            code: Some(c)
        }
    }
}

/* -------------------------------------------------------------------------
    CalibrationDocument struct and fucntions
    A document is a sequence of calibration rows 
  -------------------------------------------------------------------------- */
struct CalibrationDocument {
    lines: Vec<CalibrationRow>,
    spelled_digits: bool
}

impl CalibrationDocument {
    // Read-in the input file and parse each line
    fn new(input_file: &str, is_spelled: bool) -> Result<CalibrationDocument, std::io::Error> {
        let f = match File::open(input_file) {
            Ok(k) => k,
            Err(e) => return Err(e)
        };
        let mut c = CalibrationDocument { lines: vec![], spelled_digits: is_spelled };
        for (_line_number, line_string) in BufReader::new(f).lines()
            .map(|x| match x {
                Ok(y) => y,
                Err(_) => String::from("<CANNOT READ LINE>")
            }).enumerate() {
            let row = CalibrationRow::from(&line_string, is_spelled);
            c.lines.push(row);
        }
        Ok(c)
    }

    /*       ---------------------- TOOOOOOOOOO DOOOOOOOOOOOOOOOOOOO -----------------
    /*  If spelled digits is false just consider numeric digits.  Otherwise consider both
        numeric and spelled digits in the document */
    fn spelled_digits(&mut self, flag: bool) {
        self.spelled_digits = flag; 
        // TODO: Re-read the document with spelled digits or not depending of flag value
        for line in self.lines.iter() {
            todo!();
        }
    }
             ---------------------- TOOOOOOOOOO DOOOOOOOOOOOOOOOOOOO ----------------- */

    fn document_total(&self) -> u32 {
        let total = 0u32;
        
        total
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

    // Read and parse the input document containing calibration values
    let input_document: CalibrationDocument = match CalibrationDocument::new(&input_file, false) {
        Ok(c) => c,
        Err(e) => panic!("ERROR: Failed to read input file {:?}!\n{:?}", input_file, e)
    };

    // Print the required sum of all calibaration values
    println!("The calibration code is {:?}", input_document.document_total());
}
