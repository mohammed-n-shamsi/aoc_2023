use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BASE: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
struct PartNumber {
    number: i32,
    line: usize,
    start_col: usize,
    end_col: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Symbol {
    line: usize,
    col: usize,
}

#[derive(Clone, Copy)]
struct GearPart {
    star_symbol: Symbol,
    gear_piece: PartNumber,
}

#[derive(Clone, Copy)]
struct Gear {
    gear_one: GearPart,
    gear_two: GearPart,
    gear_ratio: i32,
}

fn parse_part_numbers(input: &String, line: usize) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    let mut number_vec = Vec::new();
    for (idx, curr_char) in input.chars().enumerate() {
        if curr_char.is_digit(BASE) {
            number_vec.push(curr_char);
        } else {
            if !number_vec.is_empty() {
                let num_string: String = number_vec.iter().collect();

                part_numbers.push(PartNumber {
                    number: num_string.parse::<i32>().unwrap(),
                    line,
                    start_col: idx - number_vec.len(),
                    end_col: idx - 1,
                });

                number_vec.clear();
            }
        }
    }

    if !number_vec.is_empty() {
        let num_string: String = number_vec.iter().collect();

        let idx = input.len();

        part_numbers.push(PartNumber {
            number: num_string.parse::<i32>().unwrap(),
            line,
            start_col: idx - number_vec.len(),
            end_col: idx - 1,
        });

        number_vec.clear();
    }
    part_numbers
}

fn parse_symbols(input: &String, line: usize) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (idx, curr_char) in input.chars().enumerate() {
        if curr_char == '*' {
            symbols.push(Symbol { line, col: idx });
        }
    }

    symbols
}

fn filter_part_numbers(complete_part_numbers: Vec<PartNumber>, symbols: &Vec<Symbol>) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    let mut gear_parts = Vec::new();

    for part_number in complete_part_numbers {
        let line_range;
        if part_number.line == 0 {
            line_range = 0..=part_number.line + 1;
        } else {
            line_range = part_number.line - 1..=part_number.line + 1;
        }
        let col_range;
        if part_number.start_col == 0 {
            col_range = 0..=part_number.end_col + 1;
        } else {
            col_range = part_number.start_col - 1..=part_number.end_col + 1;
        }
        for symbol in symbols {
            if line_range.contains(&symbol.line) && col_range.contains(&symbol.col) {
                gear_parts.push(GearPart {
                    star_symbol: *symbol,
                    gear_piece: part_number,
                });
                break;
            }
        }
    }

    let mut potential_gears: HashMap<Symbol, Vec<GearPart>> = HashMap::new();

    for symbol in symbols {
        potential_gears.insert(*symbol, Vec::new());
        for gear_part in &gear_parts {
            if *symbol == gear_part.star_symbol {
                // potential_gears[symbol].push(*gear_part)
                let gear_vec = potential_gears.get_mut(symbol).unwrap();
                gear_vec.push(*gear_part)
            }
        }
    }

    for (_symbol, gear_parts) in potential_gears {
        if gear_parts.len() == 2 {
            let gear_one = gear_parts[0];
            let gear_two = gear_parts[1];
            gears.push(Gear {
                gear_one,
                gear_two,
                gear_ratio: gear_one.gear_piece.number * gear_two.gear_piece.number,
            });
        }
    }

    gears
}

fn sum_part_numbers(gears: &Vec<Gear>) -> i32 {
    let mut sum = 0;

    for gear in gears {
        sum += gear.gear_ratio
    }
    sum
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
        let mut complete_part_numbers: Vec<PartNumber> = Vec::new();
        let mut complete_symbols: Vec<Symbol> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for (idx, wrapped_line) in lines.enumerate() {
            if let Ok(line) = wrapped_line {
                let mut row_part_numbers = parse_part_numbers(&line, idx);
                let mut row_symbols = parse_symbols(&line, idx);

                complete_part_numbers.append(&mut row_part_numbers);
                complete_symbols.append(&mut row_symbols);
            }
        }
        let filtered_part_numbers = filter_part_numbers(complete_part_numbers, &complete_symbols);
        println!("Sum is: {}", sum_part_numbers(&filtered_part_numbers));
    }
}
