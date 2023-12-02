use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct PullData {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_game_data(input: String) -> Vec<PullData> {
    let mut pull_data = Vec::new();
    let pull_str_data = input.split(";");

    for pull in pull_str_data {
        let mut current_pull = PullData {
            red: 0,
            blue: 0,
            green: 0,
        };

        let cubes = pull.split(",");

        for cube in cubes {
            let parsed = sscanf::sscanf!(cube.trim(), "{u32} {String}");
            if let Ok((cube_count, color)) = parsed {
                match color.as_str() {
                    "red" => current_pull.red = cube_count,
                    "blue" => current_pull.blue = cube_count,
                    "green" => current_pull.green = cube_count,
                    _ => {}
                };
            } else {
            }
        }

        pull_data.push(current_pull);
    }
    pull_data
}

// fn check_game_data(pull_data: Vec<PullData>) -> bool {
//     const RED_LIM: u32 = 12;
//     const GREEN_LIM: u32 = 13;
//     const BLUE_LIM: u32 = 14;
//
//     for pull in pull_data {
//         if pull.red > RED_LIM || pull.green > GREEN_LIM || pull.blue > BLUE_LIM {
//             return false;
//         }
//     }
//
//     true
// }

fn get_min_cube_set(pull_data: Vec<PullData>) -> u32 {
    let mut red_values = Vec::new();
    let mut blue_values = Vec::new();
    let mut green_values = Vec::new();

    for pull in pull_data {
        red_values.push(pull.red);
        blue_values.push(pull.blue);
        green_values.push(pull.green);
    }

    red_values.iter().max().unwrap()
        * blue_values.iter().max().unwrap()
        * green_values.iter().max().unwrap()
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
        let mut valid_games = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for wrapped_line in lines {
            if let Ok(line) = wrapped_line {
                let parsed = sscanf::sscanf!(line, "Game {i32}: {String}");
                if let Ok((game_id, game_string)) = parsed {
                    let pull_data = parse_game_data(game_string);
                    valid_games.push(get_min_cube_set(pull_data))
                }
            }
        }
        println!("Total valid games sum: {}", valid_games.iter().sum::<u32>())
    }
}
