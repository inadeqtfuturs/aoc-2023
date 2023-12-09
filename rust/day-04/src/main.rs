use std::collections::HashSet;
use std::fs;

fn main() {
    println!("day 04");
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let test_winners = get_winners(test_input);
    let test_resp = part1(test_winners);
    println!("part 1 test response: {:#?}", test_resp);
    let test_winners_two = get_winners(test_input);
    let test_resp_part_2 = part2(test_winners_two);
    println!("part 2 test response: {:#?}", test_resp_part_2);

    let input = fs::read_to_string("./data/input.txt").unwrap();

    let winners_per_card = get_winners(&input);
    let part_one = part1(winners_per_card);
    println!("part1 response: {:#?}", part_one);

    let winners_per_card_part_2 = get_winners(&input);
    let part_two = part2(winners_per_card_part_2);
    println!("part2 response: {:#?}", part_two);
}

fn get_winners(input: &str) -> Vec<usize> {
    // get lines
    let lines = input.lines();

    // get winners per card
    let winners_per_card: Vec<usize> = lines
        .map(|card| {
            // split meta from data
            let (_, data) = card.split_once(':').unwrap();
            // split winners from numbers
            let (winners, numbers) = data.trim().split_once('|').unwrap();
            let winners_hash: HashSet<usize> = winners.split(' ').flat_map(str::parse).collect();
            let numbers_hash: HashSet<usize> = numbers.split(' ').flat_map(str::parse).collect();

            winners_hash.intersection(&numbers_hash).count()
        })
        .collect();
    println!("{:?}", winners_per_card);
    winners_per_card
}

fn part1(input: Vec<usize>) -> usize {
    let total_score: usize = input.iter().fold(0, |acc, x| {
        if x < &1 {
            acc
        } else {
            let card_score = (1..*x).fold(1, |inner_acc, _| inner_acc * 2);
            acc + card_score
        }
    });
    total_score
}

fn part2(input: Vec<usize>) -> usize {
    println!("{:#?}", input);
    let mut card_count_at_index = vec![1; input.len()];
    for (i, number_of_winners) in input.iter().enumerate() {
        for j in i + 1..=i + number_of_winners {
            if card_count_at_index.get(j).is_none() {
                break;
            }
            card_count_at_index[j] += card_count_at_index[i];
        }
    }
    card_count_at_index.iter().sum::<usize>()
}
