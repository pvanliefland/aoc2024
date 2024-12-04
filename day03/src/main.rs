const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<String>;

fn main() {
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input_1));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input_2));
    println!("         validation    {} ", part_2(&input));
}

enum State {
    Do,
    Dont,
    Mul(Option<String>, Option<String>, bool),
}

fn part_1(input: &Input) -> u32 {
    process(input, false)
}

fn part_2(input: &Input) -> u32 {
    process(input, true)
}

fn process(input: &Input, dos_and_donts: bool) -> u32 {
    let mut state = State::Do;
    let mut valid = 0u32;
    for line in input {
        let mut cursor = 0;
        loop {
            if cursor >= line.len() {
                break;
            } else if let Some("don't()") = line.get(cursor..cursor + 7) {
                if dos_and_donts {
                    state = State::Dont;
                };
                cursor += 7;
            }
            match state {
                State::Do => {
                    if let Some("mul(") = line.get(cursor..cursor + 4) {
                        state = State::Mul(None, None, false);
                        cursor += 4;
                    } else {
                        cursor += 1;
                    }
                }
                State::Dont => {
                    if let Some("do()") = line.get(cursor..cursor + 4) {
                        state = State::Do;
                        cursor += 4;
                    } else {
                        cursor += 1;
                    }
                }
                State::Mul(None, None, false) => {
                    if let Some(digit) = get_digit(line, cursor) {
                        state = State::Mul(Some(digit.to_string()), None, false);
                        cursor += 1;
                    } else {
                        state = State::Do;
                        cursor += 1;
                    }
                }
                State::Mul(Some(left), None, false) => {
                    if let Some(digit) = get_digit(line, cursor) {
                        state = State::Mul(Some(left + &digit.to_string()), None, false);
                    } else if &line[cursor..cursor + 1] == "," {
                        state = State::Mul(Some(left), None, true);
                    } else {
                        state = State::Do;
                    }
                    cursor += 1;
                }
                State::Mul(Some(left), None, true) => {
                    if let Some(digit) = get_digit(line, cursor) {
                        state = State::Mul(Some(left), Some(digit.to_string()), true);
                    } else {
                        state = State::Do;
                    }
                    cursor += 1;
                }
                State::Mul(Some(left), Some(right), true) => {
                    if let Some(digit) = get_digit(line, cursor) {
                        state = State::Mul(Some(left), Some(right + &digit.to_string()), true);
                    } else if &line[cursor..cursor + 1] == ")" {
                        valid += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
                        state = State::Do;
                    } else {
                        state = State::Do;
                    }
                    cursor += 1;
                }
                _ => {
                    state = State::Do;
                    cursor += 1;
                }
            }
        }
    }
    valid
}

fn get_digit(line: &str, position: usize) -> Option<char> {
    line.get(position..position + 1)
        .map(|slice| slice.chars().next().unwrap())
        .filter(|char| char.is_ascii_digit())
}

fn parse(input: &str) -> Input {
    input.trim().lines().map(String::from).collect()
}
