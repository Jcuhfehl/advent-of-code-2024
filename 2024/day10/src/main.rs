mod vector2;

use std::collections::HashSet;
use vector2::Vector2i;

struct HeightMap {
    size: i64,
    heights: Vec<Vec<usize>>,
}

impl HeightMap {
    fn get(&self, location: &Vector2i) -> usize {
        if location.x < 0 || location.x >= self.size || location.y < 0 || location.y >= self.size {
            return usize::MAX;
        }
        self.heights[location.x as usize][location.y as usize]
    }

    fn get_trailheads(&self) -> Vec<Vector2i> {
        self.heights
            .iter()
            .enumerate()
            .flat_map(|(x, ys)| ys.iter().enumerate().map(move |(y, h)| (x, y, h)))
            .filter(|(_, _, h)| **h == 0)
            .map(|(x, y, _)| Vector2i::new(x as i64, y as i64))
            .collect()
    }
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let map = parse_input(input);

    let total_score = map_score(&map);
    println!("Total score: {total_score}");

    let total_rating = map_rating(&map);
    println!("Total rating: {total_rating}");
}

fn map_score(map: &HeightMap) -> usize {
    map.get_trailheads()
        .iter()
        .map(|trailhead| trailhead_score(*trailhead, map))
        .sum()
}

fn map_rating(map: &HeightMap) -> usize {
    map.get_trailheads()
        .iter()
        .map(|trailhead| trailhead_rating(*trailhead, map))
        .sum()
}

fn trailhead_score(location: Vector2i, map: &HeightMap) -> usize {
    let mut to_explore: HashSet<Vector2i> = HashSet::new();
    to_explore.insert(location);
    let mut explored: HashSet<Vector2i> = HashSet::new();
    let mut n_reachable = 0;

    while !to_explore.is_empty() {
        let explore_now = to_explore
            .take(&to_explore.iter().next().unwrap().clone())
            .unwrap();
        explored.insert(explore_now);
        let height = map.get(&explore_now);

        if height == 9 {
            n_reachable += 1;
        }

        for direction in DIRECTION_VECTORS {
            let new_position = explore_now + direction;
            let new_height = map.get(&new_position);
            if new_height == height + 1 && !explored.contains(&new_position) {
                to_explore.insert(new_position);
            }
        }
    }

    n_reachable
}

fn trailhead_rating(location: Vector2i, map: &HeightMap) -> usize {
    let mut to_explore: HashSet<Vec<Vector2i>> = HashSet::new();
    to_explore.insert(vec![location]);
    let mut explored: HashSet<Vec<Vector2i>> = HashSet::new();
    let mut n_reachable = 0;

    while !to_explore.is_empty() {
        let explore_now = to_explore
            .take(&to_explore.iter().next().unwrap().clone())
            .unwrap();
        explored.insert(explore_now.clone());
        let height = map.get(explore_now.last().unwrap());

        if height == 9 {
            n_reachable += 1;
        }

        for direction in DIRECTION_VECTORS {
            let new_position = *explore_now.last().unwrap() + direction;
            let new_height = map.get(&new_position);
            if new_height != height + 1 {
                continue;
            }
            let mut new_route = explore_now.clone();
            new_route.push(new_position);
            to_explore.insert(new_route);
        }
    }
    n_reachable
}

fn parse_input(input: &str) -> HeightMap {
    let size = input.lines().count() as i64;
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|h| h.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    HeightMap { size, heights }
}

const DIRECTION_VECTORS: [Vector2i; 4] = [
    Vector2i { x: 1, y: 0 },
    Vector2i { x: -1, y: 0 },
    Vector2i { x: 0, y: 1 },
    Vector2i { x: 0, y: -1 },
];

const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
