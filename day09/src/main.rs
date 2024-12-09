const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Option<usize>>;
type Input2 = Vec<Chunk>;

#[derive(Copy, Clone)]
enum Chunk {
    File(usize, usize),
    Free(usize),
}

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let test_input_2 = parse_2(INPUT_TEST);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    // println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut blocks = input.clone();
    let mut head = 0;
    let mut tail = input.len() - 1;
    loop {
        if head >= tail {
            break;
        } else if blocks[head].is_none() && blocks[tail].is_some() {
            blocks.swap(head, tail);
            head += 1;
            tail -= 1;
        } else if blocks[head].is_none() {
            tail -= 1;
        } else if blocks[head].is_some() {
            head += 1;
        }
    }
    // yolo(&blocks);
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b * i))
        .sum()
}

fn part_2(input: &Input2) -> usize {
    let mut blocks = input.clone();
    let mut head = 0;
    let mut tail = input.len() - 1;
    println!("{}-{}", head, tail);
    loop {
        if tail <= head {
            break;
        }
        match (blocks[tail], blocks[head]) {
            (Chunk::File(_, s1), Chunk::Free(s2)) if s1 <= s2 => {
                blocks.swap(head, tail);
                tail -= 1;
                head += 1;
                if s1 - s2 > 0 {
                    let new = Chunk::Free(s2 - s1);
                    blocks.insert(head, new);
                    head += 1;
                }
            }
            (Chunk::File(_, _), _) => {
                head += 1;
            }
            (Chunk::Free(_), _) => {
                tail -= 1;
            }
        }
        yolo(&blocks);
    }
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Chunk::File(id, _) => Some(i * id),
            Chunk::Free(_) => None,
        })
        .sum()
}

fn yolo(blocks: &[Chunk]) {
    println!(
        "{}",
        blocks
            .iter()
            .map(|b| match b {
                Chunk::File(id, size) =>
                    vec![id.to_string(); *size].into_iter().collect::<String>(),
                Chunk::Free(size) => vec![".".to_string(); *size].into_iter().collect::<String>(),
            })
            .collect::<String>()
    );
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c.to_digit(10).unwrap() as usize]
            } else {
                vec![None; c.to_digit(10).unwrap() as usize]
            }
        })
        .collect()
}
fn parse_2(input: &str) -> Input2 {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Chunk::File(i / 2, c.to_digit(10).unwrap() as usize)
            } else {
                Chunk::Free(c.to_digit(10).unwrap() as usize)
            }
        })
        .collect()
}
