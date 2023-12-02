use std::fs;

fn main() {
    // read the input
    let input = fs::read_to_string("./data/input1.txt").unwrap();

    // get the lines
    let lines = input.lines();

    // map over lines
    let mapped_lines = lines.map(|line| {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();

        // debug
        // dbg!(first);
        // dbg!(last);

        let res: usize = format!("{first}{last}").parse().unwrap();
        res
    });

    let sum: usize = mapped_lines.sum();

    println!("{sum}")
}
