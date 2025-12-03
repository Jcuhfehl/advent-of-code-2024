use rayon::prelude::*;
use std::{collections::HashSet, fs};

#[derive(Clone)]
struct MapData {
    obstacles: Vec<(usize, usize)>,
    start_position: (usize, usize),
    map_size: usize,
}

#[derive(PartialEq, Eq)]
struct PathPoint {
    position: (usize, usize),
    direction: (isize, isize),
}

fn main() {
    let input_with_trailing =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();
    let data: MapData = parse_input(input);
    let path = find_path(&data);
    let path_unique_locations = HashSet::<(usize, usize)>::from_iter(path.clone());
    let unique_count = path_unique_locations.len();
    println!("Part one: {unique_count}");
    let possible_loop_count = find_all_loops(&data, &path_unique_locations.into_iter().collect());
    println!("Part two: {possible_loop_count}");
}

fn find_path(data: &MapData) -> Vec<(usize, usize)> {
    let mut position = data.start_position;
    let mut direction = (0, 1);
    let mut path = Vec::new();
    while 0 < position.0
        && position.0 < data.map_size
        && 0 < position.1
        && position.1 < data.map_size
    {
        if !can_move_to_next_tile(data, position, direction) {
            direction = rotate_vec2_right(direction);
            continue;
        }
        position = (
            (position.0 as isize + direction.0) as usize,
            (position.1 as isize + direction.1) as usize,
        );
        path.push(position);
    }
    path
}

fn find_all_loops(data: &MapData, path: &Vec<(usize, usize)>) -> usize {
    path.par_iter()
        .filter(|(x, y)| {
            if data.start_position == (*x, *y) {
                return false;
            }
            let mut new_data: MapData = data.clone();
            new_data.obstacles.push((*x, *y));
            if has_loop(&new_data) {
                return true;
            }
            false
        })
        .count()
}

fn has_loop(data: &MapData) -> bool {
    let mut position = data.start_position;
    let mut direction = (0, 1);
    let mut path: Vec<PathPoint> = Vec::new();
    let mut visited_tiles: Vec<Vec<bool>> = (0..data.map_size + 1)
        .map(|_| (0..data.map_size + 1).map(|_| false).collect())
        .collect();
    while 0 < position.0
        && position.0 < data.map_size
        && 0 < position.1
        && position.1 < data.map_size
    {
        if !can_move_to_next_tile(data, position, direction) {
            direction = rotate_vec2_right(direction);
            continue;
        }
        position = (
            (position.0 as isize + direction.0) as usize,
            (position.1 as isize + direction.1) as usize,
        );
        let path_point = PathPoint {
            position,
            direction,
        };
        if visited_tiles[position.0][position.1] {
            if path.contains(&path_point) {
                return true;
            }
        }
        visited_tiles[position.0][position.1] = true;
        path.push(path_point);
    }
    false
}

fn can_move_to_next_tile(
    data: &MapData,
    position: (usize, usize),
    direction: (isize, isize),
) -> bool {
    let next_position = (
        (position.0 as isize + direction.0) as usize,
        (position.1 as isize + direction.1) as usize,
    );
    !data
        .obstacles
        .iter()
        .any(|obstacle| *obstacle == next_position)
}

fn rotate_vec2_right(current: (isize, isize)) -> (isize, isize) {
    (current.1, -current.0)
}

fn parse_input(input: &str) -> MapData {
    let map_size = input.split("\n").count();
    let mut start_position = (0, 0);
    let mut obstacles = Vec::new();
    input
        .split("\n")
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char == '#' {
                    obstacles.push((x, y))
                } else if char == '^' {
                    start_position = (x, y)
                }
            })
        });
    MapData {
        obstacles,
        start_position,
        map_size,
    }
}

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
