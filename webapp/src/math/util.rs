use visioncortex::PointF64;

pub(crate) fn euclid_dist_f64(p1: &PointF64, p2: &PointF64) -> f64 {
    (*p1-*p2).norm()
}

/// Returns true iff the traversal p1->p2->p3 is in clockwise order
pub(crate) fn clockwise_points_f64(p1: &PointF64, p2: &PointF64, p3: &PointF64) -> bool {
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    fn f64_approximately(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-5
    }

    #[test]
    fn test_euclid_dist_unit() {
        let p1 = PointF64::new(1.0, 0.0);
        let p2 = PointF64::new(0.0, 1.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 2.0_f64.sqrt()))
    }

    #[test]
    fn test_euclid_dist_zero() {
        let p1 = PointF64::new(0.0, 0.0);
        let p2 = PointF64::new(0.0, 0.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 0.0))
    }

    #[test]
    fn test_euclid_dist_regular() {
        let p1 = PointF64::new(3.0, 4.0);
        let p2 = PointF64::new(1.0, 5.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 5.0_f64.sqrt()))
    }
}