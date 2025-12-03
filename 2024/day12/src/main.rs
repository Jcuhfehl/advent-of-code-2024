mod vector2;

use std::collections::HashSet;
use vector2::Vector2i;

#[derive(Debug)]
struct Grid<T> {
    default: T,
    size: usize,
    values: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn get(&self, location: &Vector2i) -> &T {
        if location.x < 0
            || location.x >= self.size as i64
            || location.y < 0
            || location.y >= self.size as i64
        {
            return &self.default;
        }
        &self.values[location.x as usize][location.y as usize]
    }

    fn get_mut(&mut self, location: &Vector2i) -> &mut T {
        if location.x < 0
            || location.x >= self.size as i64
            || location.y < 0
            || location.y >= self.size as i64
        {
            return &mut self.default;
        }
        &mut self.values[location.x as usize][location.y as usize]
    }

    fn set(&mut self, location: &Vector2i, value: T) -> bool {
        if location.x < 0
            || location.x >= self.size as i64
            || location.y < 0
            || location.y >= self.size as i64
        {
            return false;
        }
        self.values[location.x as usize][location.y as usize] = value;
        true
    }

    fn coordinates(&self) -> Vec<Vector2i> {
        (0..self.size)
            .flat_map(|y| {
                (0..self.size).map(move |x| Vector2i {
                    x: x as i64,
                    y: y as i64,
                })
            })
            .collect()
    }
}

impl<T: Clone> Grid<T> {
    fn empty(size: usize, default: T) -> Grid<T> {
        let values = (0..size)
            .map(|_| (0..size).map(|_| default.clone()).collect())
            .collect();

        Grid::<T> {
            default,
            size,
            values,
        }
    }
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let maps = parse_input(input);

    let cost = count_cost(&maps);
    println!("Cost: {cost}");

    let cost_with_discount = count_cost_with_discount(&maps);
    println!("Cost with discount: {cost_with_discount}");
}

fn count_cost(maps: &[Grid<bool>]) -> usize {
    maps.iter()
        .map(|map| {
            let area = map
                .coordinates()
                .into_iter()
                .filter(|coordinate| *map.get(coordinate))
                .count();
            let perimiter: usize = map
                .coordinates()
                .into_iter()
                .map(|coordinate| {
                    if *map.get(&coordinate) {
                        Vector2i::DIRECTION_VECTORS
                            .iter()
                            .filter(|direction| !*map.get(&(coordinate + **direction)))
                            .count()
                    } else {
                        0
                    }
                })
                .sum();
            area * perimiter
        })
        .sum()
}

fn count_cost_with_discount(maps: &[Grid<bool>]) -> usize {
    maps.iter()
        .map(|map| {
            let area = map
                .coordinates()
                .into_iter()
                .filter(|coordinate| *map.get(coordinate))
                .count();

            let mut direction_map: Grid<Vec<Vector2i>> = Grid::empty(map.size, Vec::new());
            map.coordinates().iter().for_each(|coord| {
                if *map.get(coord) {
                    assert!(direction_map.set(coord, Vector2i::DIRECTION_VECTORS.to_vec()))
                }
            });
            direction_map.coordinates().iter().for_each(|coord| {
                direction_map.set(
                    coord,
                    direction_map
                        .get(coord)
                        .iter()
                        .filter(|direction| !map.get(&(*coord + **direction)))
                        .copied()
                        .collect(),
                );
            });

            for coord in direction_map.coordinates() {
                if !map.get(&coord) {
                    continue;
                };
                for direction in direction_map.get(&coord).clone() {
                    let scan_direction = direction.perpendicular();
                    for i in 1..map.size {
                        let next_scan_pos = coord + scan_direction * i as i64;
                        if !direction_map.get(&(next_scan_pos)).contains(&direction) {
                            break;
                        }
                        direction_map
                            .get_mut(&next_scan_pos)
                            .retain(|x| *x != direction);
                    }
                }
            }

            let edges: usize = direction_map
                .coordinates()
                .iter()
                .map(|coord| direction_map.get(coord).len())
                .sum();
            area * edges
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Grid<bool>> {
    let size = input.lines().count();
    let values = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| char)
                .collect()
        })
        .collect();
    let map = Grid {
        size,
        values,
        default: '/',
    };
    let mut used_coords: HashSet<Vector2i> = HashSet::new();

    map.coordinates()
        .into_iter()
        .filter_map(|coord| {
            let mut new_map = Grid::<bool>::empty(size, false);
            if used_coords.contains(&coord) {
                return None;
            }
            let this_char = map.get(&coord);
            let mut to_explore: Vec<Vector2i> = vec![coord];
            while !to_explore.is_empty() {
                let explore_now = to_explore.pop().unwrap();
                new_map.values[explore_now.x as usize][explore_now.y as usize] = true;
                to_explore.extend(Vector2i::DIRECTION_VECTORS.iter().filter_map(|dir| {
                    let next_coord = explore_now + *dir;
                    let is_already_used = used_coords.contains(&next_coord);
                    let is_same_char = map.get(&next_coord) == this_char;
                    if !is_already_used && is_same_char {
                        used_coords.insert(next_coord);
                        return Some(next_coord);
                    }
                    None
                }));
            }
            Some(new_map)
        })
        .collect()
}

const TEST_INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
