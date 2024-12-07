use core::panic;
use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = HashMap<(isize, isize), char>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let (test_p1, test_targets) = part_1(&test_input);
    let (p1, targets) = part_1(&input);
    println!("Part 1   test     {test_p1} ");
    println!("         actual   {p1} ");
    println!("Part 2   test     {} ", part_2(&test_input, test_targets));
    println!("         actual   {} ", part_2(&input, targets));
}

fn part_1(input: &Input) -> (usize, HashSet<(isize, isize)>) {
    let (mut current_pos, mut current_dir) = input
        .iter()
        .find(|(_, c)| *c == &'^')
        .map(|(p, c)| (*p, *c))
        .unwrap();
    let mut visited = HashSet::new();
    let mut targets = HashSet::new();
    loop {
        match step(input, (current_pos, current_dir)) {
            Outcome::Move(next_pos) => {
                visited.insert(current_pos);
                targets.insert(current_pos);
                current_pos = next_pos;
            }
            Outcome::Turn(next_dir) => {
                current_dir = next_dir;
            }
            Outcome::GetOut => {
                visited.insert(current_pos);
                targets.insert(current_pos);
                break;
            }
        }
    }
    (visited.len(), targets)
}

enum Outcome {
    Turn(char),
    Move((isize, isize)),
    GetOut,
}

fn part_2(input: &Input, part_1: HashSet<(isize, isize)>) -> usize {
    let mut loops = 0;

    for candidate in part_1 {
        let (mut current_pos, mut current_dir) = input
            .iter()
            .find(|(_, c)| *c == &'^')
            .map(|(p, c)| (*p, *c))
            .unwrap();
        let mut visited = HashSet::new();

        loop {
            let (dx, dy) = match current_dir {
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                _ => panic!("Oops"),
            };
            let next_pos = (current_pos.0 + dx, current_pos.1 + dy);
            match (next_pos == candidate, input.get(&next_pos)) {
                (true, _) | (false, Some('#')) => {
                    let next_dir = match current_dir {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        _ => panic!("Oops"),
                    };
                    current_dir = next_dir;
                }
                (false, Some('.' | '^' | '>' | 'v' | '<')) => {
                    if visited.contains(&(current_pos, current_dir)) {
                        loops += 1;
                        break;
                    }
                    visited.insert((current_pos, current_dir));
                    current_pos = next_pos;
                }
                (false, Some(_)) => panic!("Oops"),
                (false, None) => {
                    visited.insert((current_pos, current_dir));
                    break;
                }
            }
        }
    }

    loops
}

fn step(map: &Input, current: ((isize, isize), char)) -> Outcome {
    let (dx, dy) = match current.1 {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("Oops"),
    };
    let next = (current.0 .0 + dx, current.0 .1 + dy);
    match map.get(&next) {
        Some('#') => {
            let next_dir = match current.1 {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Oops"),
            };
            Outcome::Turn(next_dir)
        }
        Some('.' | '^' | '>' | 'v' | '<') => Outcome::Move(next),
        Some(_) => panic!("Oops"),
        None => Outcome::GetOut,
    }
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()),
                        c,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
