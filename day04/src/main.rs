const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Vec<char>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .map(|(x, y, _)| {
            [[1, 0], [1, -1], [0, -1], [-1, -1]]
                .iter()
                .filter(move |[dx, dy]| check(input, (x, y), (*dx, *dy), &["XMAS", "SAMX"]))
                .count()
        })
        .sum()
}

fn part_2(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .filter(|&(x, y, c)| {
            c == &'A'
                && check(input, (x - 1, y + 1), (1, -1), &["MAS", "SAM"])
                && check(input, (x - 1, y - 1), (1, 1), &["MAS", "SAM"])
        })
        .count()
}

fn check(input: &Input, xy: (i32, i32), dxdy: (i32, i32), patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| {
        pattern.chars().enumerate().all(|(index, expected)| {
            input
                .get((xy.1 + (dxdy.1 * index as i32)) as usize)
                .and_then(|row| {
                    row.get((xy.0 + (dxdy.0 * index as i32)) as usize)
                        .filter(|&actual| actual == &expected)
                })
                .is_some()
        })
    })
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
