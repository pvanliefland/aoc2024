use std::{collections::HashMap, time::Instant};

const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input<'w> = (InitialValues<'w>, Vec<Gate<'w>>);
type InitialValues<'v> = HashMap<&'v str, u8>;
type Gate<'g> = (&'g str, &'g str, &'g str, &'g str);

fn main() {
    let start = Instant::now();
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input_1));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    bits_to_int(&compute(input))
}

fn part_2(input: &Input) -> usize {
    let (initial_values, gates) = input;
    let [x, y]: [usize; 2] = ["x", "y"]
        .iter()
        .map(|prefix| {
            let mut wires = initial_values
                .iter()
                .filter(|(w, _)| w.starts_with(prefix))
                .collect::<Vec<_>>();
            wires.sort_by_key(|(w, _)| *w);
            bits_to_int(&wires.into_iter().map(|(_, v)| *v).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let expected_bits = format!("{:#b}", x + y)
        .chars()
        .skip(2)
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let _expected_z_values = expected_bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (format!("z{:0>2}", i), b))
        .collect::<HashMap<_, _>>();
    let _bits = compute(input);
    for (xw, yw, op, ow) in gates.iter().filter(|(xw, _, _, _)| xw.starts_with("x")) {
        println!("{xw} {op} {yw} -> {ow}");
    }
    42
}

fn compute(input: &Input) -> [u8; 64] {
    let (initial_values, gates) = input;
    let mut bits = [0u8; 64];
    gates
        .iter()
        .filter(|(_, _, _, o)| o.starts_with("z"))
        .for_each(|gate| {
            bits[gate.3.replace("z", "").parse::<usize>().unwrap()] =
                compute_gate_value(*gate, gates, initial_values);
        });
    bits
}

fn compute_gate_value(gate: Gate, all_gates: &Vec<Gate>, initial_values: &InitialValues) -> u8 {
    let (w1, op, w2, _) = gate;
    let v1 = initial_values.get(w1).copied().unwrap_or_else(|| {
        let other_gate = all_gates.iter().find(|gate| gate.3 == w1).unwrap();
        compute_gate_value(*other_gate, all_gates, initial_values)
    });
    let v2 = initial_values.get(w2).copied().unwrap_or_else(|| {
        let other_gate = all_gates.iter().find(|gate| gate.3 == w2).unwrap();
        compute_gate_value(*other_gate, all_gates, initial_values)
    });
    match op {
        "AND" => v1 & v2,
        "OR" => v1 | v2,
        "XOR" => v1 ^ v2,
        _ => panic!("Oops"),
    }
}

fn bits_to_int(bits: &[u8]) -> usize {
    bits.iter()
        .rev()
        .fold(0, |acc, bit| (acc << 1) + *bit as usize)
}

fn parse(input: &str) -> Input {
    let (initial, gates) = input.split_once("\n\n").unwrap();

    (
        initial
            .lines()
            .map(|line| {
                let (wire, initial_value) = line.split_once(": ").unwrap();
                (wire, initial_value.parse().unwrap())
            })
            .collect(),
        gates
            .lines()
            .map(|line| {
                let (gate_data, output) = line.split_once(" -> ").unwrap();
                let [w1, op, w2] = gate_data.split(" ").collect::<Vec<_>>().try_into().unwrap();
                (w1, op, w2, output)
            })
            .collect(),
    )
}
