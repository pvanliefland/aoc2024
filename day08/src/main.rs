use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (HashMap<char, Vec<(isize, isize)>>, isize);

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let (grid, size) = input;
    count_antinodes(grid, *size, Some(2))
}

fn part_2(input: &Input) -> usize {
    let (grid, size) = input;
    count_antinodes(grid, *size, None)
}

fn count_antinodes(
    grid: &HashMap<char, Vec<(isize, isize)>>,
    size: isize,
    distance: Option<isize>,
) -> usize {
    let mut antinodes = HashSet::new();
    for positions in grid.values() {
        let combinations = combinations(positions, 2);
        for combination in combinations {
            let (p1, p2) = (combination[0], combination[1]);
            let (dx, dy) = ((p2.0 - p1.0), (p2.1 - p1.1));

            for (pos, dir) in [(p1, 1), (p2, -1)] {
                let range = if let Some(distance) = distance {
                    distance..=distance
                } else {
                    0..=isize::MAX
                };
                range
                    .map(|step| ((pos.0 + step * dx * dir), (pos.1 + step * dy * dir)))
                    .take_while(|&(x, y)| x >= 0 && x < size && y >= 0 && y < size)
                    .for_each(|antinode| {
                        antinodes.insert(antinode);
                    });
            }
        }
    }
    antinodes.len()
}

fn combinations<T: Copy + Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 1 {
        items.iter().map(|item| vec![*item]).collect()
    } else {
        let mut result = vec![];
        let mut items = items.to_vec();
        while !items.is_empty() {
            let item = items.remove(0);
            for combination in combinations(&items, size - 1) {
                result.push([vec![item], combination].concat());
            }
        }
        result
    }
}

fn parse(input: &str) -> Input {
    let mut grid: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .for_each(|(x, c)| {
                grid.entry(c)
                    .and_modify(|positions| {
                        positions.push((x as isize, y as isize));
                    })
                    .or_insert(vec![(x as isize, y as isize)]);
            });
    });
    (grid, input.trim().lines().count() as isize)
}
