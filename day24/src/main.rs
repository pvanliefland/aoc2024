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
    let (initial_values, gates) = input;
    let mut bits = [0u8; 64];
    gates
        .iter()
        .filter(|(_, _, _, o)| o.starts_with("z"))
        .for_each(|gate| {
            bits[gate.3.replace("z", "").parse::<usize>().unwrap()] =
                compute_gate_value(*gate, gates, initial_values);
        });
    bits.into_iter()
        .rev()
        .fold(0, |acc, bit| (acc << 1) + bit as usize)
}

fn part_2(input: &Input) -> usize {
    let (initial_values, _gates) = input;
    let mut x_wires = initial_values
        .iter()
        .filter(|(w, _)| w.starts_with("x"))
        .collect::<Vec<_>>();
    x_wires.sort_by_key(|(w, _)| *w);
    let x = x_wires
        .into_iter()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) + *v as usize);
    let mut y_wires = initial_values
        .iter()
        .filter(|(w, _)| w.starts_with("y"))
        .collect::<Vec<_>>();
    y_wires.sort_by_key(|(w, _)| *w);
    let y = y_wires
        .into_iter()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) + *v as usize);
    dbg!(x, y);
    42
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
