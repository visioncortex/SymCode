use visioncortex::PointF64;

pub(crate) const EPSILON: f64 = 1e-5;

pub(crate) fn normalize_point_f64(p: &PointF64) -> PointF64 {
    let norm = p.norm();
    PointF64::new(p.x / norm, p.y / norm)
}

pub(crate) fn euclid_dist_f64(p1: &PointF64, p2: &PointF64) -> f64 {
    (*p1-*p2).norm()
}

/// Returns true iff the traversal p1->p2->p3 is in clockwise order and not collinear.
///
/// Assumes origin in top-left corner.
pub(crate) fn clockwise_points_f64(p1: &PointF64, p2: &PointF64, p3: &PointF64) -> bool {
    let cross_product_z_component = |a: PointF64, b: PointF64| { a.x * b.y - a.y * b.x };

    let p1p2 = *p2 - *p1;
    let p1p3 = *p3 - *p1;

    // (p1p2 x p1p3).z > 0 iff the traversal p1->p2->p3 is in clockwise order
    let cross_z = cross_product_z_component(p1p2, p1p3);

    cross_z > EPSILON && cross_z.is_sign_positive()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn f64_approximately(a: f64, b: f64) -> bool {
        (a - b).abs() <= EPSILON
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

    #[test]
    fn test_clockwise_points() {
        // Clockwise
        assert!(clockwise_points_f64(&PointF64::new(0.0, 0.0), &PointF64::new(1.0, 0.0), &PointF64::new(0.0, 1.0)));
        // Anti-Clockwise
        assert!(!clockwise_points_f64(&PointF64::new(0.0, 0.0), &PointF64::new(0.0, 1.0), &PointF64::new(1.0, 0.0)));
        // Collinear
        assert!(!clockwise_points_f64(&PointF64::new(0.0, 0.0), &PointF64::new(1.0, 1.0), &PointF64::new(2.0, 2.0)));
        // Same point
        assert!(!clockwise_points_f64(&PointF64::new(1.0, 1.0), &PointF64::new(1.0, 1.0), &PointF64::new(1.0, 1.0)));
        // Random clockwise
        assert!(clockwise_points_f64(&PointF64::new(7.3, 5.2), &PointF64::new(10.5, 2.5), &PointF64::new(15.6, 4.2)));
        // Random anti-clockwise
        assert!(!clockwise_points_f64(&PointF64::new(23.3, 6.8), &PointF64::new(30.1, 14.7), &PointF64::new(27.5, 11.4)));
    }
}