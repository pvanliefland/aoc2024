const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Option<usize>>;
type Input2 = Vec<Chunk>;

#[derive(Copy, Clone, Debug)]
enum Chunk {
    File(usize, usize),
    Free(usize),
}

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let test_input_2 = parse_2(INPUT_TEST);
    let input_2 = parse_2(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    println!("         validation    {} ", part_2(&input_2));
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
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b * i))
        .sum()
}

fn part_2(input: &Input2) -> usize {
    let mut blocks = input.clone();
    let mut tail = input.len() - 1;
    while tail > 1 {
        match blocks[tail] {
            Chunk::File(_, file_size) => {
                if let Some((head, free_size)) = blocks
                    .iter()
                    .enumerate()
                    .filter_map(|(i, b)| match b {
                        Chunk::Free(size) => Some((i, *size)),
                        Chunk::File(_, _) => None,
                    })
                    .find(|(i, free_size)| i < &tail && free_size >= &file_size)
                {
                    let (free1, free2) =
                        (Chunk::Free(free_size - file_size), Chunk::Free(file_size));
                    blocks.remove(head);
                    blocks.insert(head, blocks[tail - 1]);
                    blocks.remove(tail);
                    blocks.insert(head + 1, free1);
                    blocks.insert(tail, free2);
                }
                tail -= 1;
            }
            Chunk::Free(_) => {
                tail -= 1;
            }
        }
    }
    blocks
        .iter()
        .flat_map(|b| match b {
            Chunk::File(id, size) => vec![Some(id); *size],
            Chunk::Free(size) => vec![None; *size],
        })
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b * i))
        .sum()
}

#[allow(unused)]
fn debug(blocks: &[Chunk]) {
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
