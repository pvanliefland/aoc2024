use core::panic;
use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = HashMap<(isize, isize), char>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let test_p1 = part_1(&test_input);
    let p1 = part_1(&input);
    println!("Part 1   test          {} ", test_p1.len());
    println!("         validation    {} ", p1.len());
    println!("Part 2   test          {} ", part_2(&test_input, test_p1));
    println!("         validation    {} ", part_2(&input, p1));
}

fn part_1(input: &Input) -> HashSet<(isize, isize)> {
    let mut current = input
        .iter()
        .find(|(_, c)| *c == &'^')
        .map(|(p, c)| (*p, *c))
        .unwrap();
    let mut visited = HashSet::new();
    loop {
        let (dx, dy) = match current.1 {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Oops"),
        };
        let next = (current.0 .0 + dx, current.0 .1 + dy);
        match input.get(&next) {
            Some('#') => {
                let next_dir = match current.1 {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("Oops"),
                };
                current = (current.0, next_dir);
            }
            Some('.' | '^' | '>' | 'v' | '<') => {
                visited.insert(current.0);
                current = (next, current.1);
            }
            Some(_) => panic!("Oops"),
            None => {
                visited.insert(current.0);
                break;
            }
        }
    }
    visited
}

fn part_2(input: &Input, part_1: HashSet<(isize, isize)>) -> usize {
    let mut loops = 0;

    for candidate in part_1 {
        let mut current = input
            .iter()
            .find(|(_, c)| *c == &'^')
            .map(|(p, c)| (*p, *c))
            .unwrap();
        let mut visited = HashSet::new();

        loop {
            let (dx, dy) = match current.1 {
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                _ => panic!("Oops"),
            };
            let next = (current.0 .0 + dx, current.0 .1 + dy);
            match (next == candidate, input.get(&next)) {
                (true, _) | (false, Some('#')) => {
                    let next_dir = match current.1 {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        _ => panic!("Oops"),
                    };
                    current = (current.0, next_dir);
                }
                (false, Some('.' | '^' | '>' | 'v' | '<')) => {
                    if visited.contains(&(current.0, current.1)) {
                        loops += 1;
                        break;
                    }
                    visited.insert((current.0, current.1));
                    current = (next, current.1);
                }
                (false, Some(_)) => panic!("Oops"),
                (false, None) => {
                    visited.insert((current.0, current.1));
                    break;
                }
            }
        }
    }

    loops
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect::<Vec<_>>()
        })
        .collect()
}
