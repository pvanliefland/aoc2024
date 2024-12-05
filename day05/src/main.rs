use std::cmp::Ordering;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = [Vec<Vec<u32>>; 2];

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> u32 {
    input[1]
        .iter()
        .filter_map(|update| {
            if update.windows(2).all(|p1p2| {
                input[0]
                    .iter()
                    .any(|rule| rule[0] == p1p2[0] && rule[1] == p1p2[1])
            }) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &Input) -> u32 {
    input[1]
        .iter()
        .filter_map(|update| {
            if update.windows(2).any(|p1p2| {
                !input[0]
                    .iter()
                    .any(|rule| rule[0] == p1p2[0] && rule[1] == p1p2[1])
            }) {
                let mut reordered = update.clone();
                reordered.sort_by(|a, b| {
                    if input[0].iter().any(|rule| &rule[0] == a && &rule[1] == b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                Some(reordered[reordered.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.split(&[',', '|'])
                        .map(|item| item.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
