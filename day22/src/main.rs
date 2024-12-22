use std::{collections::HashMap, time::Instant};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<isize>;

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input_1));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> isize {
    input
        .iter()
        .map(|original_secret| {
            let mut secret = *original_secret;
            for _ in 0..2000 {
                secret = mix_and_prune(secret);
            }
            secret
        })
        .sum()
}

fn part_2(input: &Input) -> usize {
    let monkeys_prices = input
        .iter()
        .map(|original_secret| {
            let mut secret = *original_secret;
            let mut prices = vec![*original_secret % 10];
            for _ in 0..2000 {
                secret = mix_and_prune(secret);
                prices.push(secret % 10);
            }
            prices
        })
        .collect::<Vec<_>>();
    let monkeys_price_changes = monkeys_prices
        .iter()
        .enumerate()
        .map(|(monkey, monkey_prices)| {
            monkey_prices
                .windows(2)
                .map(|prices| (monkey, prices[1], prices[1] - prices[0]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sequences = monkeys_price_changes
        .iter()
        .flat_map(|price_changes| price_changes.windows(4))
        .collect::<Vec<_>>();
    let mut sequence_gains: HashMap<Vec<isize>, (usize, Vec<usize>)> = HashMap::new();
    sequences.iter().for_each(|seq| {
        let key = seq
            .iter()
            .map(|(_monkey, _price, change)| *change)
            .collect::<Vec<_>>();
        sequence_gains
            .entry(key.clone())
            .and_modify(|(num, monkeys): &mut (_, _)| {
                if !monkeys.contains(&seq[0].0) {
                    *num += seq[seq.len() - 1].1 as usize;
                    monkeys.push(seq[0].0);
                }
            })
            //.and_modify(|num| *num += seq[seq.len() - 1].0)
            .or_insert((seq[seq.len() - 1].1 as usize, vec![seq[0].0]));
    });
    let max = sequence_gains.iter().max_by_key(|entry| entry.1).unwrap();
    max.1 .0
}

fn mix_and_prune(secret: isize) -> isize {
    let mut secret = secret;
    secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
