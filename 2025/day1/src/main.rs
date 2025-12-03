mod vector2;
use vector2::Vector2i;

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let moves = parse_input(input);

    let sol1 = part1(&moves);
    println!("Part 1: {sol1}");

    let sol2 = part2(&moves);
    println!("Part 2: {sol2}");
}

fn part1(moves: &[i16]) -> usize {
    let mut count = 0;
    let mut position = 50;
    for this_move in moves {
        position += this_move;
        position = (position % 100);
        if position < 0 {
            position = 100 + position;
        }
        println!("{position}");
        if position == 0 {
            count += 1;
        }
    }

    count
}

fn part2(moves: &[i16]) -> usize {
    let mut count = 0;
    let mut position = 50;
    for this_move in moves {
        let was0 = position == 0;
        position += this_move;
        let (new_position, modulos) = modulo(position, 100);
        position = new_position;
        if position == 0 {
            count += 1;
        }
        count += modulos;
        if was0 {
            count -= 1;
        }
        println!("{position}, {count}");
    }

    count
}

fn modulo(mut a: i16, b: i16) -> (i16, usize) {
    let mut modulos = 0;
    while a >= b {
        modulos += 1;
        a -= b;
    }
    while a < 0 {
        modulos += 1;
        a += b;
    }
    (a, modulos)
}

fn parse_input(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|x| {
            let sign = match x.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => unreachable!(),
            };
            let value = x[1..].parse::<i16>().expect("Invalid input");
            sign * value
        })
        .collect()
}

const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
