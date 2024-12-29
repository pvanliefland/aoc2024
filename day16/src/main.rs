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
    let test_shortest_1 = part_1(&test_input_1);
    let test_shortest_2 = part_1(&test_input_2);
    let shortest = part_1(&input);
    println!("Part 1   test 1        {}", test_shortest_1);
    println!("Part 1   test 2        {}", test_shortest_2);
    println!("         validation    {}", shortest);
    println!(
        "Part 2   test 1        {}",
        part_2(&test_input_1, test_shortest_1)
    );
    println!(
        "Part 2   test 2        {}",
        part_2(&test_input_2, test_shortest_2)
    );
    println!("         validation    {} ", part_2(&input, shortest));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (map, start) = input;
    escape(map, start, None, Some(1)).first().unwrap().0
}
fn part_2(input: &Input, max_cost: usize) -> usize {
    let (map, start) = input;
    escape(map, start, Some(max_cost), None)
        .iter()
        .flat_map(|(_, path)| path)
        .collect::<HashSet<_>>()
        .len()
}

fn escape(
    map: &Map,
    start: &Position,
    max_cost: Option<usize>,
    max_paths: Option<usize>,
) -> Vec<(usize, Vec<Position>)> {
    let mut queue = VecDeque::from_iter(vec![Action::Move('>', *start, 0, vec![*start])]);
    let mut explored: HashMap<(char, Position), usize> =
        HashMap::from_iter(vec![(('>', *start), 0)]);
    let mut escapes = vec![];
    while let Some(action) = queue.pop_front() {
        if max_cost.is_some_and(|max| action.cost() > max) {
            continue;
        }
        match action {
            Action::Move(dir, pos, cost, path) => {
                let at_pos = map.get(&pos).unwrap();
                if at_pos == &'E' {
                    escapes.push((cost, path.clone()));
                    if max_paths.is_some_and(|max| escapes.len() == max) {
                        break;
                    }
                }
                for mov in [(1, 0, '>'), (0, 1, 'v'), (-1, 0, '<'), (0, -1, '^')] {
                    let adj_pos = (pos.0 + mov.0, pos.1 + mov.1);
                    let at_adj_pos = map.get(&adj_pos).unwrap();
                    let next_path = [path.clone(), vec![adj_pos]].concat();
                    if at_adj_pos != &'#'
                        && explored
                            .get(&(mov.2, adj_pos))
                            .is_none_or(|prev_cost| prev_cost >= &cost)
                    {
                        if dir == mov.2 {
                            explored.insert((mov.2, adj_pos), cost);
                            queue.push_back(Action::Move(mov.2, adj_pos, cost + 1, next_path));
                        } else {
                            queue.push_back(Action::WaitAndMove(
                                1000,
                                mov.2,
                                adj_pos,
                                cost + 1,
                                next_path,
                            ));
                        }
                    }
                }
            }
            Action::WaitAndMove(time, dir, pos, cost, path) => {
                if time == 0 {
                    queue.push_back(Action::Move(dir, pos, cost, path));
                } else {
                    queue.push_back(Action::WaitAndMove(time - 1, dir, pos, cost + 1, path));
                }
            }
        }
    }
    escapes
}

#[derive(Debug)]
enum Action {
    Move(char, Position, usize, Vec<Position>),
    WaitAndMove(usize, char, Position, usize, Vec<Position>),
}
impl Action {
    fn cost(&self) -> usize {
        match self {
            Self::Move(_, _, cost, _) => *cost,
            Self::WaitAndMove(_, _, _, cost, _) => *cost,
        }
    }
}

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
