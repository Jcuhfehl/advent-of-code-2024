use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64,
}

#[test]
fn test() {
    assert_eq!(
        Vector2i::new(3, 2) + Vector2i::new(2, 1),
        Vector2i::new(5, 3)
    );
    assert_eq!(
        Vector2i::new(3, 2) * Vector2i::new(2, 2),
        Vector2i::new(6, 4)
    );
    assert_eq!(Vector2i::new(3, 2) * 3, Vector2i::new(9, 6));
    assert_eq!(
        Vector2i::new(3, 2) - Vector2i::new(2, 1),
        Vector2i::new(1, 1)
    );
}

pub fn dijkstra<T>(
    graph: HashMap<T, Vec<(usize, T)>>,
    starting_position: T,
) -> HashMap<T, (usize, Vec<T>)>
where
    T: Eq + std::fmt::Debug,
    T: Hash,
    T: Copy,
{
    let mut unvisited: HashSet<T> = graph.keys().cloned().collect();
    let mut distances: HashMap<T, (usize, Vec<T>)> = HashMap::from_iter(
        unvisited
            .iter()
            .cloned()
            .map(|position| (position, (usize::MAX, Vec::new()))),
    );
    *distances.get_mut(&starting_position).unwrap() = (0, Vec::new());

    while !unvisited.is_empty() {
        let evaluate_position = unvisited
            .iter()
            .min_by_key(|position| distances[position].0)
            .unwrap();
        let evaluate_distance = distances[evaluate_position].0;
        if evaluate_distance == usize::MAX {
            break;
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
    distances
}

impl Vector2i {
    pub fn new(x: i64, y: i64) -> Vector2i {
        Vector2i { x, y }
    }

    pub fn modulo(self: Vector2i, rhs: Vector2i) -> Vector2i {
        let mut x = self.x % rhs.x;
        let mut y = self.y % rhs.y;
        if x < 0 {
            x = rhs.x + x;
        }
        if y < 0 {
            y = rhs.y + y;
        }

        Vector2i { x, y }
    }

    pub const DIRECTION_VECTORS: [Vector2i; 4] = [
        Vector2i { x: 1, y: 0 },
        Vector2i { x: -1, y: 0 },
        Vector2i { x: 0, y: 1 },
        Vector2i { x: 0, y: -1 },
    ];

    pub fn rotate_right(self: Vector2i) -> Vector2i {
        Vector2i {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn rotate_left(self: Vector2i) -> Vector2i {
        Vector2i {
            x: self.y,
            y: -self.x,
        }
    }
}

impl std::ops::Add for Vector2i {
    type Output = Vector2i;
    fn add(self, other: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Mul for Vector2i {
    type Output = Vector2i;
    fn mul(self, other: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::Mul<i64> for Vector2i {
    type Output = Vector2i;
    fn mul(self, rhs: i64) -> Self::Output {
        Vector2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Neg for Vector2i {
    type Output = Vector2i;
    fn neg(self) -> Self::Output {
        Vector2i {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Sub for Vector2i {
    type Output = Vector2i;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub default: T,
    pub size: Vector2i,
    pub values: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn get(&self, location: &Vector2i) -> &T {
        if location.x < 0
            || location.x >= self.size.x
            || location.y < 0
            || location.y >= self.size.y
        {
            return &self.default;
        }
        &self.values[location.x as usize][location.y as usize]
    }

    pub fn get_mut(&mut self, location: &Vector2i) -> &mut T {
        if location.x < 0
            || location.x >= self.size.x
            || location.y < 0
            || location.y >= self.size.y
        {
            return &mut self.default;
        }
        &mut self.values[location.x as usize][location.y as usize]
    }

    pub fn set(&mut self, location: &Vector2i, value: T) -> bool {
        if location.x < 0
            || location.x >= self.size.x
            || location.y < 0
            || location.y >= self.size.y
        {
            return false;
        }
        self.values[location.x as usize][location.y as usize] = value;
        true
    }

    pub fn coordinates(&self) -> Vec<Vector2i> {
        (0..self.size.y)
            .flat_map(|y| {
                (0..self.size.x).map(move |x| Vector2i {
                    x: x as i64,
                    y: y as i64,
                })
            })
            .collect()
    }
}

impl<T: Clone> Grid<T> {
    pub fn empty(size: Vector2i, default: T) -> Grid<T> {
        let values = (0..size.x)
            .map(|_| (0..size.y).map(|_| default.clone()).collect())
            .collect();

        Grid::<T> {
            default,
            size,
            values,
        }
    }
}
