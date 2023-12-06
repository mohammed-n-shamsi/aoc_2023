use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[grammar = "input.pest"]
struct CardParser {
    card_id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn parse_line(input: String) -> CardParser {
    let parse = CardParser::parse(Rule::complete, &input).expect("Bad parse");

    let mut card = CardParser {
        card_id: 0,
        winning_numbers: Vec::new(),
        card_numbers: Vec::new(),
    };

    for items in parse {
        for token in items.into_inner() {
            match token.as_rule() {
                Rule::id => card.card_id = token.as_str().parse::<u32>().unwrap(),
                Rule::pre_number => card
                    .winning_numbers
                    .push(token.as_str().parse::<u32>().unwrap()),
                Rule::post_number => card
                    .card_numbers
                    .push(token.as_str().parse::<u32>().unwrap()),
                _ => {}
            };
        }
    }

    card
}

fn calculate_matches(card: &CardParser) -> u32 {
    let mut matches: u32 = 0;

    for card_num in &card.card_numbers {
        if card.winning_numbers.contains(card_num) {
            matches += 1
        }
    }
    matches
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut cards: Vec<(CardParser, u32)> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                cards.push((parse_line(line), 1));
            }
        }

        for idx in 0..cards.len() {
            for _ in 0..cards[idx].1 {
                let matches = calculate_matches(&cards[idx].0);
                for sum_idx in idx + 1..=idx + matches as usize {
                    if (0..cards.len()).contains(&sum_idx) {
                        cards[sum_idx].1 += 1;
                    }
                }
            }
        }

        let mut total_count: u32 = 0;

        for (_card, count) in cards {
            total_count += count
        }

        println!("Total count is: {total_count}")
    }
}
