use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let file = match File::open("src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);
    let mut res = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                res += process_line(line.as_str());
            }
            Err(error) => panic!("Error reading the line: {:?}", error),
        }
    }

    println!("Total: {}", res);
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    (
        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }
    )
        .parse::<u32>()
        .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_values() {
        let lines = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string()
        ];

        let mut total = 0;
        for line in lines {
            total += process_line(&line);
        }
        assert_eq!(total, 281);
    }
}
