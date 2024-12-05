use std::{cmp::Ordering, collections::HashSet, hash::RandomState};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (HashSet<(u32, u32), RandomState>, Vec<Vec<u32>>);

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let (test_p1, test_p2) = part_1_2(&test_input);
    let (p1, p2) = part_1_2(&input);
    println!("Part 1   test          {} ", test_p1);
    println!("         validation    {} ", p1);
    println!("Part 2   test          {} ", test_p2);
    println!("         validation    {} ", p2);
}

fn part_1_2(input: &Input) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    input.1.iter().for_each(|update| {
        if update
            .windows(2)
            .all(|p1p2| input.0.contains(&(p1p2[0], p1p2[1])))
        {
            p1 += update[update.len() / 2];
        } else {
            let mut reordered = update.clone();
            reordered.sort_by(|&a, &b| {
                if input.0.contains(&(a, b)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            p2 += reordered[reordered.len() / 2]
        }
    });
    (p1, p2)
}

fn parse(input: &str) -> Input {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();
    let rules = HashSet::<_>::from_iter(rules.lines().map(|rule| {
        let (p1, p2) = rule.split_once('|').unwrap();
        (p1.parse().unwrap(), p2.parse().unwrap())
    }));
    (
        rules,
        updates
            .lines()
            .map(|line| {
                line.split(&[',', '|'])
                    .map(|item| item.parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}
