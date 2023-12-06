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
                if is_game_possible(line.clone()) {
                    let id: i32 = line
                        .split(':')
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap();
                    res += id;
                }
            }
            Err(error) => panic!("Error reading the line: {:?}", error),
        }
    }

    println!("Sum of IDs of possible games: {}", res);
}

fn is_game_possible(game: String) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let subsets = game.split(':').skip(1).next().unwrap().split(';');
    for subset in subsets {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for token in subset.split(',') {
            let parts: Vec<&str> = token.trim().split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            let count: i32 = parts[0].parse().unwrap_or(0);
            match parts[1] {
                "red" => {
                    red = count;
                }
                "green" => {
                    green = count;
                }
                "blue" => {
                    blue = count;
                }
                _ => {}
            }
        }

        if red > max_red || green > max_green || blue > max_blue {
            return false;
        }
    }

    true
}
