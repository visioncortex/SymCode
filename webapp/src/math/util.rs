use visioncortex::PointF64;

pub(crate) fn euclid_dist_f64(p1: &PointF64, p2: &PointF64) -> f64 {
    (*p1-*p2).norm()
}