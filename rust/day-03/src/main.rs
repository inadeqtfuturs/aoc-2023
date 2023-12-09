use std::fs;

fn main() {
    let _input = fs::read_to_string("./data/input1.txt").unwrap();
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let _res1 = fn1(input);
    println!("Hello, world!");
}

fn fn1(input: &str) -> &'static str {
    let parts_map: Vec<(char, usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| (character, x, y))
        })
        .collect();
    println!("{:#?}", parts_map);

    // [[467, [(0,0), (1, 0), (2,0)]], [113, [(5,0), (6,0), (7,0)]]]
    let mut number_map: Vec<(&char, Vec<(usize, usize)>)> = vec![];
    for (value, x, y) in parts_map.iter() {
        if value.is_numeric() {
            println!("{value}");
            // check number_map
            number_map.push((value, vec![(*x, *y)]));
            let previous_number = number_map.iter().last().unwrap();
            if Some(previous_number).is_some() {
                let previous_index = number_map.len();
                let thing = previous_number.0;
                let next_number = format!("{value}", thing);
                println!("{:?}", previous_number.0)
            }
        }
    }
    println!("{:?}", number_map);
    "hello"
}
