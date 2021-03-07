use bit_vec::BitVec;
use visioncortex::PointF64;

pub(crate) const EPSILON: f64 = std::f64::EPSILON;

pub(crate) fn f64_approximately(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}

pub(crate) fn normalize_point_f64(p: &PointF64) -> PointF64 {
    let norm = p.norm();
    PointF64::new(p.x / norm, p.y / norm)
}

pub(crate) fn normalize_vec_f64(v: &[f64]) -> Vec<f64> {
    let norm = v.iter().fold(0.0, |acc, element| acc + element * element).sqrt();
    v.iter().map(|element| element / norm).collect()
}

pub(crate) fn euclid_dist_f64(p1: &PointF64, p2: &PointF64) -> f64 {
    (*p1-*p2).norm()
}

pub(crate) fn euclid_dist_vec_f64(v1: &[f64], v2: &[f64]) -> f64 {
    if v1.len() != v2.len() {
        panic!("Lengths of vectors do not agree.");
    }
    v1.iter().enumerate()
        .fold(0.0, |acc, (i, element)| acc + (element - v2[i])*(element - v2[i]))
        .sqrt()
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

/// Returns the minimum number of bits needed to store n elements
pub(crate) fn num_bits_to_store(n: usize) -> usize {
    // Special cases
    if n == 0 {
        return 0; // 0 bits are needed to store 0 elements
    } else if n == 1 {
        return 1; // 1 bit is needed to store 1 element
    }
    let n = n-1; // Given 32 elements, we can label them from 0-31
    num_significant_bits(n)
}

pub(crate) fn num_significant_bits(n: usize) -> usize {
    (0_usize.leading_zeros() - n.leading_zeros()) as usize
}

/// Converts a usize into BitVec using the specified number of bits
pub(crate) fn into_bitvec(mut n: usize, len: usize) -> BitVec {
    if len < num_significant_bits(n) {
        panic!(format!("Not enough bits to store {}", n));
    }
    let mut bitvec = BitVec::from_elem(len, false);
    for i in (0..len).rev() {
        bitvec.set(i, n % 2 == 1);
        n >>= 1;
    }
    bitvec 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn math_euclid_dist_unit() {
        let p1 = PointF64::new(1.0, 0.0);
        let p2 = PointF64::new(0.0, 1.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 2.0_f64.sqrt()))
    }

    #[test]
    fn math_euclid_dist_zero() {
        let p1 = PointF64::new(0.0, 0.0);
        let p2 = PointF64::new(0.0, 0.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 0.0))
    }

    #[test]
    fn math_euclid_dist_regular() {
        let p1 = PointF64::new(3.0, 4.0);
        let p2 = PointF64::new(1.0, 5.0);
        let dist = euclid_dist_f64(&p1, &p2);
        assert!(f64_approximately(dist, 5.0_f64.sqrt()))
    }

    #[test]
    fn math_clockwise_points() {
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

    #[test]
    fn math_num_bits_to_store() {
        assert_eq!(num_bits_to_store(0), 0); // 0 bits are needed to store 0 elements
        assert_eq!(num_bits_to_store(1), 1); // 1 bit is needed to store 1 element
        assert_eq!(num_bits_to_store(2), 1); // 0,1 are the two elements
        assert_eq!(num_bits_to_store(3), 2); // 00,01,10 are the three elements
        assert_eq!(num_bits_to_store(32), 5); // 00000 -> 11111 (31)
        assert_eq!(num_bits_to_store(33), 6); // 000000 -> 100000 (32)
    }

    #[test]
    fn math_euclid_dist_vec() {
        let v1 = vec![1.0, 1.0, 1.0];
        let v2 = vec![1.0, 1.0, 1.0];
        assert!(f64_approximately(euclid_dist_vec_f64(&v1, &v2), 0.0));
        let v1 = vec![1.0, 0.5, 1.0];
        let v2 = vec![2.0, 1.0, 3.0];
        println!("{}", euclid_dist_vec_f64(&v1, &v2));
        assert!(f64_approximately(euclid_dist_vec_f64(&v1, &v2), 2.29128784747792));
    }

    #[test]
    fn math_into_bitvec() {
        let n = 0;
        assert!(into_bitvec(n, 1).eq_vec(&[false]));
        let n = 1;
        assert!(into_bitvec(n, 1).eq_vec(&[true]));
        let n = 9;
        assert!(into_bitvec(n, 4).eq_vec(&[true, false, false, true]));
        let n = 14;
        assert!(into_bitvec(n, 4).eq_vec(&[true, true, true, false])); 
        let n = 20;
        assert!(into_bitvec(n, 5).eq_vec(&[true, false, true, false, false])); 
    }
}