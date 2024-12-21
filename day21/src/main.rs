use std::time::Instant;

const INPUT_TEST: &str = include_str!("../input_test.txt");
// const INPUT: &str = include_str!("../input.txt");

type Input = [[char; 4]; 5];
type Position = (isize, isize);

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    // let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    // println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    input.iter().take(1).for_each(|seq| {
        println!(
            "{}",
            cheapest_moves(seq, numeric_keypad_position)
                .iter()
                .collect::<String>()
        );
    });
    42
}

fn cheapest_moves(sequence: &[char; 4], position_function: fn(char) -> Position) -> Vec<char> {
    let mut moves = vec![];
    let a_pos = position_function('A');
    let mut pos = a_pos;
    for c in sequence {
        let c_pos = position_function(*c);
        let (dx1, dy1) = (pos.0 - c_pos.0, pos.1 - c_pos.1);
        moves.extend(vec![if dx1 > 0 { '<' } else { '>' }; dx1.unsigned_abs()]);
        moves.extend(vec![if dy1 > 0 { '^' } else { 'v' }; dy1.unsigned_abs()]);
        moves.push('A');
        pos = c_pos;
    }
    moves
}

fn numeric_keypad_position(c: char) -> Position {
    match c {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!("Oops"),
    }
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|code| code.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
