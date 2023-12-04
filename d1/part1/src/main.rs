use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let file = match File::open("src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);
    let mut res = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                res += extract_calibration_value(&line);
                println!("{}", res);
            }
            Err(error) => panic!("Problem reading the line: {:?}", error),
        }
    }
}

fn extract_calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line
        .chars()
        .rev()
        .find(|c| c.is_digit(10));

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            let value = format!("{}{}", f, l);
            value.parse::<u32>().unwrap_or(0)
        }
        _ => 0,
    }
}
