use std::collections::HashMap;
use std::iter;

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let stones = parse_input(TEST_INPUT);
    let zero_history = blink_n_times_with_history(vec![0], 40);
    let count_ultra = blink_n_times_v2(stones, 25, &zero_history);
    println!("Count: {count_ultra}");
}

fn blink_n_times(mut stones: Vec<usize>, n: usize) -> usize {
    for i in 0..n {
        println!("Step {i}");
        //println!("Stones: {:?}", stones);
        println!("Length: {}", stones.len());
        stones = blink(stones);
    }
    stones.len()
}

fn blink_n_times_with_history(mut stones: Vec<usize>, n: usize) -> Vec<usize> {
    let mut history = vec![stones.len()];
    for i in 0..n {
        println!("Step {i}");
        //println!("Stones: {:?}", stones);
        println!("Length: {}", stones.len());
        stones = blink(stones);
        history.push(stones.len());
    }
    history
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    stones
        .into_iter()
        .flat_map(|stone| {
            let stone_string = stone.to_string();
            if stone == 0 {
                Box::new(iter::once(1_usize)) as Box<dyn Iterator<Item = usize>>
            } else if (stone_string.len()) % 2 == 0 {
                let (left, right) = stone_string.split_at(stone_string.len() / 2);
                Box::new(
                    iter::once(left.parse::<usize>().unwrap())
                        .chain(iter::once(right.parse::<usize>().unwrap())),
                ) as Box<dyn Iterator<Item = usize>>
            } else {
                Box::new(iter::once(stone * 2024)) as Box<dyn Iterator<Item = usize>>
            }
        })
        .collect()
}

fn blink_n_times_v2(mut stones: Vec<usize>, n: usize, zero_history: &Vec<usize>) -> usize {
    let mut n_removed = 0;
    for i in 0..n {
        println!("Step {i}");
        //println!("Stones: {:?}", stones);
        println!("Length: {}", stones.len());
        stones = blink(stones);
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                let stone_string = stone.to_string();
                if stone == 0 {
                    match zero_history.get(n - i) {
                        Some(n) => {
                            n_removed += n;
                            Box::new(iter::empty())
                        }
                        None => Box::new(iter::once(1_usize)) as Box<dyn Iterator<Item = usize>>,
                    }
                } else if (stone_string.len()) % 2 == 0 {
                    let (left, right) = stone_string.split_at(stone_string.len() / 2);
                    Box::new(
                        iter::once(left.parse::<usize>().unwrap())
                            .chain(iter::once(right.parse::<usize>().unwrap())),
                    ) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new(iter::once(stone * 2024)) as Box<dyn Iterator<Item = usize>>
                }
            })
            .collect()
    }
    stones.len() + n_removed
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

const TEST_INPUT: &str = "0";
