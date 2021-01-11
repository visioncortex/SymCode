use crate::Field;

/// Constructs an `n`-by-`n` identity matrix.
pub fn identity(n: usize) -> Field<f32> {
    assert_ne!(n, 0);

    let mut f = Field::with_initial(n, n, 0.0f32);
    for i in 0..n-1 {
        let index = f.index_at(i, i);
        f.set(index, &1.0);
    }
    f
}

pub fn transpose(matrix: &Field<f32>) -> Field<f32> {
    let mut result = Field::with_default(matrix.height(), matrix.width());
    for (i, x) in matrix.iter().enumerate() {
        let pos = matrix.locate(i);
        let index = result.index_at(pos.1, pos.0);
        result.set(index, x);
    }
    result
}

pub fn dot_mm(lhs: &Field<f32>, rhs: &Field<f32>) -> Field<f32> {
    assert_eq!(lhs.width(), rhs.height());

    let mut result = Field::with_default(lhs.height(), rhs.width());

    for i in 0..(lhs.height() * rhs.width()) {
        let pos = result.locate(i);
        let mut entry = 0.0f32;

        for j in 0..lhs.width() {
            entry += lhs.get(lhs.index_at(j, pos.1)).unwrap() * rhs.get(rhs.index_at(pos.0, j)).unwrap();
        }

        result.set(i, &entry);
    }
    
    result
}

pub fn dot_mv(matrix: &Field<f32>, vector: &[f32]) -> Vec<f32> {
    let cols = std::cmp::min(matrix.width(), vector.len());

    (0..matrix.height())
        .map(|row| (0..cols).
            map(|col| matrix.get(matrix.index_at(row, col)).unwrap() * vector[col]).sum()
        )
        .collect()
}

pub fn dot_vv(row: &[f32], col: &[f32]) -> f32 {
    row
        .iter()
        .zip(col.iter())
        .map(|(a, b)| a*b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f32 = 0.00005f32;

    #[test]
    fn test_mm() {
        let m1 = Field::with_vec(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let m2 = Field::with_vec(2, 3, vec![-1.0, 3.0, -5.0, 7.0, -9.0, 11.0]).unwrap();
        let m3 = dot_mm(&m1, &m2);

        println!("{}", m3.get(0).unwrap());
        println!("{}", m3.get(1).unwrap());
        println!("{}", m3.get(2).unwrap());
        println!("{}", m3.get(3).unwrap());
        assert!((m3.get(0).unwrap() + 38.0).abs() < EPS);
        assert!((m3.get(1).unwrap() - 50.0).abs() < EPS);
        assert!((m3.get(2).unwrap() + 83.0).abs() < EPS);
        assert!((m3.get(3).unwrap() - 113.0).abs() < EPS);
    }
}