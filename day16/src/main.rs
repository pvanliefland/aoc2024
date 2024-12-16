use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Input = (Map, Position);

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test 1        {} ", part_1(&test_input_1));
    println!("Part 1   test 2        {} ", part_1(&test_input_2));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (map, start) = input;
    let mut queue = VecDeque::from_iter(vec![Action::Move('>', *start, 0)]);
    let mut explored: HashSet<(char, Position)> = HashSet::from_iter(vec![('>', *start)]);
    let mut end_move = None;
    while let Some(action) = queue.pop_front() {
        match action {
            Action::Move(dir, pos, cost) => {
                let at_pos = map.get(&pos).unwrap();
                if at_pos == &'E' {
                    end_move = Some(action);
                    break;
                }
                for mov in [(1, 0, '>'), (0, 1, 'v'), (-1, 0, '<'), (0, -1, '^')] {
                    let adj_pos = (pos.0 + mov.0, pos.1 + mov.1);
                    let at_adj_pos = map.get(&adj_pos).unwrap();
                    if at_adj_pos != &'#' && !explored.contains(&(mov.2, adj_pos)) {
                        if dir == mov.2 {
                            explored.insert((mov.2, adj_pos));
                            queue.push_back(Action::Move(mov.2, adj_pos, cost + 1));
                        } else {
                            queue.push_back(Action::WaitAndMove(1000, mov.2, adj_pos, cost + 1));
                        }
                    }
                }
            }
            Action::WaitAndMove(time, dir, pos, cost) => {
                if time == 0 {
                    queue.push_back(Action::Move(dir, pos, cost));
                } else {
                    queue.push_back(Action::WaitAndMove(time - 1, dir, pos, cost + 1));
                }
            }
        }
    }
    match end_move {
        Some(Action::Move(_dir, _pos, cost)) => cost,
        _ => panic!("Oops"),
    }
}

#[derive(Debug)]
enum Action {
    Move(char, Position, usize),
    WaitAndMove(usize, char, Position, usize),
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    let mut start = None;
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some((x as isize, y as isize));
                    }
                    ((x as isize, y as isize), c)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (map, start.unwrap())
}
