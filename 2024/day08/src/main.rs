use std::collections::{HashMap, HashSet};
use vector2d::Vector2D;

#[derive(Debug, PartialEq)]
struct Antenna {
    location: Vector2D<i64>,
    distances: Vec<Vec<usize>>,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let (antennas, size) = parse_input(input);

    let resonances = find_resonances(&antennas, size as i64);
    println!("Resonances count: {resonances}");

    let resonances_2 = find_resonances_2(&antennas, size as i64);
    println!("Resonances 2 count: {resonances_2}");
}

fn gcd(mut n: i64, mut m: i64) -> i64 {
    if n == 0 || m == 0 {
        return 1;
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn mul_vector2d(n: i64, vec: Vector2D<i64>) -> Vector2D<i64> {
    Vector2D::new(n * vec.x, n * vec.y)
}

fn find_resonances(all_antennas: &HashMap<char, Vec<Antenna>>, size: i64) -> usize {
    let mut resonance_points = HashSet::<(i64, i64)>::new();

    for antennas in all_antennas.values() {
        for antenna1 in antennas.iter() {
            for antenna2 in antennas.iter() {
                if antenna1 == antenna2 {
                    continue;
                }
                let direction = antenna1.location - antenna2.location;
                let normalized_direction = direction / gcd(direction.x.abs(), direction.y.abs());

                let position = antenna1.location + normalized_direction;
                if 0 > position.x || position.x >= size || 0 > position.y || position.y >= size {
                    continue;
                }
                resonance_points.insert((position.x, position.y));
            }
        }
    }

    resonance_points.len()
}

fn find_resonances_2(all_antennas: &HashMap<char, Vec<Antenna>>, size: i64) -> usize {
    let mut resonance_points = HashSet::<(i64, i64)>::new();

    for antennas in all_antennas.values() {
        for antenna1 in antennas.iter() {
            for antenna2 in antennas.iter() {
                if antenna1 == antenna2 {
                    continue;
                }
                let direction = antenna1.location - antenna2.location;
                let normalized_direction = direction / gcd(direction.x.abs(), direction.y.abs());

                for n in -size * 2..size * 2 {
                    let position = antenna1.location + mul_vector2d(n, normalized_direction);
                    if !(0 <= position.x
                        && position.x < size
                        && 0 <= position.y
                        && position.y < size)
                    {
                        continue;
                    }
                    resonance_points.insert((position.x, position.y));
                }
            }
        }
    }

    resonance_points.len()
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Antenna>>, usize) {
    let size = input.split("\n").count();
    let mut antennas = HashMap::new();
    input
        .split("\n")
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
        .filter(|(_, _, char)| *char != '.')
        .for_each(|(x, y, char)| {
            let frequency = char;
            let location = Vector2D::new(x as i64, y as i64);
            let distances = (0..size)
                .map(|y_l| {
                    (0..size)
                        .map(|x_l| x.abs_diff(x_l) + y.abs_diff(y_l))
                        .collect()
                })
                .collect();
            let antenna = Antenna {
                location,
                distances,
            };
            antennas
                .entry(frequency)
                .or_insert_with(Vec::new)
                .push(antenna);
        });
    (antennas, size)
}

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
