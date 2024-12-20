use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Input = (Map, Position, Position, usize);

#[derive(Debug, Clone)]
struct Action {
    pos: Position,
    kind: ActionKind,
    has_cheated: bool,
    steps: usize,
    explored: HashSet<Position>,
    cheat_pos_1: Option<Position>,
    cheat_pos_2: Option<Position>,
}
#[derive(Debug, Clone)]
enum ActionKind {
    Start,
    Move,
    Cheat,
    Escape,
}

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
    let (map, start, end, _size) = input;
    let without_cheating = escape(map, *start, *end);
    cheat_and_escape(map, *start, *end, without_cheating.unwrap())
        .iter()
        .map(|action| (action.cheat_pos_1.unwrap(), action.cheat_pos_2.unwrap()))
        .collect::<HashSet<_>>()
        .len()
}

fn escape(map: &Map, start: Position, end: Position) -> Option<usize> {
    let mut queue = VecDeque::from_iter([(start, 0)]);
    let mut explored: HashSet<Position> = HashSet::from_iter([start]);
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return Some(steps);
        }
        for mov in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let adj_pos = (pos.0 + mov.0, pos.1 + mov.1);
            if explored.contains(&adj_pos) {
                continue;
            }
            explored.insert(adj_pos);
            if let Some('.' | 'E') = map.get(&adj_pos) {
                queue.push_back((adj_pos, steps + 1));
            }
        }
    }
    None
}

fn cheat_and_escape(
    map: &Map,
    start: Position,
    end: Position,
    score_to_beat: usize,
) -> Vec<Action> {
    let mut cache: HashMap<Position, Action> = HashMap::new();
    let mut escapes = vec![];
    let mut queue = VecDeque::from_iter([Action {
        pos: start,
        explored: HashSet::new(),
        steps: 0,
        has_cheated: false,
        cheat_pos_1: None,
        cheat_pos_2: None,
        kind: ActionKind::Start,
    }]);
    while let Some(Action {
        pos,
        explored,
        steps,
        has_cheated,
        cheat_pos_1,
        cheat_pos_2,
        kind,
    }) = queue.pop_front()
    {
        if cheat_pos_2
            .is_some_and(|cheat_pos_2| pos == cheat_pos_2 && cache.contains_key(&cheat_pos_2))
        {
            println!("cached");
            escapes.push(cache.get(&cheat_pos_2.unwrap()).unwrap().clone());
        } else if pos == end {
            let end_action = Action {
                pos,
                explored,
                steps,
                has_cheated,
                cheat_pos_1,
                cheat_pos_2,
                kind: ActionKind::Escape,
            };
            cache.insert(cheat_pos_2.unwrap(), end_action.clone());
            escapes.push(end_action);
            continue;
        } else if steps + 2 == score_to_beat {
            println!("discarding");
            continue;
        }
        for mov in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let adj_pos = (pos.0 + mov.0, pos.1 + mov.1);
            let mut explored = explored.clone();
            if explored.contains(&adj_pos) {
                continue;
            }
            explored.insert(adj_pos);
            match kind {
                ActionKind::Start | ActionKind::Move => {
                    if let Some('.' | 'E') = map.get(&adj_pos) {
                        queue.push_back(Action {
                            pos: adj_pos,
                            explored,
                            steps: steps + 1,
                            has_cheated,
                            cheat_pos_1,
                            cheat_pos_2,
                            kind: ActionKind::Move,
                        });
                    } else if map.get(&adj_pos).is_some_and(|c| c == &'#' && !has_cheated) {
                        queue.push_back(Action {
                            pos: adj_pos,
                            explored,
                            steps: steps + 1,
                            has_cheated: true,
                            cheat_pos_1: Some(adj_pos),
                            cheat_pos_2,
                            kind: ActionKind::Cheat,
                        });
                    }
                }
                ActionKind::Cheat => {
                    if map.get(&adj_pos) != Some(&'#') {
                        queue.push_back(Action {
                            pos: adj_pos,
                            explored,
                            steps: steps + 1,
                            has_cheated,
                            cheat_pos_1,
                            cheat_pos_2: Some(adj_pos),
                            kind: ActionKind::Move,
                        });
                    }
                }
                _ => panic!("Oops"),
            }
        }
    }
    escapes
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    let (mut start, mut end) = (None, None);
    (
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some((x as isize, y as isize))
                        };
                        if c == 'E' {
                            end = Some((x as isize, y as isize))
                        };
                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        start.unwrap(),
        end.unwrap(),
        input.trim().lines().count(),
    )
}
