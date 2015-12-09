// modified Graham scan algorithm
// see page 13 of http://www.cs.umd.edu/~mount/754/Lects/754lects.pdf

use point::point::Point;

pub fn det(p: &Point, q: &Point, r: &Point) -> f64 {
    return (q.x*r.y - q.y*r.x) -
           (p.x*r.y - p.y*r.x) +
           (p.x*q.y - p.y*q.x);
}

pub fn cvx_hull(v: &Vec<Point>) -> Vec<Point> {
    if v.len() < 3 {
        return v.clone();
    }

    let mut v = v.clone();
    v.sort();

    // upper convex hull
    let mut up = Vec::<Point>::new();
    up.push(v[0].clone()); up.push(v[1].clone());

    for i in 3 .. v.len() {
        while up.len() >= 2 && det(&v[i], &up[up.len()-1], &up[up.len()-2]) <= 0f64 {
            up.pop();
        }
        up.push(v[i].clone());
    }

    // lower convex hull
    let mut lo = Vec::<Point>::new();
    lo.push(v[v.len()-1].clone()); lo.push(v[v.len()-2].clone());

    for i in (0 .. v.len()-3).rev() {
        while lo.len() >= 2 && det(&v[i], &lo[lo.len()-1], &lo[lo.len()-2]) <= 0f64 {
            lo.pop();
        }
        lo.push(v[i].clone());
    }

    // concat lower convex hull to upper convex hull
    for i in 1 .. lo.len()-1 {
        up.push(lo[i].clone());
    }

    up
}
