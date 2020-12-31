pub(crate) struct Numeric {}

pub(crate) type Matrix = Vec<Vec<f64>>;

impl Numeric {
    // adapted from numeric.js

    pub(crate) fn dim(x: &Matrix) -> Vec<usize> {
        return vec![x.len(), x[0].len()];
    }

    pub(crate) fn clone(x: &Matrix) -> Matrix {
        let mut yy = Vec::new();
        for xx in x.iter() {
            yy.push(xx.clone());
        }
        return yy;
    }

    pub(crate) fn identity(n: usize) -> Matrix {
        let mut x = Vec::new();
        for i in 0..n {
            let mut yy = Vec::new();
            for j in 0..n {
                yy.push(if i == j {1.0} else {0.0});
            }
            x.push(yy);
        }
        return x;
    }

    pub(crate) fn transpose(x: &Matrix) -> Matrix {
        let mut yy = Vec::new();
        for _j in 0..x[0].len() {
            yy.push(Vec::new());
        }
        for j in 0..x[0].len() {
            for i in 0..x.len() {
                yy[j].push(x[i][j]);
            }
        }
        return yy;
    }

    pub(crate) fn inv(a: &Matrix) -> Option<Matrix> {
        let s = Self::dim(a);
        let m = s[0];
        let n = s[1];
        let mut A = Self::clone(a);
        let mut I = Self::identity(m);
        for j in 0..n {
            let mut i0 = 0;
            let mut v0 = -1.0;
            for i in j..m {
                let k = (A[i][j]).abs();
                if k > v0 {
                    i0 = i;
                    v0 = k;
                }
            }
            A.swap(i0, j);
            I.swap(i0, j);
            let x = A[j][j];
            if x == 0.0 {
                return None;
            }
            for k in j..n {
                A[j][k] /= x; 
            }
            for k in (0..n).rev() {
                I[j][k] /= x;
            }
            for i in (0..m).rev() {
                if i != j {
                    let x = A[i][j];
                    for k in j+1..n {
                        A[i][k] -= A[j][k]*x;
                    }
                    let mut k = n as i32 - 1;
                    while k > 0 {
                        I[i][k as usize] -= I[j][k as usize]*x;
                        k -= 1;
                        I[i][k as usize] -= I[j][k as usize]*x;
                        k -= 1;
                    }
                    if k == 0 {
                        I[i][0] -= I[j][0]*x;
                    }
                }
            }
        }
        return Some(I);
    }

    pub(crate) fn dot_mm_small(x: &Matrix, y: &Matrix) -> Matrix {
        let p = x.len(); let q = y.len(); let r = y[0].len();
        let mut ret = vec![Vec::new(); p];
        for i in (0..p).rev() {
            let mut foo = vec![0.0; r];
            let ref bar = x[i];
            for k in (0..r).rev() {
                let mut woo = bar[q-1]*y[q-1][k];
                let mut j = q as i32 - 2;
                while j >= 1 {
                    let i0 = j-1;
                    woo += bar[j as usize]*y[j as usize][k] + bar[i0 as usize]*y[i0 as usize][k];
                    j -= 2;
                }
                if j == 0 {
                    woo += bar[0]*y[0][k];
                }
                foo[k] = woo;
            }
            ret[i] = foo;
        }
        return ret;
    }

    pub(crate) fn dot_mv(x: &Matrix, y: &Vec<f64>) -> Vec<f64> {
        let p = x.len();
        let mut ret = vec![0.0; p];
        for i in (0..p).rev() {
            ret[i] = Self::dot_vv(&x[i], y);
        }
        return ret;
    }

    pub(crate) fn dot_vv(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let n = x.len();
        let mut ret = x[n-1]*y[n-1];
        let mut i = n as i32 - 2;
        while i >= 1 {
            let i1 = i-1;
            ret += x[i as usize]*y[i as usize] + x[i1 as usize]*y[i1 as usize];
            i -= 2;
        }
        if i == 0 {
            ret += x[0]*y[0];
        }
        return ret;
    }
}