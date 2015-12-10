use std::clone::Clone;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x {
            return Some(Ordering::Less);
        }
        else if self.x > other.x {
            return Some(Ordering::Greater);
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.x, self.y)
    }

    fn clone_from(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod test {
    use std::ops::{Add, Sub};
    use super::Point;

    #[test]
    fn test_eq() {
        assert_eq!(Point::new(2f64, -9.3f64), Point::new(2f64, -9.3f64));
    }

    #[test]
    fn test_lt() {
        assert!(Point::new(2f64, 9.3f64) < Point::new(3f64, -9.3f64));
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(14.01f64, -3.2f64);
        let p2 = Point::new(-14.01f64, 5f64);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 0f64);
        assert!(p3.y - 1.8f64 < 1e-5); // floating point errors
    }

    #[test]
    fn test_sub() {
        let p1 = Point::new(3.01f64, 3.2f64);
        let p2 = Point::new(3f64, 5f64);
        let p3 = p1 - p2;
        assert!(p3.x - 0.01f64 < 1e-5); // floating point errors
        assert!(p3.y + 1.8f64 < 1e-5); // floating point errors
    }
}
