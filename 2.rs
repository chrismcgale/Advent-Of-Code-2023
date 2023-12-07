use std::cmp::max;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn valid(game: &str) -> i32 {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;
    let index = game.find(':').unwrap();

    let binding: _ = game[index + 2..].replace(";", ",");
    let amounts = binding.split(", ");

    for c in amounts {
        let mut score: i32 = c.chars().next().unwrap().to_digit(10).unwrap() as i32;

        match c.chars().nth(1).unwrap().to_digit(10) {
            Some(d) => score = score * 10 + d as i32,
            _ => {}
        }

        println!("{}", c);

        if c.contains("blue") {
            blue = max(score, blue);
        } else if c.contains("red") {
            red = max(score, red);
        } else if c.contains("green") {
            green = max(score, green);
        }
    }

    return blue * red * green;
}

fn main() -> std::io::Result<()> {
    let file = File::open("2.input")?;
    let reader = BufReader::new(file);
    let array: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut sum = 0;
    let mut i = 0;
    for game in array {
        sum += valid(&game);
    }

    println!("{}", sum);

    Ok(())
}
