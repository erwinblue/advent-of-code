
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
struct CalibrationRow {
    line: String,
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

#[derive(Clone)]
struct Digit {
    digit: String,
    index: usize
}


/*
    If the string does not have any digits return None
 */
fn get_int_digit(s: &str, last: bool) -> Option<Digit> {
    let mut v = Digit { digit: "0".to_string(), index: 0 };
    let arr: Vec<char> = match last {
        true => s.chars().into_iter().rev().collect(),
        false => s.chars().into_iter().collect()
    };
    for (i, m) in arr.iter().enumerate() {
        if m.is_digit(10) {
            v.digit = m.to_string();
            v.index  = i;
            break;
        }
    }
    println!("get_digit: {:?}, {:?}, {:?} {:?}", s, v.digit, v.index, last);
    Some(v)
}

fn get_str_digit(s: &str, last: bool) -> Option<Digit> {
    let mut v: Option<Digit> = None;
    let mut found_digit = false;
    let digits_vec: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits_arr: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for (i, p) in digits_vec.iter().enumerate() {
        if ! last {
            let num = match s.to_ascii_lowercase().find(*p) {
                Some(k) => k,
                None => continue
            };
            println!("Found first: {:?} {:?}", *p, num);
            if v.is_none() {
                v = Some(Digit {
                    digit: digits_arr[i].to_owned(),
                    index: num
                });
            } else {
                if v.unwrap().index > num {
                    let g = Digit {  
                        digit: digits_arr[i].to_owned(),
                        index: num
                    };
                    v = Some(g.clone());
                } else {
                    continue;
                }
            }
            found_digit = true;
        } else {
            let num = match s.to_ascii_lowercase().rfind(*p) {
                Some(k) =>  k,
                None => continue
            };
            println!("Found last: {:?} {:?}", *p, num);
            if v.is_none() {
                v = Some(Digit {
                    digit: digits_arr[i].to_owned(),
                    index: num
                });
            } else {
                if num > v.unwrap().index {
                    let g = Digit {  
                        digit: digits_arr[i].to_owned(),
                        index: num
                    };
                    v = Some(g.clone());
                } else {
                    continue;
                }
            }
            found_digit = true;
        }
    }
    if found_digit {
        println!("get_string_digit: {:?}, {:?}, {:?} {:?}", s, v.unwrap().digit, v.unwrap().index, last);
        return v;
    } else {
        println!("get_string_digit: {:?}, n/a, n/a n/a", s);
        return None;
    }
}

fn get_real_digit(s: &str, last: bool) -> Option<Digit> {
    let digit_int = get_int_digit(s, last);
    let digit_str = get_str_digit(s, last);
    let mut d = Digit { digit: '0', index: 0 };
    if digit_int.is_some() && digit_str.is_some() {
        if last {
            if digit_int.as_ref().unwrap().index > digit_str.as_ref().unwrap().index {
                d = digit_int.clone().unwrap();
            } else {
                d = digit_str.clone().unwrap();
            }
        } else {
            if digit_int.as_ref().unwrap().index < digit_str.as_ref().unwrap().index {
                d = digit_int.clone().unwrap();
            } else {
                d = digit_str.clone().unwrap();
            }
        }
    } else if digit_int.is_some() {
        d = digit_int.clone().unwrap();
    } else if digit_str.is_some() {
        d = digit_str.clone().unwrap();
    } else {
        return None;
    }
    println!("get_real_digit: {:?}, {:?}, {:?} {:?}", s, d.digit, d.index, last);
    Some(d)
}

fn parse_input_line(s: &str) -> CalibrationRow {

    let mut c = CalibrationRow { line: s.to_owned(), first: '0', last: '0'};

    /* -------------------------------------------------------------------
        Advent of Code 2023 Day 1 Part 1:
          Calibration code is the first occurence of a number from beginning of the line
          and the first occurence of a number from the end of the line.

        Assumption in get_digit function for the first digit occurence is
        that, if no digit is found, then last occurence is none as well
        and it won't reach that part of the code.

    let mut v = match get_int_digit(s, false) {
        Some(x) => x,
        None => return c
    };
    c.first = v.digit;

    v = match get_int_digit(s, true) {
        Some(x) => x,
        None => return c
    };
    c.last = v.digit;
    ------------------------------------------------------------------- */


    /* -------------------------------------------------------------------
        Advent of Code 2023 Day 1 Part 2:
          Some digits are spelled out with letters: one, two, three, four,.., nine.
          Recallibrate the parsing of the document

        Same assumption, in get_real_digit function for the first digit occurence is
        that, if no string digit is found, then last occurence is none as well
        and it won't reach that part of the code.
    ------------------------------------------------------------------- */

    println!("Parse first occurencce---");
    let mut v = match get_real_digit(s, false) {
        Some(x) => x,
        None => return c
    };
    c.first = v.digit;

    println!("Parse last occurencce---");
    v = match get_real_digit(s, true) {
        Some(x) => x,
        None => return c
    };
    c.last = v.digit;

    // print for debugging purposes only
    println!(">>>> Parsed line: {:?} => {:?} {:?}", s, c.first, c.last);

    // Return the wanted value
    c

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
        /*match parse_input_line(&l) {
            Some(r) => c.lines.push(r),
            None => continue
        };*/
        c.lines.push(parse_input_line(&l));
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