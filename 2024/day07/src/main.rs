#[derive(Debug)]
struct Equation {
    result: i64,
    terms: Vec<i64>,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let equations = parse_input(input);

    let calibration_sum = total_calibration_sum(&equations, false);
    println!("Calibration sum: {calibration_sum}");
    let calibration_sum_2 = total_calibration_sum(&equations, true);
    println!("Calibration sum 2: {calibration_sum_2}");
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .split("\n")
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let result = left.parse::<i64>().unwrap();
            let terms = right
                .split(" ")
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            Equation { result, terms }
        })
        .collect()
}

fn total_calibration_sum(equations: &[Equation], concat: bool) -> i64 {
    equations
        .iter()
        .filter_map(|equation| {
            if can_compute(equation, concat) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}

fn can_compute(equation: &Equation, concat: bool) -> bool {
    let possibilities = compute_all_possibilities(&equation.terms, concat);
    let can_compute = possibilities.iter().any(|x| *x == equation.result);
    can_compute
}

fn compute_all_possibilities(terms: &[i64], concat: bool) -> Vec<i64> {
    if terms.len() == 1 {
        return vec![terms[0]];
    }
    let all_deeper_possibilities = compute_all_possibilities(&terms[0..terms.len() - 1], concat);
    let mut add: Vec<i64> = all_deeper_possibilities
        .iter()
        .map(|x| x + terms[terms.len() - 1])
        .collect();
    let mut mul: Vec<i64> = all_deeper_possibilities
        .iter()
        .map(|x| x * terms[terms.len() - 1])
        .collect();
    add.append(&mut mul);

    if concat {
        let mut concat: Vec<i64> = all_deeper_possibilities
            .iter()
            .map(|x| {
                (x.to_string() + &terms[terms.len() - 1].to_string())
                    .parse::<i64>()
                    .unwrap()
            })
            .collect();
        add.append(&mut concat);
    }
    add
}

const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
