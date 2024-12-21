use std::{collections::HashSet, iter::Product, time::Instant};

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
                .map(|moves| { moves.iter().collect::<String>() })
                .collect::<Vec<_>>()
                .join("\n")
        );
    });
    42
}

fn cheapest_moves(sequence: &[char; 4], position_function: fn(char) -> Position) -> Vec<Vec<char>> {
    let mut moves = vec![vec![]; 4];
    let a_pos = position_function('A');
    let mut pos = a_pos;
    for (i, c) in sequence.iter().enumerate() {
        let mut c_moves = vec![];
        let c_pos = position_function(*c);
        let (dx1, dy1) = (pos.0 - c_pos.0, pos.1 - c_pos.1);
        c_moves.extend(vec![if dx1 > 0 { '<' } else { '>' }; dx1.unsigned_abs()]);
        c_moves.extend(vec![if dy1 > 0 { '^' } else { 'v' }; dy1.unsigned_abs()]);
        pos = c_pos;
        permutations(c_moves).into_iter().for_each(|perm| {
            moves[i].push([perm, vec!['A']].concat());
        });
    }
    dbg!(&moves);
    let yolo = product(moves);
    dbg!(&yolo
        .iter()
        .map(|yo| yo.iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>());
    vec![vec![]]
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

fn permutations(seq: Vec<char>) -> Vec<Vec<char>> {
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
    // def product(*iterables, repeat=1):
    //   # product('ABCD', 'xy') → Ax Ay Bx By Cx Cy Dx Dy
    //   # product(range(2), repeat=3) → 000 001 010 011 100 101 110 111

    //   if repeat < 0:
    //       raise ValueError('repeat argument cannot be negative')
    //   pools = [tuple(pool) for pool in iterables] * repeat

    //   result = [[]]
    //   for pool in pools:
    //       result = [x+[y] for x in result for y in pool]

    //   for prod in result:
    //       yield tuple(prod)
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
