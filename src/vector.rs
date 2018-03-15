use std::ops::{Add, AddAssign, BitAnd, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub,
               SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn from_scalar(v: f64) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    pub fn from_xyz(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn assign_from_scalar(&mut self, f: f64) {
        self.x = f;
        self.y = f;
        self.z = f;
    }

    pub fn gt(&self, other: f64) -> bool {
        self.x > other && self.y > other && self.z > other
    }

    pub fn lt(&self, other: f64) -> bool {
        self.x < other && self.y < other && self.z < other
    }

    pub fn normalize(&self) -> Vector {
        *self / !*self
    }

    pub fn clip(self) -> Vector {
        Vector {
            x: self.x.min(1.0).max(0.0),
            y: self.y.min(1.0).max(0.0),
            z: self.z.min(1.0).max(0.0),
        }
    }
}

impl From<f64> for Vector {
    fn from(v: f64) -> Vector {
        Vector::from_scalar(v)
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(v: (f64, f64, f64)) -> Vector {
        Vector::from_xyz(v.0, v.1, v.2)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        *self = Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        *self = Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl BitAnd for Vector {
    type Output = f64;

    fn bitand(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl BitXor for Vector {
    type Output = Vector;

    fn bitxor(self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Not for Vector {
    type Output = f64;

    fn not(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn vector_is_f64() {
        format!(
            "{:?}",
            Vector {
                x: 1.,
                y: 1.,
                z: 1.,
            }
        );
    }

    #[test]
    fn vector_from_scalar() {
        assert_eq!(
            Vector::from_scalar(123.12),
            Vector {
                x: 123.12,
                y: 123.12,
                z: 123.12,
            }
        );
    }

    #[test]
    fn vector_assign_from_scalar() {
        let mut v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v.assign_from_scalar(1.23);
        assert_eq!(
            v,
            Vector {
                x: 1.23,
                y: 1.23,
                z: 1.23,
            }
        );
    }

    #[test]
    fn vector_neg() {
        assert_eq!(
            -Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vector {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn vector_add() {
        assert_eq!(
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            } + Vector {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
            Vector {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            },
        );
    }

    #[test]
    fn vector_sub() {
        assert_eq!(
            Vector {
                x: 1.1,
                y: 2.2,
                z: 3.3,
            } - Vector {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
            Vector {
                x: -2.9,
                y: -2.8,
                z: -2.7,
            },
        );
    }

    #[test]
    fn vector_mul_f64() {
        assert_eq!(
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            } * 3.0,
            Vector {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            },
        );
        assert_eq!(
            3.0 * Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vector {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            },
        );
    }

    #[test]
    fn vector_div_f64() {
        assert_eq!(
            Vector {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            } / 3.0,
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        );
    }

    #[test]
    fn vector_div() {
        assert_eq!(
            Vector {
                x: 3.0,
                y: 6.0,
                z: 10.0,
            } / Vector {
                x: 1.5,
                y: 2.0,
                z: 2.5,
            },
            Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
        );
    }

    #[test]
    fn vector_add_assign() {
        let mut v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v += Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        assert_eq!(
            v,
            Vector {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            },
        );
    }

    #[test]
    fn vector_sub_assign() {
        let mut v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v -= Vector {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        assert_eq!(
            v,
            Vector {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            },
        );
    }

    #[test]
    fn vector_mul_assign() {
        let mut v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v *= Vector {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        assert_eq!(
            v,
            Vector {
                x: 2.0,
                y: 8.0,
                z: 18.0,
            },
        );
    }

    #[test]
    fn vector_mul_assign_f64() {
        let mut v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        v *= 3.0;
        assert_eq!(
            v,
            Vector {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            },
        );
    }

    #[test]
    fn vector_div_assign_f64() {
        let mut v = Vector {
            x: 3.0,
            y: 6.0,
            z: 9.0,
        };
        v /= 3.0;
        assert_eq!(
            v,
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        );
    }

    #[test]
    fn vector_bitand() {
        assert_eq!(
            Vector {
                x: 1.5,
                y: 2.0,
                z: 3.0,
            } & Vector {
                x: 1.0,
                y: 2.0,
                z: -3.0,
            },
            1.5 * 1.0 + 2.0 * 2.0 - 3.0 * 3.0
        );
    }

    #[test]
    fn vector_bitxor() {
        assert_eq!(
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            } ^ Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
    }

    #[test]
    fn vector_not() {
        assert_eq!(
            !Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            14.0_f64.sqrt()
        );
    }

    #[test]
    fn vector_compare() {
        assert!(
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }.gt(0.5)
        );
        assert!(
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }.lt(3.5)
        );
    }

    #[test]
    fn vector_normalize() {
        assert_eq!(
            Vector {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            }.normalize(),
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }

    #[test]
    fn vector_clip() {
        let mut v = Vector {
            x: 3.0,
            y: 2.0,
            z: 0.5,
        };
        assert_eq!(
            v.clip(),
            Vector {
                x: 1.0,
                y: 1.0,
                z: 0.5,
            }
        );
        assert_eq!(
            Vector {
                x: -0.5,
                y: 2.0,
                z: 0.5,
            }.clip(),
            Vector {
                x: 0.0,
                y: 1.0,
                z: 0.5,
            }
        );
    }
}
