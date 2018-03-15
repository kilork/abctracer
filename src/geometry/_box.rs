use super::*;

pub struct Box {
    material: Surface,
    loc: Vector,
    center: Vector,
    n: [Vector; 3],
    d1: [f64; 3],
    d2: [f64; 3],
    e1: Vector,
    e2: Vector,
    e3: Vector,
}

impl Box {
    pub fn new(material: Surface, loc: Vector, e1: Vector, e2: Vector, e3: Vector) -> Box {
        let center = loc + (e1 + e2 + e3) * 0.5;
        Box::init_normals(&material, &loc, &e1, &e2, &e3, &center)
    }

    fn init_normals(
        &material: &Surface,
        &loc: &Vector,
        &e1: &Vector,
        &e2: &Vector,
        &e3: &Vector,
        &center: &Vector,
    ) -> Box {
        let mut n = [
            (e1 ^ e2).normalize(),
            (e1 ^ e3).normalize(),
            (e2 ^ e3).normalize(),
        ];
        let mut d1 = [-(n[0] & loc), -(n[1] & loc), -(n[2] & loc)];
        let mut d2 = [
            -(n[0] & (loc + e3)),
            -(n[1] & (loc + e2)),
            -(n[2] & (loc + e1)),
        ];
        for i in 0..3 {
            if d1[i] > d2[i] {
                d1[i] = -d1[i];
                d2[i] = -d2[i];
                n[i] = -n[i];
            }
        }
        Box {
            material,
            loc,
            center,
            n,
            d1,
            d2,
            e1,
            e2,
            e3,
        }
    }
}

impl GObject for Box {
    fn material(&self) -> &Surface {
        &self.material
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        let mut t_near = -INFINITY;
        let mut t_far = INFINITY;

        for i in 0..3 {
            let vd = ray.dir & self.n[i];
            let vo = ray.org & self.n[i];

            let (t1, t2) = if vd > EPS {
                (-(vo + self.d2[i]) / vd, -(vo + self.d1[i]) / vd)
            } else if vd < -EPS {
                (-(vo + self.d1[i]) / vd, -(vo + self.d2[i]) / vd)
            } else {
                if vo < self.d1[i] || vo > self.d2[i] {
                    return false;
                } else {
                    continue;
                }
            };

            if t1 > t_near {
                t_near = t1;
            }

            if t2 < t_far {
                t_far = if t2 < GEOMETRY_THRESHOLD {
                    return false;
                } else {
                    t2
                };
            }

            if t_near > t_far {
                return false;
            }
        }

        *t = t_near;

        *t > GEOMETRY_THRESHOLD
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        let mut min_dist = INFINITY;
        let mut index = 0;

        for i in 0..3 {
            let d = *p & self.n[i];
            let dist = (d + self.d1[i]).abs().min((d + self.d2[i]).abs());
            if dist < min_dist {
                min_dist = dist;
                index = i;
            }
        }
        let normal = self.n[index];
        if (*p - self.center) & normal < 0.0 {
            return -normal
        }
        normal
    }
}
