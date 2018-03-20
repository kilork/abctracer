use super::*;

pub struct Cylinder {
    material: Surface,
    e1: Vector,
    e2: Vector,
    d1: f64,
    d2: f64,
    len: f64,
    len2: f64,
    radius2: f64,
    radius4: f64,
    loc: Vector,
    dir: Vector,
    radius: f64,
}

impl Cylinder {
    pub fn new(material: Surface, loc: Vector, dir: Vector, radius: f64) -> Cylinder {
        let radius2 = radius * radius;
        let len2 = dir & dir;
        let e1 = if dir.x.abs() + dir.y.abs() > dir.z.abs() {
            Vector::from((dir.y, -dir.x, 0.0))
        } else {
            Vector::from((0.0, dir.z, -dir.y))
        };
        let e1 = e1.normalize() * radius;
        Cylinder {
            material,
            loc,
            dir,
            radius,
            radius2,
            radius4: radius2 * radius2,
            d1: -(loc & dir),
            d2: -((loc + dir) & dir),
            len2,
            len: len2.sqrt(),
            e1,
            e2: (dir ^ e1).normalize() * radius,
        }
    }

    fn intersect_bottom_top(&self, ray: &Ray, l1: f64, len: f64, t: &mut f64) {
        if len < 0.0 {
            *t = -((ray.org & self.dir) + self.d1) / l1;
            let p = ray.point(*t) - self.loc;
            if (p & p) >= self.radius2 {
                *t = -1.0;
            }
        } else if len > 1.0 {
            *t = -((ray.org & self.dir) + self.d2) / l1;
            let p = ray.point(*t) - self.loc - self.dir;
            if (p & p) >= self.radius2 {
                *t = -1.0;
            }
        }
    }

    fn intersect_top_bottom(&self, ray: &Ray, l1: f64, len: f64, t: &mut f64) {
        if len < 0.0 {
            *t = -((ray.org & self.dir) + self.d2) / l1;
            let p = ray.point(*t) - self.loc - self.dir;
            if (p & p) >= self.radius2 {
                *t = -1.0;
            }
        } else if len > 1.0 {
            *t = -((ray.org & self.dir) + self.d1) / l1;
            let p = ray.point(*t) - self.loc;
            if (p & p) >= self.radius2 {
                *t = -1.0;
            }
        }
    }
}

impl GObject for Cylinder {
    fn material(&self) -> &Surface {
        &self.material
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        let l = ray.org - self.loc;
        let u0 = l & self.e1;
        let u1 = ray.dir & self.e1;
        let v0 = l & self.e2;
        let v1 = ray.dir & self.e2;
        let l0 = l & self.dir;
        let l1 = ray.dir & self.dir;
        let a = u1 * u1 + v1 * v1;
        let b = u0 * u1 + v0 * v1;
        let c = u0 * u0 + v0 * v0 - self.radius4;
        let d = b * b - a * c;

        if d <= 0.0 {
            return false;
        }

        let d = d.sqrt();

        let mut t1 = (-b - d) / a;
        let mut t2 = (-b + d) / a;

        let len1 = (l0 + t1 * l1) / self.len2;
        let len2 = (l0 + t2 * l1) / self.len2;

        if l1 > EPS {
            self.intersect_bottom_top(ray, l1, len1, &mut t1);
            self.intersect_bottom_top(ray, l1, len2, &mut t2);
        } else if l1 < -EPS {
            self.intersect_top_bottom(ray, l1, len1, &mut t1);
            self.intersect_top_bottom(ray, l1, len2, &mut t2);
        }

        if t1 > GEOMETRY_THRESHOLD {
            *t = t1;
            return true;
        }

        *t = t2;
        t2 > GEOMETRY_THRESHOLD
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        let t = ((*p - self.loc) & self.dir) / self.len2;

        if t < EPS {
            -self.dir / self.len
        } else if t > 1.0 - EPS {
            self.dir / self.len
        } else {
            (*p - self.loc - self.dir * t).normalize()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_intersect_case_1() {
        let cylinder = Cylinder::new(
            Surface::new(),
            Vector::from((0.0, 10.0, 0.0)),
            Vector::from((0.0, 0.0, 1.0)),
            1.0,
        );

        let dir = Vector::from((0.0, 1.0, 0.0)).normalize();
        let ray = Ray::new(Vector::from(0.0), dir);
        let mut dist = 30000.0;
        assert!(cylinder.intersect(&ray, &mut dist));

        let dir = Vector::from((0.0, 1.0, 0.1)).normalize();
        let ray = Ray::new(Vector::from(0.0), dir);
        let mut dist = 30000.0;
        assert!(cylinder.intersect(&ray, &mut dist));

        let dir = Vector::from((0.0, 1.0, -0.1)).normalize();
        let ray = Ray::new(Vector::from(0.0), dir);
        let mut dist = 30000.0;
        assert!(!cylinder.intersect(&ray, &mut dist));
    }
}
