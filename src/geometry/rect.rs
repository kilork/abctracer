use super::*;

pub struct Rect {
    material: Surface,
    loc: Vector,
    side_a: Vector,
    side_b: Vector,
    normal: Vector,
    ku: Vector,
    kv: Vector,
    u0: f64,
    v0: f64,
}

impl Rect {
    pub fn new(material: Surface, loc: Vector, side_a: Vector, side_b: Vector) -> Rect {
        let s_aa = side_a & side_a;
        let s_ab = side_a & side_b;
        let s_bb = side_b & side_b;
        let d = s_aa * s_bb - s_ab * s_ab; // determinant
        let ku = (side_a * s_bb - side_b * s_ab) / d;
        let kv = (side_b * s_aa - side_a * s_ab) / d;
        Rect {
            material,
            loc,
            side_a,
            side_b,
            normal: (side_a ^ side_b).normalize(),
            ku,
            kv,
            u0: -(loc & ku),
            v0: -(loc & kv),
        }
    }

    pub fn intersect_uv(&self, ray: &Ray, t: &mut f64) -> Option<(f64, f64)> {
        let vd = self.normal & ray.dir;
        if vd > -EPS && vd < EPS {
            return None;
        }

        *t = ((self.loc - ray.org) & self.normal) / vd;
        if *t < GEOMETRY_THRESHOLD {
            return None;
        }

        let p = ray.point(*t);
        let u = self.u0 + (p & self.ku);
        let v = self.v0 + (p & self.kv);

        Some((u, v))
    }
}

impl GObject for Rect {
    fn material(&self) -> &Surface {
        &self.material
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        match self.intersect_uv(ray, t) {
            Some((u, v)) => u > 0.0 && v > 0.0 && u < 1.0 && v < 1.0,
            None => false,
        }
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        self.normal
    }
}
