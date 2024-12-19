use std::{collections::HashMap, time::Instant};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (Vec<String>, Vec<String>);

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (available, wanted) = input;

    wanted.iter().filter(|w| is_possible(w, available)).count()
}

fn part_2(input: &Input) -> usize {
    let (available, wanted) = input;
    let mut cache = HashMap::new();

    wanted
        .iter()
        .map(|w| count_possible(w, available, &mut cache))
        .sum()
}

fn is_possible(wanted: &str, available: &Vec<String>) -> bool {
    for towel in available {
        if wanted == towel
            || (towel.len() <= wanted.len()
                && &wanted[0..towel.len()] == towel
                && is_possible(&wanted[towel.len()..], available))
        {
            return true;
        }
    }
    false
}
fn count_possible<'w>(
    wanted: &'w str,
    available: &Vec<String>,
    cache: &mut HashMap<&'w str, usize>,
) -> usize {
    if !cache.contains_key(wanted) {
        let mut count = 0;
        for towel in available {
            if wanted == towel {
                count += 1;
            } else if towel.len() <= wanted.len()
                && &wanted[0..towel.len()] == towel
                && is_possible(&wanted[towel.len()..], available)
            {
                count += count_possible(&wanted[towel.len()..], available, cache);
            }
        }
        cache.insert(wanted, count);
    }
    *cache.get(wanted).unwrap()
}

fn parse(input: &str) -> Input {
    let (available, wanted) = input.trim().split_once("\n\n").unwrap();
    (
        available.split(", ").map(String::from).collect(),
        wanted.lines().map(String::from).collect(),
    )
}
