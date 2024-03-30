use std::fs::File;
use std::io::{BufReader, prelude::*};
use regex::Regex;

pub fn solve() {
    let file = File::open("src/year23/day2.txt").expect("valid file");
    let reader = BufReader::new(file);

    let mut total = 0;

    'outer: for l in reader.lines() {
        let line = l.expect("valid line");
        let game = load_game(&line);

        for round in game.rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                println!("Game {} is not valid", game.number);
                println!("\tR:{} G:{} B{}", round.red, round.green, round.blue);
                continue 'outer;
            }
        }

        total += game.number;
    }

    println!("{total}");
}

struct Game {
    number: i32,
    rounds: Vec<Round>,
}

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn get_num(input: &str) -> i32 {
    let regex = Regex::new(r"\D").unwrap();
    regex.replace_all(input, "").parse::<i32>().unwrap()
}

fn load_game(line: &str) -> Game {
    let game_sub = line
        .split(": ")
        .collect::<Vec<&str>>();
    let game_num = get_num(game_sub[0]);

    let round_sub = game_sub[1]
        .split("; ")
        .collect::<Vec<&str>>();
    let mut rounds: Vec<Round> = vec![];

    for string in round_sub {
        rounds.push(load_round(string));
    }

    Game{ number: game_num, rounds }
}

fn load_round(string: &str) -> Round {
    let mut round = Round{ red: 0, green: 0, blue: 0 };

    let color_sub = string
        .split(", ")
        .collect::<Vec<&str>>();

    for value in color_sub {
        let color = value.split(" ").collect::<Vec<&str>>()[1];

        match color {
            "red" => round.red = get_num(value),
            "green" => round.green = get_num(value),
            "blue" => round.blue = get_num(value),
            _ => panic!("{color} is not a valid color"),
        }
    }

    round
}