const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<String>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(test_input));
    println!("         validation    {} ", part_1(input));
    // println!("Part 2   test          {} ", part_2(test_input));
    // println!("         validation    {} ", part_2(input));
}

fn part_1(input: Input) -> u32 {
    let foo = input
        .iter()
        .map(|line| {
            let mut valid = 0u32;
            let mut char_iterator = line.chars();
            let mut expected_char = Some('m');
            let mut left: Option<String> = None;
            let mut right: Option<String> = None;

            loop {
                let next = match char_iterator.next() {
                    Some(char) => char,
                    None => {
                        break;
                    }
                };

                match (expected_char.as_mut(), next, &left, &right) {
                    (Some('m'), 'm', _, _) => {
                        expected_char.replace('u');
                    }
                    (Some('u'), 'u', _, _) => {
                        expected_char.replace('l');
                    }
                    (Some('l'), 'l', _, _) => {
                        expected_char.replace('(');
                    }
                    (Some('('), '(', _, _) => {
                        expected_char = None;
                        left = Some("".to_string());
                    }
                    (None, next, Some(prev_left), None) if next.is_digit(10) => {
                        left.replace(prev_left.clone() + &next.to_string());
                    }
                    (_, next, Some(prev_left), None) if prev_left != "" && next == ',' => {
                        right = Some("".to_string());
                    }
                    (_, next, Some(_), Some(prev_right)) if next.is_digit(10) => {
                        right.replace(prev_right.clone() + &next.to_string());
                    }
                    (_, next, Some(prev_left), Some(prev_right))
                        if prev_right != "" && next == ')' =>
                    {
                        valid +=
                            prev_left.parse::<u32>().unwrap() * prev_right.parse::<u32>().unwrap();
                        left = None;
                        right = None;
                        expected_char = Some('m');
                    }
                    _ => {
                        left = None;
                        right = None;
                        expected_char = Some('m');
                        continue;
                    }
                }
            }
            valid
        })
        .sum();
    foo
}

// fn part_2(input: Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn parse(input: &str) -> Input {
    input.trim().lines().map(String::from).collect()
}
