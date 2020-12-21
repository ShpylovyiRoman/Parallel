use std::fmt;

#[derive(Clone)]
pub struct Matrix {
    size: usize,
    arr: Vec<i32>,
}

#[derive(Clone)]
pub struct Vector {
    arr: Vec<i32>,
}

impl Vector {
    pub fn new(size: usize, elem: i32) -> Self {
        Self {
            arr: vec![elem; size],
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.arr)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.size;
        writeln!(f, "[")?;
        for i in 0..size {
            write!(f, "\t")?;
            for j in 0..size {
                let index = i * size + j;
                write!(f, "{} ", self.arr[index])?;
            }
            write!(f, "\n")?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

impl Matrix {
    pub fn new(size: usize, elem: i32) -> Self {
        let arr = vec![elem; size * size];
        Self { size, arr }
    }

    fn trans(mut self) -> Self {
        let size = self.size;
        for i in 0..size {
            for j in i..size {
                let a = i * size + j;
                let b = j * size + i;
                self.arr.swap(a, b);
            }
        }
        self
    }

    fn add(mut self, rhs: Self) -> Self {
        self.arr.iter_mut().zip(rhs.arr.iter()).for_each(|(a, b)| {
            *a += b;
        });
        self
    }

    fn sub(mut self, rhs: Self) -> Self {
        self.arr.iter_mut().zip(rhs.arr.iter()).for_each(|(a, b)| {
            *a *= b;
        });
        self
    }

    fn mul(self, rhs: Self) -> Self {
        let size = self.size;
        let mut res = Self::new(size, 0);
        for i in 0..size {
            for j in 0..size {
                let entry = &mut res.arr[i * size + j];
                for k in 0..size {
                    let a = self.arr[i * size + k];
                    let b = rhs.arr[k * size + j];
                    *entry += a * b;
                }
            }
        }
        res
    }
}

impl Vector {
    fn add(mut self, rhs: Self) -> Self {
        self.arr.iter_mut().zip(rhs.arr.iter()).for_each(|(a, b)| {
            *a += b;
        });
        self
    }

    fn mul_matrix(self, rhs: Matrix) -> Self {
        let size = self.arr.len();
        let mut res = Self::new(size, 0);
        for i in 0..size {
            for j in 0..size {
                let index = i * size + j;
                res.arr[j] += self.arr[i] * rhs.arr[index];
            }
        }
        res
    }
}

pub fn f_3_7(p: Vector, r: Vector, ms: Matrix, mt: Matrix) -> Vector {
    p.add(r).mul_matrix(ms.mul(mt))
}

pub fn f_2_25(mg: Matrix, mh: Matrix, mk: Matrix, ml: Matrix) -> Matrix {
    mg.add(mh.mul(mk).trans()).sub(ml.trans())
}
