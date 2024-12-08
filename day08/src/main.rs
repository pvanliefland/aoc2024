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
    let mut antinodes = HashSet::new();
    for (freq, positions) in grid {
        if freq == &'.' {
            continue;
        }
        let combinations = combinations(positions, 2);
        for combination in combinations {
            let (p1, p2) = (combination[0], combination[1]);
            let (dx, dy) = ((p2.0 - p1.0), (p2.1 - p1.1));
            let antinode_1 = (p1.0 + 2 * dx, p1.1 + 2 * dy);
            let antinode_2 = (p2.0 - 2 * dx, p2.1 - 2 * dy);

            if antinode_1.0 >= 0
                && antinode_1.0 < *size
                && antinode_1.1 >= 0
                && antinode_1.1 < *size
            {
                antinodes.insert(antinode_1);
            }
            if antinode_2.0 >= 0
                && antinode_2.0 < *size
                && antinode_2.1 >= 0
                && antinode_2.1 < *size
            {
                antinodes.insert(antinode_2);
            }
        }
    }

    antinodes.len()
}

fn part_2(input: &Input) -> usize {
    let (grid, size) = input;
    let mut antinodes = HashSet::new();
    for (freq, positions) in grid {
        if freq == &'.' {
            continue;
        }
        let combinations = combinations(positions, 2);
        for combination in combinations {
            let (p1, p2) = (combination[0], combination[1]);
            let (dx, dy) = ((p2.0 - p1.0), (p2.1 - p1.1));

            #[allow(clippy::maybe_infinite_iter)]
            (0..)
                .map(|step| ((p1.0 + step * dx), (p1.1 + step * dy)))
                .take_while(|&(x, y)| x >= 0 && x < *size && y >= 0 && y < *size)
                .for_each(|antinode| {
                    antinodes.insert(antinode);
                });
            #[allow(clippy::maybe_infinite_iter)]
            (0..)
                .map(|step| ((p1.0 - step * dx), (p1.1 - step * dy)))
                .take_while(|&(x, y)| x >= 0 && x < *size && y >= 0 && y < *size)
                .for_each(|antinode| {
                    antinodes.insert(antinode);
                });
        }
    }

    antinodes.len()
}

fn combinations<T: Copy + Clone + Eq>(items: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 1 {
        items.iter().map(|item| vec![*item]).collect()
    } else {
        let mut visited = vec![];
        items
            .iter()
            .flat_map(|&item| {
                visited.push(item);
                combinations(
                    &items
                        .iter()
                        .filter(|&other_item| *other_item != item && !visited.contains(other_item))
                        .copied()
                        .collect::<Vec<_>>(),
                    size - 1,
                )
                .into_iter()
                .map(move |perms| [vec![item], perms].concat())
            })
            .collect()
    }
}

fn parse(input: &str) -> Input {
    let mut grid: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.entry(c)
                .and_modify(|positions| {
                    positions.push((x as isize, y as isize));
                })
                .or_insert(vec![(x as isize, y as isize)]);
        });
    });
    (grid, input.trim().lines().count() as isize)
}
