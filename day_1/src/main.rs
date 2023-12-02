use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BASE: u32 = 10;

const STRING_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn generate_map() -> HashMap<&'static str, i32> {
    maplit::hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9
    }
}

fn get_dig_chars(chars: &Vec<char>, front: bool) -> Option<(i32, usize)> {
    let mut digit_char = '-';
    let mut pos: usize = 0;

    for (idx, char) in chars.iter().enumerate() {
        if char.is_digit(BASE) {
            digit_char = *char;
            pos = idx;
            if front {
                break;
            }
        }
    }
    if digit_char != '-' {
        return Some((digit_char.to_digit(BASE).unwrap() as i32, pos));
    }
    None
}

fn get_string_digits(input: &String, front: bool) -> Option<(i32, usize)> {
    let digit_map = generate_map();
    let mut string_digits = BTreeMap::<usize, i32>::new();
    for digit in STRING_DIGITS {
        if front {
            if let Some(pos) = input.find(digit) {
                string_digits.insert(pos, digit_map[digit]);
            }
        } else {
            if let Some(pos) = input.rfind(digit) {
                string_digits.insert(pos, digit_map[digit]);
            }
        }
    }

    if front {
        if let Some((pos, digit)) = string_digits.first_key_value() {
            return Some((*digit, *pos));
        }
    } else {
        if let Some((pos, digit)) = string_digits.last_key_value() {
            return Some((*digit, *pos));
        }
    }

    None
}

fn get_line_value(input: String) -> i32 {
    let chars = input.chars().collect();

    // find first digit
    let found_digs = vec![
        get_dig_chars(&chars, true),
        get_dig_chars(&chars, false),
        get_string_digits(&input, true),
        get_string_digits(&input, false),
    ];

    let mut dig_map = BTreeMap::new();

    for dig in found_digs {
        if let Some((val, pos)) = dig {
            dig_map.insert(pos, val);
        }
    }

    let (_first_pos, first_dig) = dig_map.first_key_value().unwrap();
    let (_last_pos, last_dig) = dig_map.last_key_value().unwrap();

    let string_val = first_dig.to_string() + &last_dig.to_string();
    string_val.parse::<i32>().unwrap()
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
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                // sum += get_dig_chars(line.chars().collect())
                sum += get_line_value(line);
            }
        }

        println!("Sum is {sum}")
    }
}
