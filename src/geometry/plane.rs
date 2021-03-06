use super::*;

pub struct Plane {
    // Plane Eq. (n,r) + D = 0
    material: Surface,
    /// unit plane normal
    normal: Vector,
    /// distance from origin
    distance: f64,
}

impl Plane {
    pub fn new(material: Surface, normal: Vector, distance: f64) -> Plane {
        Plane {
            material,
            normal,
            distance,
        }
    }

    pub fn from_abcd(material: Surface, a: f64, b: f64, c: f64, d: f64) -> Plane {
        let mut normal = Vector::from((a, b, c));

        let normal_length = !normal;

        normal /= normal_length;

        let distance = d / normal_length;

        Plane {
            material,
            normal,
            distance,
        }
    }
}

impl GObject for Plane {
    fn material(&self) -> &Surface {
        &self.material
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        let vd = self.normal & ray.dir;

        if vd > -EPS && vd < EPS {
            return false;
        }

        *t = -((self.normal & ray.org) + self.distance) / vd;

        *t > GEOMETRY_THRESHOLD
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        self.normal
    }
}
