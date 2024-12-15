use core::panic;
use std::{collections::HashMap, time::Instant};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Move = (isize, isize);
type Input = (Map, Position, isize, Vec<Move>);

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (mut map, mut pos, _size, moves) = input.clone();
    for mov in moves {
        step(&mut map, &mut pos, mov);
    }
    map.iter()
        .filter_map(|((x, y), c)| {
            if c == &'O' {
                Some((x + 100 * y) as usize)
            } else {
                None
            }
        })
        .sum()
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn step(map: &mut Map, pos: &mut Position, mov: Move) {
    let next_pos = (pos.0 + mov.0, pos.1 + mov.1);
    match map.get(&next_pos).unwrap() {
        '#' => {}
        '.' => {
            *pos = next_pos;
        }
        'O' => {
            let mut next_boxes = vec![];
            let mut i = 1;
            loop {
                let maybe_box_pos = (next_pos.0 + i * mov.0, next_pos.1 + i * mov.1);
                match map.get(&maybe_box_pos).unwrap() {
                    '.' => {
                        next_boxes.push(maybe_box_pos);
                        break;
                    }
                    'O' => {
                        next_boxes.push(maybe_box_pos);
                        i += 1;
                    }
                    '#' => {
                        next_boxes = vec![];
                        break;
                    }
                    _ => panic!("Oops"),
                }
            }
            if !next_boxes.is_empty() {
                next_boxes
                    .iter()
                    .map(|next_box_pos| {
                        let prev_box_pos = (next_box_pos.0 - mov.0, next_box_pos.1 - mov.1);
                        let prev_char = *map
                            .get(&(prev_box_pos.0 - mov.0, prev_box_pos.1 - mov.1))
                            .unwrap();
                        (next_box_pos, prev_char)
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|(next_box_pos, prev_char)| {
                        let prev_box_pos = (next_box_pos.0 - mov.0, next_box_pos.1 - mov.1);
                        *map.get_mut(next_box_pos).unwrap() = 'O';
                        let prev = map.get_mut(&prev_box_pos).unwrap();
                        *prev = *prev_char;
                    });
                *pos = next_pos;
            }
        }
        _ => panic!("Oops"),
    }
}

fn parse(input: &str) -> Input {
    let (map_data, moves_data) = input.trim().split_once("\n\n").unwrap();
    let mut map: Map = map_data
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect::<Vec<_>>()
        })
        .collect();
    let start = *map
        .iter()
        .find_map(|(pos, c)| if c == &'@' { Some(pos) } else { None })
        .unwrap();
    *map.get_mut(&start).unwrap() = '.';
    (
        map,
        start,
        map_data.lines().count() as isize,
        moves_data
            .lines()
            .collect::<String>()
            .trim()
            .chars()
            .map(|c| match c {
                '<' => (-1, 0),
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                _ => panic!("Oops"),
            })
            .collect(),
    )
}

#[allow(unused)]
fn print_map(map: &Map, current_pos: Position, size: isize) {
    for (pos, c) in map {}
    for y in 0..size {
        for x in 0..size {
            print!(
                "{}",
                if (x, y) == current_pos {
                    '@'
                } else {
                    *map.get(&(x, y)).unwrap()
                }
            );
        }
        println!();
    }
    println!();
}
