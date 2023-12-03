use std::fs;

fn main() {
    // read the input
    let input = fs::read_to_string("./data/input1.txt").unwrap();

    // get the lines
    let lines = input.lines();

    let game_sums: i32 = lines.fold(0, |total, line| {
        let (game, rounds) = line.split_once(':').unwrap();
        println!("{game}");
        println!("{:#?}", rounds);
        let rounds = rounds.trim().split(';').map(|set| {
            let res = set.split(',').map(str::trim);
            res
        });

        let game_results: (i32, i32, i32) = rounds.fold((1, 1, 1), |round_acc, round| {
            round.fold(round_acc, |acc, pull| {
                let (number, color) = pull.split_once(' ').unwrap();
                let color_number = number.parse::<i32>().unwrap();
                let round_number = match color {
                    "blue" => round_acc.0,
                    "green" => round_acc.1,
                    "red" => round_acc.2,
                    _ => 1,
                };

                if color_number > round_number {
                    match color {
                        "blue" => (color_number, acc.1, acc.2),
                        "green" => (acc.0, color_number, acc.2),
                        "red" => (acc.0, acc.1, color_number),
                        _ => acc,
                    }
                } else {
                    acc
                }
            })
        });

        println!("{:?}", game_results);

        let game_sum = game_results.0 * game_results.1 * game_results.2;

        println!("{game_sum}");

        total + game_sum
    });

    println!("{game_sums}");
}
