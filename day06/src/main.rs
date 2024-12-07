use core::panic;
use std::collections::HashSet;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (((usize, usize), char), usize, Vec<Vec<char>>);

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let test_visited = part_1(&test_input);
    let visited = part_1(&input);
    println!("Part 1   test          {} ", test_visited.len());
    println!("         validation    {} ", visited.len());
    println!(
        "Part 2   test          {} ",
        part_2(&test_input, test_visited)
    );
    println!("         validation    {} ", part_2(&input, visited));
}

fn part_1(input: &Input) -> HashSet<(usize, usize)> {
    let ((mut pos, mut dir), max_xy, map) = input;
    let mut visited = HashSet::with_capacity(max_xy.pow(2));
    loop {
        visited.insert(pos);
        if let Some((next_pos, next_dir)) = step(pos, dir, *max_xy, map, None) {
            pos = next_pos;
            dir = next_dir;
        } else {
            break;
        }
    }
    visited
}

fn part_2(input: &Input, obstacles: HashSet<(usize, usize)>) -> usize {
    let ((start_pos, start_dir), max_xy, map) = input;
    let mut loops = 0;
    for obstacle in obstacles {
        let (mut pos, mut dir) = (*start_pos, *start_dir);
        let mut visited = HashSet::new();
        loop {
            if visited.contains(&(pos, dir)) {
                loops += 1;
                break;
            }
            visited.insert((pos, dir));
            if let Some((next_pos, next_dir)) = step(pos, dir, *max_xy, map, Some(obstacle)) {
                pos = next_pos;
                dir = next_dir;
            } else {
                break;
            }
        }
    }

    loops
}

fn step(
    pos: (usize, usize),
    dir: char,
    max_xy: usize,
    map: &Vec<Vec<char>>,
    obstacle: Option<(usize, usize)>,
) -> Option<((usize, usize), char)> {
    let next_pos = match (pos, dir) {
        ((_, 0), '^') | ((0, _), '<') => None,
        ((x, y), '^') => Some((x, y - 1)),
        ((x, _), '>') if x == max_xy => None,
        ((x, y), '>') => Some((x + 1, y)),
        ((_, y), 'v') if y == max_xy => None,
        ((x, y), 'v') => Some((x, y + 1)),
        ((x, y), '<') => Some((x - 1, y)),
        _ => panic!("oops"),
    };
    if let Some(next_pos) = next_pos {
        let at_pos = if Some(next_pos) == obstacle {
            '#'
        } else {
            *map[next_pos.1].get(next_pos.0).unwrap()
        };
        match at_pos {
            '#' => step(
                pos,
                match dir {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("Oops"),
                },
                max_xy,
                map,
                obstacle,
            ),
            '.' | '^' | '>' | 'v' | '<' => Some((next_pos, dir)),
            _ => panic!("Oops"),
        }
    } else {
        None
    }
}

fn parse(input: &str) -> Input {
    let map: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let start = map
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .find(|(_, c)| c == &'^')
        .unwrap();
    (start, map.len() - 1, map)
}
