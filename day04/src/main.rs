const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Vec<char>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(test_input));
    println!("         validation    {} ", part_1(input));
    // println!("Part 2   test          {} ", part_2(test_input));
    // println!("         validation    {} ", part_2(input));
}

fn part_1(input: Input) -> u32 {
    let mut total = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            for [dx, dy] in [
                [1, 0],
                [1, -1],
                [0, -1],
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, 1],
                [1, 1],
            ] {
                if ['X', 'M', 'A', 'S']
                    .iter()
                    .enumerate()
                    .all(|(position, expected_char)| {
                        input
                            .get((y as i32 + (dy * position as i32)) as usize)
                            .and_then(|row| {
                                row.get((x as i32 + (dx * position as i32)) as usize)
                                    .filter(|&char| char == expected_char)
                            })
                            .is_some()
                    })
                {
                    total += 1;
                }
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
