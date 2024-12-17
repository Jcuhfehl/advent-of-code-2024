mod vector2;
use vector2::Vector2i;

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let _ = parse_input(TEST_INPUT);
}

fn parse_input(input: &str) {
    todo!()
}

const TEST_INPUT: &str = "";
