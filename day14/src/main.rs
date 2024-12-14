use std::{collections::HashMap, time::Instant};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
struct Robot {
    p: Position,
    v: Velocity,
}
type Position = (isize, isize);
type Velocity = (isize, isize);

fn main() {
    let start = Instant::now();
    let (test_input, tw, th) = (parse(INPUT_TEST), 11, 7);
    let (input, w, h) = (parse(INPUT), 101, 103);
    println!("Part 1   test          {} ", part_1(&test_input, tw, th));
    println!("         validation    {} ", part_1(&input, w, h));
    // println!("Part 2   test          {:?} ", part_2(&test_input, w, h));
    println!("         validation    {} ", part_2(&input, w, h).unwrap());
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(robots: &[Robot], width: isize, height: isize) -> usize {
    let mut robots = robots.to_owned();
    for _ in 0..100 {
        step(&mut robots, width, height);
    }
    let (qw, qh) = ((width - 1) / 2, (height - 1) / 2);
    [
        ((0, 0), qw, qh),
        ((qw + 1, 0), qw, qh),
        ((0, qh + 1), qw, qh),
        ((qw + 1, qh + 1), qw, qh),
    ]
    .iter()
    .map(|q| {
        map_robots(
            &robots
                .iter()
                .filter(|r| {
                    ((q.0 .0)..(q.0 .0 + q.1)).contains(&r.p.0)
                        && ((q.0 .1)..(q.0 .1 + q.2)).contains(&r.p.1)
                })
                .copied()
                .collect::<Vec<_>>(),
        )
        .values()
        .sum::<usize>()
    })
    .product()
}

fn part_2(robots: &[Robot], width: isize, height: isize) -> Option<usize> {
    let mut robots = robots.to_owned();
    for i in 1..10000 {
        step(&mut robots, width, height);
        let map = map_robots(&robots);
        if map
            .iter()
            .filter(|(p, _)| {
                map.contains_key(&(p.0 - 1, p.1 + 1))
                    && map.contains_key(&(p.0 - 2, p.1 + 2))
                    && map.contains_key(&(p.0 - 3, p.1 + 3))
            })
            .count()
            > 10
        {
            // print_map(&map, width, height);
            return Some(i);
        }
    }
    None
}

fn map_robots(robots: &Vec<Robot>) -> HashMap<Position, usize> {
    let mut map = HashMap::new();
    for robot in robots {
        map.entry(robot.p)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    map
}

fn step(robots: &mut Vec<Robot>, width: isize, height: isize) {
    for robot in robots {
        robot.p = (
            match robot.p.0 + robot.v.0 {
                x if x < 0 => x + width,
                x if x >= width => x - width,
                x => x,
            },
            match robot.p.1 + robot.v.1 {
                y if y < 0 => y + height,
                y if y >= height => y - height,
                y => y,
            },
        );
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (x, y) = p.split_once("=").unwrap().1.split_once(",").unwrap();
            let (dx, dy) = v.split_once("=").unwrap().1.split_once(",").unwrap();
            Robot {
                p: (x.parse().unwrap(), y.parse().unwrap()),
                v: (dx.parse().unwrap(), dy.parse().unwrap()),
            }
        })
        .collect()
}

#[allow(unused)]
fn print_map(map: &HashMap<Position, usize>, width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                map.get(&(x, y))
                    .map(|_| "\x1b[93mX\x1b[0m".to_string())
                    .unwrap_or(".".to_string())
            );
        }
        println!();
    }
    println!();
}
