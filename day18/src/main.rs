mod vector2;
use std::collections::{HashMap, HashSet};
use vector2::{Grid, Vector2i};

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let byte_positions = parse_input(input);

    let grid_size = 71;

    let grid = grid_after_n_bytes(&byte_positions, grid_size, 1024);

    let shortest_length = find_shortest_path(&grid);
    println!("Shortest length: {:?}", shortest_length);

    let first_cutoff = find_first_byte_cutoff(&byte_positions, grid_size);
    println!("First cutoff: {:?}", first_cutoff);
}

fn find_shortest_path(grid: &Grid<bool>) -> Option<usize> {
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

    let distances = vector2::dijkstra(graph, Vector2i::new(0, 0));
    let distance = distances.get(&(grid.size - Vector2i::new(1, 1))).unwrap().0;
    println!("Distance: {distance}");
    if distance == usize::MAX {
        return None;
    }
    Some(distance)
}

fn find_first_byte_cutoff(byte_positions: &[Vector2i], grid_size: i64) -> String {
    let index = (2950..byte_positions.len())
        .find(|n| {
            let grid = grid_after_n_bytes(byte_positions, grid_size, *n);
            find_shortest_path(&grid).is_none()
        })
        .unwrap();
    let coordinate = byte_positions[index - 1];
    format!("{},{}", coordinate.x, coordinate.y)
}

fn grid_after_n_bytes(byte_positions: &[Vector2i], grid_size: i64, n: usize) -> Grid<bool> {
    let mut grid = Grid::empty(Vector2i::new(grid_size, grid_size), false);
    grid.coordinates().into_iter().for_each(|coord| {
        grid.set(&coord, true);
    });
    byte_positions.iter().take(n).for_each(|coord| {
        grid.set(coord, false);
    });

    grid
}

fn parse_input(input: &str) -> Vec<Vector2i> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            Vector2i::new(left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
        })
        .collect()
}

const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
