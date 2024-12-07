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
    let (mut pos, mut dir) = input
        .iter()
        .find(|(_, c)| *c == &'^')
        .map(|(p, c)| (*p, *c))
        .unwrap();
    let mut visited = HashSet::new();
    let mut targets = HashSet::new();
    loop {
        match step(input, pos, dir, None) {
            Outcome::Move(next_pos) => {
                visited.insert(pos);
                targets.insert(pos);
                pos = next_pos;
            }
            Outcome::Turn(next_dir) => {
                dir = next_dir;
            }
            Outcome::GetOut => {
                visited.insert(pos);
                targets.insert(pos);
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
    let (init_pos, init_dir) = input
        .iter()
        .find(|(_, c)| *c == &'^')
        .map(|(p, c)| (*p, *c))
        .unwrap();
    let mut loops = 0;
    for candidate in part_1 {
        let mut visited = HashSet::new();
        let (mut pos, mut dir) = (init_pos, init_dir);
        loop {
            match step(input, pos, dir, Some(candidate)) {
                Outcome::Move(next_pos) => {
                    if visited.contains(&(pos, dir)) {
                        loops += 1;
                        break;
                    }
                    visited.insert((pos, dir));
                    pos = next_pos;
                }
                Outcome::Turn(next_dir) => {
                    dir = next_dir;
                }
                Outcome::GetOut => {
                    visited.insert((pos, dir));
                    break;
                }
            }
        }
    }

    loops
}

fn step(map: &Input, pos: (isize, isize), dir: char, obstacle: Option<(isize, isize)>) -> Outcome {
    let (dx, dy) = match dir {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("Oops"),
    };
    let next_pos = (pos.0 + dx, pos.1 + dy);
    let at_pos = if Some(next_pos) == obstacle {
        Some(&'#')
    } else {
        map.get(&next_pos)
    };
    match at_pos {
        Some('#') => Outcome::Turn(match dir {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Oops"),
        }),
        Some('.' | '^' | '>' | 'v' | '<') => Outcome::Move(next_pos),
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
