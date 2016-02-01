use primitives::line::Line;
use primitives::point::Point;
use std::f64;

fn trunc(x: &mut f64) {
    if *x > 1f64 && *x - 1f64 < 1e-5 {
        *x = 1f64;
    } else if *x < -1f64 && -1f64 - *x < 1e-5 {
        *x = -1f64;
    }
}

pub fn det(p: &Point, q: &Point, r: &Point) -> f64 {
    (q.x*r.y - q.y*r.x) -
    (p.x*r.y - p.y*r.x) +
    (p.x*q.y - p.y*q.x)
}

pub fn angle(p: &Point, q: &Point, r: &Point) -> f64 {
    let orient = det(p, q, r);

    let pq = Line::new(p, q);
    let rq = Line::new(r, q);

    let mut sin_th = orient / (pq.len() * rq.len());
    let mut cos_th = pq.dot(&rq) / (pq.len() * rq.len());

    let pi = f64::consts::PI;

    trunc(&mut sin_th);
    trunc(&mut cos_th);

    let asin = sin_th.asin();
    let acos = cos_th.asin();

    if sin_th >= 0f64 && cos_th >= 0f64 {
        asin
    } else if sin_th >= 0f64 && cos_th < 0f64 {
        acos
    } else if sin_th < 0f64 && cos_th >= 0f64 {
        (2f64 * pi) + asin
    }  else if sin_th < 0f64 && cos_th < 0f64 {
        (1.5f64 * pi) + asin
    } else {
        panic!("shouldn't reach here");
    }
}

#[cfg(test)]
mod test {
    use primitives::point::Point;
    use std::f64;
    use super::angle;

    fn check_angle(p: &Point, q: &Point, r: &Point, expected: f64) {
        let angle = angle(&p, &q, &r);
        assert!(angle - expected < 1e-5);
    }

    #[test]
    fn test_angle() {
        let pi = f64::consts::PI;
        let origin = Point::new(0f64, 0f64);
        let pt_1_0 = Point::new(1f64, 0f64);

        check_angle(&Point::new(0f64, 1f64),
                    &origin,
                    &pt_1_0,
                    pi / 2f64);

        check_angle(&Point::new(4f64, 4f64),
                    &origin,
                    &pt_1_0,
                    pi * 0.25f64);

        check_angle(&Point::new(-4f64, 4f64),
                    &origin,
                    &pt_1_0,
                    pi * 0.75f64);

        check_angle(&pt_1_0,
                    &origin,
                    &pt_1_0,
                    0f64);

        check_angle(&pt_1_0,
                    &origin,
                    &Point::new(3f64.sqrt(), -1f64),
                    11f64 * pi / 6f64);

        check_angle(&Point::new(-6f64, 6f64),
                    &origin,
                    &Point::new(1.3f64, 1.3f64),
                    pi / 2f64);
    }
}