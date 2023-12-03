use std::fs;

struct Bounds {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    // read the input
    let input = fs::read_to_string("./data/input1.txt").unwrap();
    // bounds
    let bounds = Bounds {
        red: 12,
        green: 13,
        blue: 14,
    };

    // get the lines
    let lines = input.lines();

    // vec for any impossible game
    let mut possible_games: Vec<i32> = vec![];

    let mapped_lines: String = lines
        .map(|line| {
            let (game, rounds) = line.split_once(':').unwrap();

            let (_, id) = game.split_once(' ').unwrap();
            let id: i32 = id.parse::<i32>().unwrap();

            let rounds = rounds.trim().split(';').map(|set| {
                let res = set.split(',').map(str::trim);
                res
            });

            let mut possible: bool = true;

            for game in rounds {
                if !possible {
                    break;
                }
                for pull in game {
                    let (number, color) = pull.split_once(' ').unwrap();

                    let color_bound = match color {
                        "red" => bounds.red,
                        "green" => bounds.green,
                        "blue" => bounds.blue,
                        _ => unreachable!(),
                    };

                    // compare against game
                    if color_bound < number.parse::<i32>().unwrap() {
                        possible = false;
                        break;
                    }
                }
            }

            if possible {
                possible_games.push(id);
            }

            line
        })
        .collect();

    println!("{mapped_lines}");

    let res = possible_games.iter().sum::<i32>();
    println!("{res}")
}
