use core::panic;
use std::{collections::HashMap, time::Instant};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (Map, (isize, isize), Position, Vec<Caisse>, Vec<Move>);
type Map = HashMap<Position, char>;
type Position = (isize, isize);
type Move = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Caisse {
    Single(Position),
    Double(Position, Position),
}
impl Caisse {
    fn new(coords: Vec<Position>) -> Self {
        match coords.len() {
            1 => Self::Single(coords[0]),
            2 => Self::Double(coords[0], coords[1]),
            _ => panic!("Nope"),
        }
    }

    fn collides_with_pos(&self, other_pos: &Position) -> bool {
        match self {
            Self::Single(pos) => pos == other_pos,
            Self::Double(pos1, pos2) => pos1 == other_pos || pos2 == other_pos,
        }
    }

    fn collides_with(&self, other_caisse: &Caisse) -> bool {
        other_caisse
            .coords()
            .iter()
            .any(|pos| self.collides_with_pos(pos))
    }

    fn coords(&self) -> Vec<Position> {
        match self {
            Self::Single(pos) => vec![*pos],
            Self::Double(pos1, pos2) => vec![*pos1, *pos2],
        }
    }

    fn project(&self, mov: Move) -> Self {
        match self {
            Self::Single(pos) => Self::new(vec![(pos.0 + mov.0, pos.1 + mov.1)]),
            Self::Double(pos1, pos2) => Self::new(vec![
                (pos1.0 + mov.0, pos1.1 + mov.1),
                (pos2.0 + mov.0, pos2.1 + mov.1),
            ]),
        }
    }

    fn shift(&mut self, mov: Move) {
        *self = self.project(mov);
    }

    fn can_be_pushed_by(&self, other_caisses: &[Caisse], mov: Move) -> bool {
        other_caisses
            .iter()
            .any(|caisse| caisse.project(mov).collides_with(self))
    }

    fn render(&self, pos: &Position) -> char {
        if !self.collides_with_pos(pos) {
            panic!("Oops");
        }
        match self {
            Self::Single(_) => 'O',
            Self::Double(pos1, _) => {
                if pos == pos1 {
                    '['
                } else {
                    ']'
                }
            }
        }
    }

    fn score(&self) -> usize {
        (match self {
            Self::Single((x, y)) => x + 100 * y,
            Self::Double((x1, y1), (x2, y2)) => {
                if y1 != y2 {
                    panic!("Oops");
                };
                x1.min(x2) + 100 * y1
            }
        }) as usize
    }
}

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1, false);
    let test_input_2 = parse(INPUT_TEST_2, false);
    let test_input_3 = parse(INPUT_TEST_1, true);
    let input = parse(INPUT, false);
    let input_2 = parse(INPUT, true);
    println!("Part 1   test (simple) {} ", move_caisses(&test_input_2));
    println!("Part 1   test          {} ", move_caisses(&test_input_1));
    println!("         validation    {} ", move_caisses(&input));
    println!("Part 2   test          {} ", move_caisses(&test_input_3));
    println!("         validation    {} ", move_caisses(&input_2));
    println!("Duration: {:?}", start.elapsed());
}

fn move_caisses(input: &Input) -> usize {
    let (map, _size, mut pos, caisses, moves) = input;
    let mut caisses = caisses.clone();
    let mut map = map.clone();
    for mov in moves {
        step(&mut map, &mut caisses, &mut pos, *mov);
    }
    // 1360570 is too low, 1384465 is too high, it's not 1373098 :(
    caisses.into_iter().map(|caisse| caisse.score()).sum()
    //  map.into_iter()
    //      .filter_map(|((x, y), c)| {
    //          if c == '.'
    //              && caisses
    //                  .iter()
    //                  .any(|caisse| caisse.coords().first().unwrap() == &(x, y))
    //          {
    //              Some((x + 100 * y) as usize)
    //          } else {
    //              None
    //          }
    //      })
    //      .sum()
}

fn step(map: &mut Map, caisses: &mut [Caisse], pos: &mut Position, mov: Move) {
    let next_pos = (pos.0 + mov.0, pos.1 + mov.1);
    match map.get(&next_pos).unwrap() {
        '#' => {}
        '.' => {
            if let Some(caisse) = caisses
                .iter()
                .find(|caisse| caisse.collides_with_pos(&next_pos))
            {
                // Maybe this caisse is pushing other caisses
                let mut pushed_caisses = vec![*caisse];
                let mut pushing_caisses = vec![*caisse];
                loop {
                    let next_pushed_caisses = caisses
                        .iter()
                        .filter(|caisse| {
                            !pushed_caisses.contains(caisse)
                                && caisse.can_be_pushed_by(&pushing_caisses, mov)
                        })
                        .copied()
                        .collect::<Vec<_>>();
                    if next_pushed_caisses.is_empty() {
                        break;
                    }
                    pushed_caisses.extend(next_pushed_caisses.clone());
                    pushing_caisses = next_pushed_caisses;
                }
                // Let's see if all those caisses can move
                let caisses_can_move = !pushing_caisses.iter().any(|caisse| {
                    caisse
                        .project(mov)
                        .coords()
                        .iter()
                        .any(|pos| map.get(pos).unwrap() == &'#')
                });
                if caisses_can_move {
                    caisses
                        .iter_mut()
                        .filter(|caisse| pushed_caisses.contains(caisse))
                        .for_each(|caisse| caisse.shift(mov));
                    *pos = next_pos;
                }
            } else {
                *pos = next_pos;
            }
        }
        _ => panic!("Oops"),
    }
}

fn parse(input: &str, double: bool) -> Input {
    let (map_data, moves_data) = input.trim().split_once("\n\n").unwrap();
    let mut caisses = vec![];
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
                .map(|(x, c)| {
                    let map_c = match c {
                        '[' => {
                            caisses.push(Caisse::new(vec![
                                (x as isize, y as isize),
                                (x as isize + 1, y as isize),
                            ]));
                            '.'
                        }
                        ']' => '.',
                        'O' => {
                            caisses.push(Caisse::new(vec![(x as isize, y as isize)]));
                            '.'
                        }
                        _ => c,
                    };
                    ((x as isize, y as isize), map_c)
                })
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
        (map_line_count * if double { 2 } else { 1 }, map_line_count),
        start,
        caisses,
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
fn print_map(map: &Map, caisses: &[Caisse], current_pos: Position, size: (isize, isize)) {
    for (pos, c) in map {}
    for y in 0..size.1 {
        for x in 0..size.0 {
            print!(
                "{}",
                if (x, y) == current_pos {
                    '@'
                } else if let Some(caisse) = caisses
                    .iter()
                    .find(|caisse| caisse.collides_with_pos(&(x, y)))
                {
                    caisse.render(&(x, y))
                } else {
                    *map.get(&(x, y)).unwrap()
                }
            );
        }
        println!();
    }
    println!();
}
