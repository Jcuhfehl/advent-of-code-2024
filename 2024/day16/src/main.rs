mod vector2;
use std::collections::{HashMap, HashSet};
use vector2::Grid;
use vector2::Vector2i;

struct Maze {
    map: Grid<bool>,
    start: Vector2i,
    end: Vector2i,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct MazePosition {
    location: Vector2i,
    rotation: Vector2i,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let maze = parse_input(input);

    let (points, optimal_positions) = solve_maze(&maze);
    println!("Minimum points: {:?}", points);
    println!("Optimal nodes: {:?}", optimal_positions.len());
}

fn solve_maze(maze: &Maze) -> (usize, HashSet<Vector2i>) {
    let mut graph: HashMap<MazePosition, Vec<(usize, MazePosition)>> = HashMap::new();

    for coord in maze.map.coordinates() {
        if !maze.map.get(&coord) {
            continue;
        }
        for rotation in Vector2i::DIRECTION_VECTORS {
            let this_position = MazePosition {
                location: coord,
                rotation,
            };
            let left_position = MazePosition {
                location: coord,
                rotation: rotation.rotate_left(),
            };
            let right_position = MazePosition {
                location: coord,
                rotation: rotation.rotate_right(),
            };
            let mut reachable_positions = vec![(1000, left_position), (1000, right_position)];

            let forward_location = coord + rotation;
            if *maze.map.get(&forward_location) {
                reachable_positions.push((1, MazePosition {
                    location: forward_location,
                    rotation,
                }));
            }
            graph.insert(this_position, reachable_positions);
        }
    }

    let mut unvisited: HashSet<MazePosition> = graph.keys().cloned().collect();
    let mut distances: HashMap<MazePosition, (usize, Vec<MazePosition>)> = HashMap::from_iter(
        unvisited
            .iter()
            .cloned()
            .map(|position| (position, (usize::MAX, Vec::new()))),
    );
    let starting_position = MazePosition {
        location: maze.start,
        rotation: Vector2i::new(1, 0),
    };
    *distances.get_mut(&starting_position).unwrap() = (0, Vec::new());

    let mut optimal_distance = usize::MAX;
    let mut found_optimal_distance = false;

    while !unvisited.is_empty() {
        println!("Unvisited count: {}", unvisited.len());
        let evaluate_position = unvisited
            .iter()
            .min_by_key(|position| distances[position].0)
            .unwrap();
        let evaluate_distance = distances[evaluate_position].0;

        if evaluate_position.location == maze.end && !found_optimal_distance {
            found_optimal_distance = true;
            optimal_distance = evaluate_distance;
        }

        for (hop_distance, next_node) in &graph[evaluate_position] {
            let next_node_current_distance = distances.get_mut(next_node).unwrap();
            if next_node_current_distance.0 > evaluate_distance + *hop_distance {
                *next_node_current_distance = (evaluate_distance + hop_distance, Vec::new());
            }
            if next_node_current_distance.0 == evaluate_distance + *hop_distance {
                next_node_current_distance.1.push(*evaluate_position);
            }
        }
        unvisited.remove(&evaluate_position.clone());
    }

    let visited_nodes = find_all_visited_nodes(maze.end, optimal_distance, &distances);
    (optimal_distance, visited_nodes)
}

fn find_all_visited_nodes(
    start_position: Vector2i,
    shortest_distance: usize,
    distances: &HashMap<MazePosition, (usize, Vec<MazePosition>)>,
) -> HashSet<Vector2i> {
    let mut set = HashSet::new();
    distances.keys().for_each(|node| {
        if node.location == start_position && distances[node].0 == shortest_distance {
            set.extend(_find_all_visited_nodes(*node, distances, &HashSet::new()))
        }
    });

    set.into_iter().map(|node| node.location).collect()
}

fn _find_all_visited_nodes(
    node: MazePosition,
    distances: &HashMap<MazePosition, (usize, Vec<MazePosition>)>,
    nodes_so_far: &HashSet<MazePosition>,
) -> HashSet<MazePosition> {
    let mut set = HashSet::new();
    set.insert(node);

    let mut to_explore_nodes = vec![node];
    while let Some(node) = to_explore_nodes.pop() {
        for next_node in &distances[&node].1 {
            if set.contains(next_node) {
                continue;
            }
            to_explore_nodes.push(*next_node);
            set.insert(*next_node);
        }
    }

    set
}

fn parse_input(input: &str) -> Maze {
    let size = input.lines().count();
    let mut map = Grid::empty(Vector2i::new(size as i64, size as i64), false);
    let mut start = Vector2i::new(-1, -1);
    let mut end = Vector2i::new(-1, -1);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                map.set(&Vector2i::new(x as i64, y as i64), true);
            }
            '#' => {
                map.set(&Vector2i::new(x as i64, y as i64), false);
            }
            'S' => {
                start = Vector2i::new(x as i64, y as i64);
                map.set(&Vector2i::new(x as i64, y as i64), true);
            }
            'E' => {
                end = Vector2i::new(x as i64, y as i64);
                map.set(&Vector2i::new(x as i64, y as i64), true);
            }
            _ => unreachable!(),
        })
    });

    Maze { map, start, end }
}

const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const TEST_INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
