

/* *************************************************************************
                           LIBRARIES AND DECLARATIONS           
   ************************************************************************* */
use std::collections::BTreeMap;
use std::env;
use std::fs:: File;
use std::io::{BufRead, BufReader};



/* *************************************************************************
                              HELPER FUNCTIONS       
   ************************************************************************* */

fn get_digits(line: &str, is_spelled: bool) -> Option<Vec<usize>> {

    // The BTreeMap will contain the first and last occurrences of the digits only
    // since we only use Rust find and rfind functions
    let mut digits: BTreeMap<usize, usize> = BTreeMap::new();
    let mut return_digits: Vec<usize> = Vec::new();

    // Look for first occurrence of numeric digits and put in BTreeMap 
    for (index, pattern) in vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        .into_iter().enumerate() {
        let x = match line.find(pattern) {
            Some(y) => y,
            None => continue
        };
        digits.insert(x, index);
    }

    // Look for first occurrence of string digits and put in BTreeMap if we consider spelled out numbers
    if is_spelled {
        for (index, pattern) in vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .into_iter().enumerate() {
            let x = match line.to_lowercase().find(pattern) {
                Some(y) => y,
                None => continue
            };
            digits.insert(x, index);
        }
    }

    // Now get the very first occurence of a digit
    // BTreeMap auto-sorts by key hence just get the first entry
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
    for (index, pattern) in vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        .into_iter().enumerate() {
        let x = match line.rfind(pattern) {
            Some(y) => y,
            None => continue
        };
        digits.insert(x, index);
    }

    // Look for last occurrence of string digits and put in BTreeMap if we consider spelled out numbers
    if is_spelled {
        for (index, pattern) in vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .into_iter().enumerate() {
            let x = match line.to_lowercase().rfind(pattern) {
                Some(y) => y,
                None => continue
            };
            digits.insert(x, index);
        }
    }

    // Get the last digit occurence.  Again BTreeMap auto-sorts this collection hence just get the last element.  
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
#[derive(Clone, Copy)]
struct CalibrationCode {
    //first: usize,
    //last: usize,
    value: u8
}

impl CalibrationCode {
    fn new(f: usize, l: usize) -> CalibrationCode {
        let mut v: String = String::new();
        v.push_str(&f.to_string());
        v.push_str(&l.to_string());
        let c = v.to_string().parse::<u8>().unwrap();
        return CalibrationCode {
            //first: f,
            //last: l,
            value: c   
        };
    }
}

/* -------------------------------------------------------------------------
    CalibrationRow struct and functions
    
    A row can have or not have any digits as numeric or spelled-out string form.
    Hence, the code attribute is an Option enum of a CalibrationCode struct.
  -------------------------------------------------------------------------- */
struct CalibrationRow {
    //line: String,
    code: Option<CalibrationCode>
}

impl CalibrationRow {
    // Create a new calibration row from a given line
    fn from(input_line: &str, is_spelled: bool) -> CalibrationRow {
        let digits = match get_digits(input_line, is_spelled) {
            Some(d) => d,
            None => {
                return CalibrationRow {
                    //line: input_line.to_owned(),
                    code: None
                }
            }
        };
        let c = CalibrationCode::new(
            *digits.get(0).unwrap(),
            *digits.get(1).unwrap()
        );
        CalibrationRow{
            //line: input_line.to_owned(),
            code: Some(c)
        }
    }
}

/* -------------------------------------------------------------------------
    CalibrationDocument struct and fucntions

    A Calibration document is just a sequence of Calibration rows 
  -------------------------------------------------------------------------- */
struct CalibrationDocument {
    lines: Vec<CalibrationRow>
}

impl CalibrationDocument {
    // Read-in the input file and parse each line
    fn new(input_file: &str, is_spelled: bool) -> Result<CalibrationDocument, std::io::Error> {
        let f = match File::open(input_file) {
            Ok(k) => k,
            Err(e) => return Err(e)
        };
        let mut c = CalibrationDocument { lines: vec![] };
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

    fn document_total(&self) -> u32 {
        let mut total = 0u32;

        //todo!(); 
        for row in self.lines.iter() {

            let code = match row.code {
                Some(c) => c,
                None => continue
            };

            // Add value to the total to fullfill the requirement
            let v = code.value;
            total += u32::from(v);

            // For debugging purposes
            //let f = code.first;
            //let l = code.last;
            //let s = &row.line.to_owned();
            //println!("{:?} -> {:?}, {:?}, {:?}", s, f, l, v);
        }
        total
    }

}

/* *************************************************************************
                                 MAIN PROGRAM         
   ************************************************************************* */
fn main() {
    // Set to consider spelled out digits or not
    let is_spelled = true;

    // Get the input file name
    let input_file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("ERROR: Program requires an argument: <input_file>");
            std::process::exit(1);
        }
    };

    // Read and parse the input document containing calibration values
    let input_document: CalibrationDocument = match CalibrationDocument::new(&input_file, is_spelled) {
        Ok(c) => c,
        Err(e) => panic!("ERROR: Failed to read input file {:?}!\n{:?}", input_file, e)
    };

    // Print the required sum of all calibaration values
    println!("\n>>> The calibration code is {:?}", input_document.document_total());
}
