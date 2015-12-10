// Jarvis march
// see page 17 of http://www.cs.umd.edu/~mount/754/Lects/754lects.pdf

use primitives::point::Point;
use util::util::angle;

// obtain the two points used to kickstart the march
fn get_start_points(v: &Vec<Point>) -> (Point, Point) {
    let mut min_x = v[0].x;
    let mut min_y = v[0].y;
    let mut min_y_idx = 0;
    for (i, p) in v.iter().enumerate() {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
            min_y_idx = i;
        }
    }
    let fst = Point::new(min_x - 1f64, min_y - 1f64);
    let snd = v[min_y_idx].clone();

    (fst, snd)
}

fn cvx_hull(v: &Vec<Point>) -> Vec<Point> {
    if v.len() < 3 {
        return v.clone();
    }

    let res = Vec::<Point>::new();
    
}