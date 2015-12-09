use std::clone::Clone;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result};

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

#[cfg(test)]
mod test {
    use super::Point;
    #[test]
    fn test_eq() {
        assert_eq!(Point::new(2f64, -9.3f64), Point::new(2f64, -9.3f64));
    }

    #[test]
    fn test_lt() {
        assert!(Point::new(2f64, 9.3f64) < Point::new(3f64, -9.3f64));   
    }
}
