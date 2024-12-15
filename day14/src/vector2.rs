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