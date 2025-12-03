mod vector2;
use vector2::Vector2i;

const NUMPAD_POSITIONS: [Vector2i; 11] = [
    Vector2i { x: 1, y: 0 }, // 0
    Vector2i { x: 0, y: 1 }, // 1
    Vector2i { x: 1, y: 1 }, // 2
    Vector2i { x: 2, y: 1 }, // 3
    Vector2i { x: 0, y: 2 }, // 4
    Vector2i { x: 1, y: 2 }, // 5
    Vector2i { x: 2, y: 2 }, // 6
    Vector2i { x: 0, y: 3 }, // 7
    Vector2i { x: 1, y: 3 }, // 8
    Vector2i { x: 2, y: 3 }, // 9
    Vector2i { x: 2, y: 0 }, // A
];

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let codes = parse_input(TEST_INPUT);

    let total_complexity = total_complexity(&codes);
}

fn total_complexity(codes: &[Vec<u8>]) -> usize {
    codes.iter().map(|code| complexity(code)).sum()
}

fn complexity(key: &[u8]) -> usize {
    let key_num: usize = key
        .iter()
        .rev()
        .skip(1)
        .enumerate()
        .map(|(i, n)| 10_usize.pow(i as u32) * (*n as usize))
        .sum();

    let sequence_length = sequence_length(key);
    println!("{:?}: {}", key, sequence_length);
    key_num * sequence_length
}

fn sequence_length(key: &[u8]) -> usize {
    std::iter::once(&10)
        .chain(key.iter())
        .zip(key.iter())
        .map(|(a, b)| {
            let difference = NUMPAD_POSITIONS[*a as usize] - NUMPAD_POSITIONS[*b as usize];
            let distance = difference.x.abs() + difference.y.abs();
            println!("{a} to {b}: {distance}");
            distance as usize
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c.to_digit(10) {
                    Some(n) => n as u8,
                    None => 10,
                })
                .collect()
        })
        .collect()
}

const TEST_INPUT: &str = "029A
980A
179A
456A
379A";
