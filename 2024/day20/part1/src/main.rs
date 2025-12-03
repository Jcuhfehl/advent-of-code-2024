mod vector2;
use rayon::prelude::*;
use std::collections::HashMap;
use vector2::Grid;
use vector2::Vector2i;

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let (grid, start_pos, end_pos) = parse_input(input);

    let normal_solution = solve_maze_with_skip(&grid, start_pos, end_pos);
    println!("Normal solution: {}", normal_solution);

    let number_of_good_cheats = find_all_good_cheats(&grid, start_pos, end_pos);
    println!("Number of good cheats: {}", number_of_good_cheats);
}

fn find_all_good_cheats(grid: &Grid<bool>, start_pos: Vector2i, end_pos: Vector2i) -> usize {
    let all_cheats: Vec<Vec<Vector2i>> = grid
        .coordinates()
        .into_iter()
        .filter(|coord| !grid.get(coord))
        .map(|coord| vec![coord])
        .collect();
    let num_cheats_to_try = all_cheats.len();
    println!("Number of cheats to try: {num_cheats_to_try}");

    let base_speed = solve_maze_with_skip(grid, start_pos, end_pos);
    let cheat_advantages: Vec<_> = all_cheats
        .into_par_iter()
        .enumerate()
        .filter_map(|(i, cheat)| {
            println!("{i}/{num_cheats_to_try}");
            let cheated_grid = add_skip_to_maze(grid.clone(), &cheat);
            let advantage = base_speed - solve_maze_with_skip(&cheated_grid, start_pos, end_pos);
            if advantage >= 100 {
                Some(advantage)
            } else {
                None
            }
        })
        .collect();
    println!("Cheat advantages: {:?}", cheat_advantages);

    let mut hashmap_thingy = HashMap::new();
    cheat_advantages
        .iter()
        .for_each(|advantage| *hashmap_thingy.entry(advantage).or_insert(0) += 1);
    hashmap_thingy
        .into_iter()
        .for_each(|(a, b)| println!("{a}: {b}"));
    cheat_advantages.len()
}

fn add_skip_to_maze(mut grid: Grid<bool>, skip_pos: &[Vector2i]) -> Grid<bool> {
    skip_pos.iter().for_each(|pos| {
        grid.set(pos, true);
    });
    grid
}

fn solve_maze_with_skip(grid: &Grid<bool>, start_pos: Vector2i, end_pos: Vector2i) -> usize {
    let mut graph: HashMap<Vector2i, Vec<(usize, Vector2i)>> = HashMap::new();

    for coord in grid.coordinates() {
        if !grid.get(&coord) {
            continue;
        }

        let valid_destinations = Vector2i::DIRECTION_VECTORS
            .into_iter()
            .filter_map(|direction| {
                if *grid.get(&(coord + direction)) {
                    Some((1, (coord + direction)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        graph.insert(coord, valid_destinations);
    }

    let distances = vector2::dijkstra(graph, start_pos);
    let distance = distances.get(&end_pos).unwrap().0;
    distance
}

fn parse_input(input: &str) -> (Grid<bool>, Vector2i, Vector2i) {
    let mut start_pos = Vector2i::new(0, 0);
    let mut end_pos = Vector2i::new(0, 0);
    let size = input.lines().count();
    let mut grid = Grid::empty(Vector2i::new(size as i64, size as i64), false);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let coord = Vector2i::new(x as i64, y as i64);
            match c {
                '#' => {}
                '.' => {
                    grid.set(&coord, true);
                }
                'S' => {
                    start_pos = coord;
                    grid.set(&coord, true);
                }
                'E' => {
                    end_pos = coord;
                    grid.set(&coord, true);
                }
                _ => todo!(),
            };
        })
    });
    (grid, start_pos, end_pos)
}

const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
