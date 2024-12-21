use std::{collections::HashSet, time::Instant};

// const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = [[char; 4]; 5];
type Position = (isize, isize);

fn main() {
    let start = Instant::now();
    // let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    // println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    input
        .iter()
        .map(|seq_1| {
            let moves_1 = compute_moves(seq_1.to_vec(), numeric_keypad_position, true);
            let cheapest_len_1 = moves_1
                .iter()
                .min_by_key(|seq_1| seq_1.len())
                .unwrap()
                .len();
            let cheapest_moves_1 = moves_1
                .into_iter()
                .filter(|seq_1| seq_1.len() == cheapest_len_1)
                .collect::<Vec<_>>();
            let moves_2 = cheapest_moves_1
                .into_iter()
                .flat_map(|seq_2| compute_moves(seq_2, directional_keypad_position, true))
                .collect::<Vec<_>>();
            let cheapest_len_2 = moves_2
                .iter()
                .min_by_key(|seq_2| seq_2.len())
                .unwrap()
                .len();
            let cheapest_moves_2 = moves_2
                .into_iter()
                .filter(|seq_2| seq_2.len() == cheapest_len_2)
                .collect::<Vec<_>>();
            let moves_3 = cheapest_moves_2
                .into_iter()
                .flat_map(|seq_3| compute_moves(seq_3, directional_keypad_position, false))
                .collect::<Vec<_>>();
            let cheapest_move_3 = moves_3.iter().min_by_key(|seq_3| seq_3.len()).unwrap();
            let numeric_value = seq_1[0..3]
                .iter()
                .collect::<String>()
                .replace("A", "")
                .parse::<usize>()
                .unwrap();
            cheapest_move_3.len() * numeric_value
        })
        .sum()
}

fn compute_moves(
    sequence: Vec<char>,
    position_function: fn(char) -> Position,
    check_segfault: bool,
) -> Vec<Vec<char>> {
    let mut moves = vec![vec![]; sequence.len()];
    let a_pos = position_function('A');
    let mut pos = a_pos;
    for (i, c) in sequence.iter().enumerate() {
        let mut c_moves = vec![];
        let c_pos = position_function(*c);
        let (dx1, dy1) = (pos.0 - c_pos.0, pos.1 - c_pos.1);
        c_moves.extend(vec![
            if dx1 > 0 { (-1, 0) } else { (1, 0) };
            dx1.unsigned_abs()
        ]);
        c_moves.extend(vec![
            if dy1 > 0 { (0, -1) } else { (0, 1) };
            dy1.unsigned_abs()
        ]);
        if c_moves.is_empty() {
            moves[i].push(vec!['A']);
        } else {
            permutations(c_moves).into_iter().for_each(|perm| {
                let mut check_pos = pos;
                let mut segfault = false;
                for mov in &perm {
                    check_pos = (check_pos.0 + mov.0, check_pos.1 + mov.1);
                    if check_segfault && check_pos == position_function('X') {
                        segfault = true
                    }
                }
                if !segfault {
                    let perm_c = perm
                        .iter()
                        .map(|pos| match pos {
                            (1, 0) => '>',
                            (0, 1) => 'v',
                            (-1, 0) => '<',
                            (0, -1) => '^',
                            _ => panic!("Oops"),
                        })
                        .collect::<Vec<_>>();
                    moves[i].push([perm_c, vec!['A']].concat());
                }
            });
        }
        pos = c_pos;
    }
    product(moves)
        .into_iter()
        .map(|combo| combo.into_iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn numeric_keypad_position(c: char) -> Position {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        'X' => (0, 3),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("Oops"),
    }
}

fn directional_keypad_position(c: char) -> Position {
    match c {
        'X' => (0, 0),
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("Oops"),
    }
}

fn permutations(seq: Vec<Position>) -> Vec<Vec<Position>> {
    if seq.len() == 1 {
        return vec![seq];
    }
    seq.iter()
        .enumerate()
        .flat_map(|(i, c)| {
            let mut others = seq.clone();
            others.remove(i);
            permutations(others)
                .into_iter()
                .map(|perm| [vec![*c], perm].concat())
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn product(seqs: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    if seqs.len() == 1 {
        return seqs[0].iter().map(|left| vec![left.clone()]).collect();
    }
    let (left, right) = seqs.split_at(1);
    let right_product = product(right.to_vec());
    left[0]
        .iter()
        .flat_map(|left_seq| {
            right_product
                .clone()
                .iter()
                .map(|right_seq| [vec![left_seq.clone()], right_seq.clone()].concat())
                .collect::<Vec<_>>()
        })
        .collect()
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
