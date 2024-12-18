use core::panic;
use std::{collections::HashMap, time::Instant};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
// const INPUT_TEST_3: &str = include_str!("../input_test_3.txt");
// const INPUT_TEST_PART_2_SIMPLE: &str = include_str!("../input_test_part2_simple.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Move = (isize, isize);
type Input = (Map, Position, (isize, isize), Vec<Move>);

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1, false);
    let test_input_2 = parse(INPUT_TEST_2, false);
    // let test_input_part2_simple = parse(INPUT_TEST_PART_2_SIMPLE, true);
    let input = parse(INPUT, false);
    println!("Part 1   test (simple) {} ", part_1(&test_input_2));
    println!("Part 1   test          {} ", part_1(&test_input_1));
    println!("         validation    {} ", part_1(&input));
    // println!(
    //     "Part 2   test          {} ",
    //     part_2(&test_input_part2_simple)
    // );
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (mut map, mut pos, _size, moves) = input.clone();
    // print_map(&map, pos, _size);
    for mov in moves {
        step(&mut map, &mut pos, mov);
        // print_map(&map, pos, _size);
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

// fn part_2(input: &Input) -> usize {
//     let (mut map, mut pos, size, moves) = input.clone();
//     // print_map(&map, pos, size);
//     for mov in moves {
//         step(&mut map, &mut pos, mov);
//         // print_map(&map, pos, size);
//     }
//     42
// }

fn step(map: &mut Map, pos: &mut Position, mov: Move) {
    let next_pos = (pos.0 + mov.0, pos.1 + mov.1);
    match map.get(&next_pos).unwrap() {
        '#' => {}
        '.' => {
            *pos = next_pos;
        }
        'O' => {
            // 1. Find target (next .) on axis
            let (mut target_pos, mut offset) = (None, 1);
            loop {
                let maybe_target_pos = (next_pos.0 + offset * mov.0, next_pos.1 + offset * mov.1);
                let at_target_pos = map.get(&maybe_target_pos).unwrap();
                if at_target_pos == &'.' {
                    target_pos = Some(maybe_target_pos);
                    break;
                } else if at_target_pos == &'#' {
                    break;
                }
                offset += 1;
            }

            // 2. List boxes we are trying to move
            let mut boxes_to_move = vec![];
            if let Some(target_pos) = target_pos {
                boxes_to_move.push(next_pos);
                let mut offset = 1;
                loop {
                    let box_pos = (next_pos.0 + offset * mov.0, next_pos.1 + offset * mov.1);
                    if box_pos == target_pos {
                        break;
                    }
                    boxes_to_move.push(box_pos);
                    offset += 1;
                }
            }

            // 3. Check if all boxes can move
            let boxes_can_move = !boxes_to_move.is_empty()
                && boxes_to_move
                    .iter()
                    .all(|box_pos| map.get(box_pos).unwrap() != &'#');

            // 4. Move if all boxes can move
            if boxes_can_move {
                boxes_to_move.iter().for_each(|box_pos| {
                    *map.get_mut(box_pos).unwrap() = '.';
                });
                boxes_to_move.iter().for_each(|box_pos| {
                    *map.get_mut(&(box_pos.0 + mov.0, box_pos.1 + mov.1))
                        .unwrap() = 'O';
                });
                *pos = next_pos;
            }
        }
        '[' | ']' => {}
        _ => panic!("Oops"),
    }
}

fn parse(input: &str, double: bool) -> Input {
    let (map_data, moves_data) = input.trim().split_once("\n\n").unwrap();
    let mut map: Map = map_data
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.trim()
                .chars()
                .flat_map(|c| {
                    if double {
                        match c {
                            'O' => vec!['[', ']'],
                            '@' => vec!['@', '.'],
                            o if o == '.' || o == '#' => vec![o, o],
                            _ => panic!("Oops"),
                        }
                    } else {
                        vec![c]
                    }
                })
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
    let map_line_count = map_data.lines().count() as isize;
    (
        map,
        start,
        (map_line_count * if double { 2 } else { 1 }, map_line_count),
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
fn print_map(map: &Map, current_pos: Position, size: (isize, isize)) {
    for (pos, c) in map {}
    for y in 0..size.1 {
        for x in 0..size.0 {
            print!(
                "{}",
                if (x, y) == current_pos {
                    '@'
                } else {
                    *map.get(&(x, y)).unwrap_or_else(|| {
                        dbg!((x, y));
                        panic!("OOps");
                    })
                }
            );
        }
        println!();
    }
    println!();
}
