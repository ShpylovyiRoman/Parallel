use crate::data::{f_2_25, f_3_7, Matrix, Vector};

pub fn f2(n: usize) {
    let p = Vector::new(n, 2);
    let r = Vector::new(n, 2);
    let ms = Matrix::new(n, 2);
    let mt = Matrix::new(n, 2);
    let res = f_3_7(p, r, ms, mt);
    println!("From F2: O = {}", res);
}

pub fn f1(n: usize) {
    let mg = Matrix::new(n, 2);
    let mh = Matrix::new(n, 2);
    let mk = Matrix::new(n, 2);
    let ml = Matrix::new(n, 2);
    let res = f_2_25(mg, mh, mk, ml);
    println!("From F1: MF = {}", res);
}
