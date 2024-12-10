const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type DiskLayout = Vec<Block>;

#[derive(Copy, Clone, Debug)]
enum Block {
    File(usize, usize),
    Free(usize),
}

fn main() {
    let (small_test_layout, big_test_layout) = parse(INPUT_TEST);
    let (small_layout, big_layout) = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&small_test_layout));
    println!("         validation    {} ", part_1(&small_layout));
    println!("Part 2   test          {} ", part_2(&big_test_layout));
    println!("         validation    {} ", part_2(&big_layout));
}

fn part_1(input: &DiskLayout) -> usize {
    let mut blocks = input.clone();
    let mut head = 0;
    let mut tail = input.len() - 1;
    loop {
        if head >= tail {
            break;
        }
        match (blocks[head], blocks[tail]) {
            (Block::Free(_), Block::File(_, _)) => {
                blocks.swap(head, tail);
                head += 1;
                tail -= 1;
            }
            (Block::Free(_), Block::Free(_)) => {
                tail -= 1;
            }
            (Block::File(_, _), _) => {
                head += 1;
            }
        }
    }
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Block::File(id, _) => Some(i * id),
            _ => None,
        })
        .sum()
}

fn part_2(input: &DiskLayout) -> usize {
    let mut blocks = input.clone();
    let mut tail = input.len() - 1;
    while tail > 1 {
        match blocks[tail] {
            Block::File(_, file_size) => {
                if let Some((head, free_size)) = blocks
                    .iter()
                    .enumerate()
                    .filter_map(|(i, b)| match b {
                        Block::Free(size) => Some((i, *size)),
                        Block::File(_, _) => None,
                    })
                    .find(|(i, free_size)| i < &tail && free_size >= &file_size)
                {
                    blocks[head] = blocks[tail];
                    blocks[tail] = Block::Free(file_size);
                    blocks.insert(head + 1, Block::Free(free_size - file_size));
                }
                tail -= 1;
            }
            Block::Free(_) => {
                tail -= 1;
            }
        }
    }
    blocks
        .iter()
        .flat_map(|b| match b {
            Block::File(id, size) => vec![Some(id); *size],
            Block::Free(size) => vec![None; *size],
        })
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b * i))
        .sum()
}

fn parse(input: &str) -> (DiskLayout, DiskLayout) {
    let (mut small_blocks, mut big_blocks) = (vec![], vec![]);
    input.trim().chars().enumerate().for_each(|(i, b)| {
        if i % 2 == 0 {
            small_blocks.extend(vec![
                Block::File(i / 2, 1);
                b.to_digit(10).unwrap() as usize
            ]);
            big_blocks.push(Block::File(i / 2, b.to_digit(10).unwrap() as usize));
        } else {
            small_blocks.extend(vec![Block::Free(1); b.to_digit(10).unwrap() as usize]);
            big_blocks.push(Block::Free(b.to_digit(10).unwrap() as usize));
        }
    });
    (small_blocks, big_blocks)
}
