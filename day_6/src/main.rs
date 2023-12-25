use std::fs;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "input.pest"]
struct RaceParser {
    times: Vec<u64>,
    distances: Vec<u64>,
}

fn simulate_race(time: u64, dist: u64) -> u64 {
    let mut winning_options = 0;

    for hold_time in 0..=time {
        let speed = hold_time; // in mm / ms
        let traveled_dist = speed * (time - hold_time);
        if traveled_dist > dist {
            winning_options += 1;
        }
    }

    winning_options
}

fn parse_input(input: String) -> RaceParser {
    let parse = RaceParser::parse(Rule::grammar, &input).expect("Bad parse");

    let mut data = RaceParser {
        times: Vec::new(),
        distances: Vec::new(),
    };

    let mut time: String = "".to_string();
    let mut dist: String = "".to_string();

    for item in parse {
        match item.as_rule() {
            Rule::time => time += item.as_str(),
            Rule::distance => dist += item.as_str(),
            _ => println!("Bad parsing"),
        }
    }

    data.times.push(time.parse::<u64>().unwrap());
    data.distances.push(dist.parse::<u64>().unwrap());

    data
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let data = parse_input(input_string);

    let mut error_margin = 1;
    for idx in 0..data.times.len() {
        error_margin *= simulate_race(data.times[idx], data.distances[idx])
    }

    println!("Error margin: {error_margin}");
}
