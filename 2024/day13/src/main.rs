struct Machine {
    x_1: i64,
    y_1: i64,
    x_2: i64,
    y_2: i64,
    x: i64,
    y: i64,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let mut machines = parse_input(input);

    let cost = calculate_prize(&machines);
    println!("Cost: {}", cost);

    add_offset(&mut machines);
    let cost = calculate_prize(&machines);
    println!("Cost 2: {}", cost);
}
fn add_offset(machines: &mut [Machine]) {
    machines.iter_mut().for_each(|machine| {
        machine.x += 10000000000000;
        machine.y += 10000000000000;
    });
}

fn calculate_prize(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| {
            if let Some((p, q)) = solution(machine) {
                (3 * p + q) as usize
            } else {
                0
            }
        })
        .sum()
}

fn solution(machine: &Machine) -> Option<(i64, i64)> {
    let top = machine.y * machine.x_1 - machine.x * machine.y_1;
    let bottom = machine.y_2 * machine.x_1 - machine.x_2 * machine.y_1;

    if top % bottom != 0 {
        return None;
    }
    let q = top / bottom;

    let top_2 = machine.x - q * machine.x_2;

    if top_2 % machine.x_1 != 0 {
        return None;
    }

    let p = top_2 / machine.x_1;

    Some((p, q))
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| {
            let lines: Vec<_> = machine.split("\n").collect();
            let line1: Vec<_> = lines[0].split(" ").collect();
            let line2: Vec<_> = lines[1].split(" ").collect();
            let line3: Vec<_> = lines[2].split(" ").collect();

            let x_1 = parse_integer_stuff(line1[2], "+", ",");
            let y_1 = parse_integer_stuff(line1[3], "+", "\n");

            let x_2 = parse_integer_stuff(line2[2], "+", ",");
            let y_2 = parse_integer_stuff(line2[3], "+", "\n");

            let x = parse_integer_stuff(line3[1], "=", ",");
            let y = parse_integer_stuff(line3[2], "=", "\n");
            Machine {
                x_1,
                y_1,
                x_2,
                y_2,
                x,
                y,
            }
        })
        .collect()
}

fn parse_integer_stuff(word: &str, begin: &str, end: &str) -> i64 {
    word.split(begin)
        .nth(1)
        .unwrap()
        .split(end)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
