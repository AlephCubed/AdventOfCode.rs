use std::fs::File;
use std::io::{BufReader, prelude::*};
use super::super::utils;

pub fn solve() {
    let file = File::open("src/year23/day2.txt").expect("valid file");
    let reader = BufReader::new(file);

    let mut total = 0;

    for l in reader.lines() {
        let line = l.expect("valid line");
        let game = load_game(&line);

        let mut max = Round{ red: 0, green: 0, blue: 0 };

        for round in game.rounds {
            if round.red > max.red { max.red = round.red }
            if round.green > max.green { max.green = round.green }
            if round.blue > max.blue { max.blue = round.blue }
        }

        total += max.red * max.green * max.blue;
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

fn load_game(line: &str) -> Game {
    let game_sub = line
        .split(": ")
        .collect::<Vec<&str>>();
    let game_num = utils::get_num(game_sub[0]);

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
            "red" => round.red = utils::get_num(value),
            "green" => round.green = utils::get_num(value),
            "blue" => round.blue = utils::get_num(value),
            _ => panic!("{color} is not a valid color"),
        }
    }

    round
}