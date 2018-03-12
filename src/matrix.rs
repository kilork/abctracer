use super::*;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix(pub [[f64; 4]; 4]);

impl Matrix {
    pub fn from_scalar(v: f64) -> Matrix {
        Matrix([
            [v, 0.0, 0.0, 0.0],
            [0.0, v, 0.0, 0.0],
            [0.0, 0.0, v, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn invert(&mut self) {
        let mut out = Matrix::from_scalar(1.0);
        for i in 0..4 {
            let d = self.0[i][i];
            if d != 1.0 {
                for j in 0..4 {
                    out.0[i][j] /= d;
                    self.0[i][j] /= d;
                }
            }
            for j in 0..4 {
                if j != i {
                    let mulby = self.0[j][i];
                    if mulby != 0.0 {
                        for k in 0..4 {
                            self.0[j][k] -= mulby * self.0[i][k];
                            out.0[j][k] -= mulby * out.0[i][k];
                        }
                    }
                }
            }
        }
        *self = out;
    }

    /// Transpose the matrix.
    /// # Examples
    /// ```
    /// use abctracer::Matrix;
    ///
    /// let mut m = Matrix([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0],
    ///     [13.0, 14.0, 15.0, 16.0],
    /// ]);
    /// m.transpose();
    /// assert_eq!(
    ///     m,
    ///     Matrix([
    ///         [1.0, 5.0, 9.0, 13.0],
    ///         [2.0, 6.0, 10.0, 14.0],
    ///         [3.0, 7.0, 11.0, 15.0],
    ///         [4.0, 8.0, 12.0, 16.0],
    ///     ],)
    /// );
    /// ```
    pub fn transpose(&mut self) {
        for i in 0..3 {
            for j in i + 1..4 {
                let t = self.0[i][j];
                self.0[i][j] = self.0[j][i];
                self.0[j][i] = t;
            }
        }
    }

    pub fn translate(v: &Vector) -> Matrix {
        Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [v.x, v.y, v.z, 1.0],
        ])
    }

    pub fn scale(v: &Vector) -> Matrix {
        Matrix([
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_x(angle: f64) -> Matrix {
        let cosine = angle.cos();
        let sine = angle.sin();
        Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cosine, sine, 0.0],
            [0.0, -sine, cosine, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_y(angle: f64) -> Matrix {
        let cosine = angle.cos();
        let sine = angle.sin();
        Matrix([
            [cosine, 0.0, sine, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sine, 0.0, cosine, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_z(angle: f64) -> Matrix {
        let cosine = angle.cos();
        let sine = angle.sin();
        Matrix([
            [cosine, sine, 0.0, 0.0],
            [-sine, cosine, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation(axis: &Vector, angle: f64) -> Matrix {
        let cosine = angle.cos();
        let sine = angle.sin();
        Matrix([
            [
                axis.x * axis.x + (1.0 - axis.x * axis.x) * cosine,
                axis.x * axis.y * (1.0 - cosine) + axis.z * sine,
                axis.x * axis.z * (1.0 - cosine) - axis.y * sine,
                0.0,
            ],
            [
                axis.x * axis.y * (1.0 - cosine) - axis.z * sine,
                axis.y * axis.y + (1.0 - axis.y * axis.y) * cosine,
                axis.y * axis.z * (1.0 - cosine) + axis.x * sine,
                0.0,
            ],
            [
                axis.x * axis.z * (1.0 - cosine) + axis.y * sine,
                axis.y * axis.z * (1.0 - cosine) - axis.x * sine,
                axis.z * axis.z + (1.0 - axis.z * axis.z) * cosine,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn mirror_x() -> Matrix {
        Matrix([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn mirror_y() -> Matrix {
        Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn mirror_z() -> Matrix {
        Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Matrix) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] += other.0[i][j];
            }
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Matrix) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] -= other.0[i][j];
            }
        }
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] *= other;
            }
        }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, other: Matrix) {
        let res = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;

                for k in 0..4 {
                    sum += res.0[i][k] * other.0[k][j];
                }

                self.0[i][j] = sum;
            }
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        let mut res = Matrix([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                res.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        res
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        let mut res = Matrix([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                res.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        res
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut res = Matrix([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;

                for k in 0..4 {
                    sum += self.0[i][k] * other.0[k][j];
                }

                res.0[i][j] = sum;
            }
        }
        res
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Matrix {
        let mut res = Matrix([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                res.0[i][j] = self.0[i][j] * other;
            }
        }
        res
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        let mut res = Vector {
            x: other.x * self.0[0][0] + other.y * self.0[1][0] + other.z * self.0[2][0]
                + self.0[3][0],
            y: other.x * self.0[0][1] + other.y * self.0[1][1] + other.z * self.0[2][1]
                + self.0[3][1],
            z: other.x * self.0[0][2] + other.y * self.0[1][2] + other.z * self.0[2][2]
                + self.0[3][2],
        };
        let denom =
            other.x * self.0[0][3] + other.y * self.0[1][3] + other.z * self.0[2][3] + self.0[3][3];
        if denom != 1.0 {
            res /= denom;
        }
        res
    }
}
