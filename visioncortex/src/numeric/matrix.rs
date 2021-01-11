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

    for i in 0..(lhs.height() * rhs.height()) {
        let pos = result.locate(i);
        let mut entry = 0.0f32;

        for j in 0..lhs.width() {
            entry += lhs.get(lhs.index_at(pos.0, j)).unwrap() * rhs.get(rhs.index_at(j, pos.1)).unwrap();
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

