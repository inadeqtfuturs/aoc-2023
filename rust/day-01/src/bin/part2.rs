use std::fs;

fn main() {
    // read the input
    let input = fs::read_to_string("./data/input1.txt").unwrap();

    // get the lines
    let lines = input.lines();

    // map over lines
    let mapped_lines = lines.map(|line| {
        let new_line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let first = new_line.chars().find(|c| c.is_numeric()).unwrap();
        let last = new_line.chars().rev().find(|c| c.is_numeric()).unwrap();

        // debug
        // dbg!(first);
        // dbg!(last);

        let res: usize = format!("{first}{last}").parse().unwrap();
        res
    });

    let sum: usize = mapped_lines.sum();

    println!("{sum}")
}
