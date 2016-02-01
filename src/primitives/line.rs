use ::point::Point;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        Line {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn len(&self) -> f64 {
        self.a.dist(&self.b)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        let da = self.a.clone() - other.a.clone();
        let db = self.b.clone() - other.b.clone();
        da.dot(&db)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}<->{}", self.a, self.b)
    }
}

impl Eq for Line {}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) ||
        (self.a == other.b && self.b == other.a)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line::new(&self.a, &self.b)
    }

    fn clone_from(&mut self, other: &Self) {
        self.a = other.a.clone();
        self.b = other.b.clone();
    }
}
