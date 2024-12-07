use core::panic;
use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Input = (HashMap<Position, char>, (Position, char));

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let test_visited = part_1(&test_input);
    let visited = part_1(&input);
    println!("Part 1   test     {}", test_visited.len());
    println!("         actual   {}", visited.len());
    println!("Part 2   test     {} ", part_2(&test_input, test_visited));
    println!("         actual   {} ", part_2(&input, visited));
}

fn part_1(input: &Input) -> HashSet<Position> {
    let (map, (mut pos, mut dir)) = input;
    let mut visited = HashSet::new();
    loop {
        match step(map, pos, dir, None) {
            Outcome::Move(next_pos) => {
                visited.insert(pos);
                pos = next_pos;
            }
            Outcome::Turn(next_dir) => {
                dir = next_dir;
            }
            Outcome::GetOut => {
                visited.insert(pos);
                break;
            }
        }
    }
    visited
}

fn part_2(input: &Input, part_1: HashSet<(isize, isize)>) -> usize {
    let (map, (start_pos, start_dir)) = input;
    let mut loops = 0;
    for candidate in part_1 {
        let mut visited = HashSet::new();
        let (mut pos, mut dir) = (*start_pos, *start_dir).to_owned();
        loop {
            match step(map, pos, dir, Some(candidate)) {
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

fn step(
    map: &HashMap<Position, char>,
    pos: Position,
    dir: char,
    obstacle: Option<Position>,
) -> Outcome {
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

enum Outcome {
    Turn(char),
    Move(Position),
    GetOut,
}

fn parse(input: &str) -> Input {
    let mut start = ((0, 0), '!');
    (
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let x = isize::try_from(x).unwrap();
                        let y = isize::try_from(y).unwrap();
                        if c == '^' {
                            start = ((x, y), '^');
                        }
                        ((x, y), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        start,
    )
}
