use std::{collections::HashMap, time::Instant};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Stone = (usize, usize);
type Input = Vec<Stone>;

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", count_stones(&test_input, 25));
    println!("         validation    {} ", count_stones(&input, 25));
    println!("Part 2   test          {} ", count_stones(&test_input, 75));
    println!("         validation    {} ", count_stones(&input, 75));
    println!("Duration: {:?}", start.elapsed());
}

fn count_stones(input: &Input, blinks: usize) -> usize {
    let mut transformed = input.clone();
    for _ in 0..blinks {
        transformed = step(&transformed);
        let mut map = HashMap::new();
        for stone in &transformed {
            map.entry(stone.0)
                .and_modify(|count| *count += stone.1)
                .or_insert(stone.1);
        }
        transformed = map.into_iter().collect();
    }
    transformed.iter().map(|s| s.1).sum()
}

fn step(input: &Input) -> Input {
    input
        .iter()
        .flat_map(|stone| match stone.0 {
            0 => vec![(1, stone.1)],
            num if num.ilog10() % 2 == 1 => {
                let as_string = num.to_string();
                let (left, right) = as_string.split_at(as_string.len() / 2);
                vec![
                    (left.parse().unwrap(), stone.1),
                    (right.parse().unwrap(), stone.1),
                ]
            }
            _ => vec![(stone.0 * 2024, stone.1)],
        })
        .collect()
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .split(" ")
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}
