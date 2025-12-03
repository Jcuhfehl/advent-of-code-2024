use std::fs;

fn main() {
    let input_with_trailing =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();
    let (rules, data) = parse_input(input);
    let (correct_data, incorrect_data) = sort_data(&rules, &data);
    let result1 = sum_middle_pages(&correct_data);
    println!("Result 1: {result1}");

    let fixed_incorrect_data = fix_incorrect_data(&rules, incorrect_data);
    let slices: Vec<&Vec<usize>> = fixed_incorrect_data.iter().collect();
    let result2 = sum_middle_pages(&slices);
    println!("Result 2: {result2}");
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules_str, data_str) = input.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules_str
        .split("\n")
        .map(|x| {
            let (a, b) = x.split_once("|").unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    let data: Vec<Vec<usize>> = data_str
        .split("\n")
        .map(|x| x.split(",").map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();
    (rules, data)
}

fn fix_incorrect_data(rules: &[(usize, usize)], data: Vec<&Vec<usize>>) -> Vec<Vec<usize>> {
    data.into_iter()
        .map(|x| fix_single_data(rules, x.clone()))
        .collect()
}

fn fix_single_data(rules: &[(usize, usize)], mut data: Vec<usize>) -> Vec<usize> {
    while !verify_data(rules, &data) {
        let mut number_indices = [usize::MAX; 100];
        data.iter()
            .enumerate()
            .for_each(|(i, n)| number_indices[*n] = i);
        for (a, b) in rules {
            if !(number_indices[*a] < number_indices[*b]
                || number_indices[*a] == usize::MAX
                || number_indices[*b] == usize::MAX)
            {
                data.swap(number_indices[*a], number_indices[*b]);
                number_indices.swap(*a, *b);
            }
        }
    }
    data
}

fn sum_middle_pages(data: &[&Vec<usize>]) -> usize {
    data.iter().map(|x| x[x.len() / 2]).sum()
}

fn sort_data<'a>(
    rules: &[(usize, usize)],
    data: &'a [Vec<usize>],
) -> (Vec<&'a Vec<usize>>, Vec<&'a Vec<usize>>) {
    let correct = data.iter().filter(|x| verify_data(rules, x)).collect();
    let incorrect = data.iter().filter(|x| !verify_data(rules, x)).collect();
    (correct, incorrect)
}

fn verify_data(rules: &[(usize, usize)], data: &[usize]) -> bool {
    let mut number_indices = [usize::MAX; 100];
    data.iter()
        .enumerate()
        .for_each(|(i, n)| number_indices[*n] = i);

    rules.iter().all(|(a, b)| {
        number_indices[*a] < number_indices[*b]
            || number_indices[*a] == usize::MAX
            || number_indices[*b] == usize::MAX
    })
}

const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
