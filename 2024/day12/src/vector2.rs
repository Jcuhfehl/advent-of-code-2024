#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64,
}

impl Vector2i {
    pub const DIRECTION_VECTORS: [Vector2i; 4] = [
        Vector2i { x: 1, y: 0 },
        Vector2i { x: -1, y: 0 },
        Vector2i { x: 0, y: 1 },
        Vector2i { x: 0, y: -1 },
    ];

    pub fn perpendicular(&self) -> Vector2i {
        Vector2i {
            x: self.y,
            y: -self.x,
        }
    }
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
