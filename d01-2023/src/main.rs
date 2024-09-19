
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CalibrationRow {
    first: char,
    last: char
}

impl CalibrationRow {
    fn calibration_value(&self) -> u32 {
        let c: String = [self.first, self.last].into_iter().collect();
        c.to_string().parse::<u32>().unwrap_or(0)
    }
}

struct CalibrationDocument {
    lines: Vec<CalibrationRow>
}

impl CalibrationDocument {
    fn document_total(&self) -> u32 {
        let mut total: u32 = 0;
        for row in self.lines.iter() {
            total += row.calibration_value();
        }
        total
    }
}

fn parse_input_line(s: &str) -> Option<CalibrationRow> {
    // Calibration code is the first occurence of a number from beginning of the line
    // and the first occurence of a number from the end of the line.
    let mut c = CalibrationRow { first: '0', last: '0'};
    // get the first occurence of a digit from beginning of line
    for m in s.chars() {
        if m.is_digit(10) {
            c.first = m;
            break;
        }
    }
    // get the first occurence of a digit from end of line going reverse
    for m in s.chars().rev() {
        if m.is_digit(10) {
            c.last = m;
            break;
        }
    }
    //println!("{:?} => {:?} {:?}", s, c.first, c.last);
    Some(c)
}

fn parse_input_document(s: &str) -> Result<CalibrationDocument, std::io::Error> {
    let f = match File::open(s) {
        Ok(k) => k,
        Err(e) => return Err(e)
    };
    let mut c = CalibrationDocument { lines: vec![] };
    for l in BufReader::new(f).lines()
        .map(|x| match x {
            Ok(y) => y,
            Err(_) => String::from("")
        }) {
        match parse_input_line(&l) {
            Some(r) => c.lines.push(r),
            None => continue
        };
    }
    Ok(c)
}

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
    let input_document: CalibrationDocument = match parse_input_document(&input_file) {
        Ok(c) => c,
        Err(e) => panic!("ERROR: Failed to read input file {:?}!\n{:?}", input_file, e)
    };

    // Print the required sum of all calibaration values
    println!("The calibration code is {:?}", input_document.document_total());
}
