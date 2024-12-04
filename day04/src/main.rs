const INPUT_TEST: &str = include_str!("../input_test.txt");
// const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Vec<char>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    // let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(test_input));
    // println!("         validation    {} ", part_1(input));
    // println!("Part 2   test          {} ", part_2(test_input));
    // println!("         validation    {} ", part_2(input));
}

fn part_1(input: Input) -> u32 {
    let mut total = 0;
    let x_pos = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    'X' => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for (x, y) in x_pos {
        for [dx, dy] in [[1i32, 0i32], [0, -1], [-1, 0], [0, 1]] {
            if x as i32 + 3 * dx > 0
                && y as i32 + 3 * dy > 0
                && ['M', 'A', 'S'].iter().enumerate().all(|(index, letter)| {
                    input
                        .get((y as i32 + dy) as usize * index)
                        .map(|row| {
                            row.get((x as i32 + dx) as usize * index)
                                .map(|char| char == letter)
                        })
                        .is_some()
                })
            {
                dbg!((x, y, dx, dy));
                total += 1;
            }
        }
    }
    total
}

// fn part_2(input: Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
