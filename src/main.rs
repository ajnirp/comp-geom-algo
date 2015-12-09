extern crate algo;
extern crate primitives;

fn main() {
    
}

#[cfg(test)]
mod test {
    use primitives::point::Point;
    use algo::graham;

    #[test]
    fn test_det() {
        let val = graham::det(&Point::new(4f64, 0f64),
                              &Point::new(2f64, 6f64),
                              &Point::new(1f64, 3f64));
        assert_eq!(val, 12f64);
    }

    #[test]
    fn test_cvx_hull() {
        let v = vec![
            Point::new(1f64, 3f64),
            Point::new(2f64, 6f64),
            Point::new(3f64, 2f64),
            Point::new(4f64, 0f64),
            Point::new(5f64, 8f64),
            Point::new(6f64, 4f64),
            Point::new(7f64, 7f64),
            Point::new(8f64, 1f64),
            Point::new(9f64, 5f64),
        ];
        let hull = graham::cvx_hull(&v);
        assert_eq!(hull, vec![
                Point::new(1f64, 3f64),
                Point::new(2f64, 6f64),
                Point::new(5f64, 8f64),
                Point::new(7f64, 7f64),
                Point::new(9f64, 5f64),
                Point::new(8f64, 1f64),
                Point::new(4f64, 0f64),
            ]);
    }
}
