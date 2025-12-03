mod vector2;
use memoize::memoize;

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let (parts, sequences) = parse_input(input);

    let number_possible = number_of_possible_sequence(parts.clone(), sequences.clone());
    println!("Number possible: {}", number_possible);

    let number_possibilities = number_of_possibilities(parts, sequences);
    println!("Number possibilities: {}", number_possibilities);
}

fn number_of_possible_sequence(parts: Vec<&str>, sequences: Vec<&str>) -> usize {
    sequences
        .into_iter()
        .filter(|sequence| {
            let possible = is_sequence_possible(&parts, sequence);
            println!("{}: {}", sequence, possible);
            possible
        })
        .count()
}

fn is_sequence_possible(parts: &Vec<&str>, sequence: &str) -> bool {
    parts.iter().any(|part| {
        if !sequence.starts_with(*part) {
            return false;
        }
        if sequence == *part {
            return true;
        }
        let trimmed_sequence = &sequence[part.len()..];
        is_sequence_possible(parts, trimmed_sequence)
    })
}

fn number_of_possibilities(parts: Vec<&str>, sequences: Vec<&str>) -> usize {
    let parts: Vec<_> = parts.into_iter().map(|x| x.to_string()).collect();
    sequences
        .into_iter()
        .map(|sequence| {
            let possible = sequence_possibilities(parts.clone(), sequence.to_string());
            println!("{}: {}", sequence, possible);
            possible
        })
        .sum()
}

#[memoize]
fn sequence_possibilities(parts: Vec<String>, sequence: String) -> usize {
    parts
        .iter()
        .map(|part| {
            if !sequence.starts_with(part) {
                return 0;
            }
            if sequence == *part {
                return 1;
            }
            let trimmed_sequence = &sequence[part.len()..];
            sequence_possibilities(parts.clone(), trimmed_sequence.to_string())
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (parts_str, sequences_str) = input.split_once("\n\n").unwrap();

    let parts = parts_str.split(", ").collect();
    let sequences = sequences_str.lines().collect();
    (parts, sequences)
}

const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
