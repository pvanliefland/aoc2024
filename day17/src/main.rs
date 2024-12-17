use core::panic;
use std::time::Instant;

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (u128, u128, u128, u128);

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input_1));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> String {
    let (a, b, c, program) = input;
    run(*a, *b, *c, program, false)
        .to_le_bytes()
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_2(input: &Input) -> u128 {
    let (_a, b, c, program) = input;
    let mut a = 0;

    loop {
        let candidate_output = run(a, *b, *c, program, true);
        if candidate_output == *program {
            break a;
        }
        if a % 1000000 == 0 {
            println!("{a}");
        }
        a += 1
    }
}

fn run(a: u128, b: u128, c: u128, program: &u128, quine: bool) -> u128 {
    let (mut a, mut b, mut c) = (a, b, c);
    let mut p = 0;
    let mut o = 0;
    let mut out = 0u128;
    loop {
        if p > 14 {
            break;
        }
        let (ocs, ops) = (p * 8, (p + 1) * 8);
        let (oc, op) = (program >> ocs & 0xff, program >> ops & 0xff);
        let cop = match op {
            l if l <= 3 => l,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Oops"),
        };
        match oc {
            0 => {
                a /= 2u128.pow(cop as u32);
            }
            1 => {
                b ^= op;
            }
            2 => {
                b = cop % 8;
            }
            3 => {
                if a != 0 {
                    p = op;
                    continue;
                }
            }
            4 => b ^= c,
            5 => {
                let nout = cop % 8;
                if quine && nout != (program >> (o * 8) & 0xff) {
                    break;
                }
                out += nout << (o * 8);
                o += 1;
            }
            6 => {
                b = a / 2u128.pow(cop as u32);
            }
            7 => {
                c = a / 2u128.pow(cop as u32);
            }
            _ => panic!("Oops"),
        }
        p += 2;
    }
    out
}

fn parse(input: &str) -> Input {
    let (register_data, program_data) = input.split_once("\n\n").unwrap();
    let registers = register_data
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect::<Vec<_>>();
    let mut program = program_data
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<_>>();
    program.resize(16, 0);
    (
        registers[0],
        registers[1],
        registers[2],
        u128::from_le_bytes(program.try_into().unwrap()),
    )
}
