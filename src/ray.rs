use super::*;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub org: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(org: Vector, dir: Vector) -> Ray {
        Ray { org, dir }
    }

    pub fn point(&self, t: f64) -> Vector {
        self.org + self.dir * t
    }
}

impl From<((f64, f64, f64), (f64, f64, f64))> for Ray {
    fn from(v: ((f64, f64, f64), (f64, f64, f64))) -> Ray {
        Ray::new(Vector::from(v.0), Vector::from(v.1))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ray_point() {
        assert_eq!(
            Ray::from(((1.0, 2.0, 3.0), (1.0, 1.0, 1.0))).point(5.0),
            Vector {
                x: 1.0 + 5.0,
                y: 2.0 + 5.0,
                z: 3.0 + 5.0,
            }
        );
    }

    #[test]
    fn ray_from_tuple() {
        assert_eq!(
            Ray::from(((1.0, 2.0, 3.0), (4.0, 5.0, 6.0))),
            Ray::new(
                Vector {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                },
                Vector {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0,
                }
            )
        )
    }
}
