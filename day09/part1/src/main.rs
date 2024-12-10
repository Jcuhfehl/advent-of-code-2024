#[derive(Clone, Copy, Debug, PartialEq)]
enum DataBlock {
    Empty,
    File(usize),
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let mut data = parse_input(input);
    defrag_data(&mut data);
    let checksum = calculate_checksum(&data);
    println!("Checksum: {checksum}");
}

fn parse_input(input: &str) -> Vec<DataBlock> {
    let data = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, size)| {
            if i % 2 == 0 {
                std::iter::repeat_n(DataBlock::File(i / 2), size as usize)
            } else {
                std::iter::repeat_n(DataBlock::Empty, size as usize)
            }
        })
        .collect();
    data
}

fn defrag_data(data: &mut [DataBlock]) {
    for i in (0..data.len()).rev() {
        if data[i] == DataBlock::Empty {
            continue;
        }
        let next_available_empty_block = data.iter().position(|x| *x == DataBlock::Empty);
        if let Some(found_index) = next_available_empty_block {
            if found_index >= i {
                break;
            }
            data.swap(i, found_index);
        } else {
            break;
        }
    }
}

fn calculate_checksum(data: &[DataBlock]) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, data)| {
            if let DataBlock::File(index) = data {
                i * index
            } else {
                0
            }
        })
        .sum()
}

const TEST_INPUT: &str = "2333133121414131402";
