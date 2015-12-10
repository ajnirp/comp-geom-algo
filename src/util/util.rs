use primitives::point::Point;

pub fn det(p: &Point, q: &Point, r: &Point) -> f64 {
    return (q.x*r.y - q.y*r.x) -
           (p.x*r.y - p.y*r.x) +
           (p.x*q.y - p.y*q.x);
}

pub fn angle(p: &Point, q: &Point, r: &Point) -> f64 {
    0f64 // TODO
}